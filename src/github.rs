use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::{
    scaffold::find_repo_root,
    status::{StatusFile, TrackedKind, list_tracked_items, resolve_tracked_path, validate_status},
};
use anyhow::{Context, Result, anyhow, bail};
use serde::{Deserialize, Serialize};
use serde_json::{Value as JsonValue, json};

const CONFIG_PATH: &str = "workflow/github.yml";
const AUTO_SYNC_WORKFLOW_PATH: &str = ".github/workflows/mxw-github-sync.yml";
const AUTO_SYNC_MODE: &str = "changed";
const MANAGED_NOTE: &str = "> This issue is managed by `mnemix-workflow`. Update the repo artifact instead of editing GitHub directly.";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct GithubConfig {
    pub(crate) enabled: bool,
    pub(crate) repo: String,
    #[serde(default)]
    pub(crate) auto_sync: AutoSyncConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct AutoSyncConfig {
    #[serde(default)]
    pub(crate) enabled: bool,
    #[serde(default = "default_auto_sync_mode")]
    pub(crate) mode: String,
}

fn default_auto_sync_mode() -> String {
    AUTO_SYNC_MODE.to_owned()
}

impl Default for AutoSyncConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            mode: default_auto_sync_mode(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct IssueLink {
    pub(crate) id: u64,
    pub(crate) number: u64,
    pub(crate) url: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GithubLinkage {
    #[serde(default)]
    pub(crate) issue: Option<IssueLink>,
    #[serde(default)]
    pub(crate) parent_issue: Option<IssueLink>,
    #[serde(default)]
    pub(crate) sub_issues: BTreeMap<String, IssueLink>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum SyncMode {
    Target,
    All,
    Changed,
}

#[derive(Clone, Debug)]
pub(crate) struct SyncRequest {
    pub(crate) target: Option<String>,
    pub(crate) mode: SyncMode,
    pub(crate) status_filter: Option<String>,
    pub(crate) dry_run: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SyncItem {
    kind: TrackedKind,
    path: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
struct IssueResponse {
    id: u64,
    number: u64,
    html_url: String,
}

#[derive(Clone, Debug)]
struct RepoRef {
    owner: String,
    name: String,
}

pub(crate) fn init_support(
    cwd: &Path,
    repo_override: Option<&str>,
    enable_auto_sync: bool,
) -> Result<Vec<String>> {
    let repo_root = find_repo_root(cwd)?;
    let repo = match repo_override {
        Some(repo) => parse_repo_ref(repo)?,
        None => detect_repo_from_git_remote(&repo_root)?,
    };

    let config = GithubConfig {
        enabled: true,
        repo: repo.slug(),
        auto_sync: AutoSyncConfig {
            enabled: enable_auto_sync,
            mode: default_auto_sync_mode(),
        },
    };

    write_config(&repo_root, &config)?;

    let mut lines = vec![
        format!("Initialized GitHub issue support for {}", config.repo),
        format!("Config: {CONFIG_PATH}"),
    ];

    if enable_auto_sync {
        write_auto_sync_workflow(&repo_root)?;
        lines.push(format!("Auto-sync workflow: {AUTO_SYNC_WORKFLOW_PATH}"));
    } else {
        lines.push(
            "Auto-sync is disabled. Re-run with `--enable-auto-sync` when you want the GitHub Action scaffold."
                .to_owned(),
        );
    }

    Ok(lines)
}

pub(crate) fn sync_support(cwd: &Path, request: SyncRequest) -> Result<Vec<String>> {
    if let Some(status) = request.status_filter.as_deref() {
        validate_status(status)?;
    }

    let repo_root = find_repo_root(cwd)?;
    let config = read_config(&repo_root)?;
    let repo = parse_repo_ref(&config.repo)?;
    let items = select_sync_items(&repo_root, &request)?;

    if items.is_empty() {
        return Ok(vec![
            "No tracked items matched the requested GitHub sync scope.".to_owned(),
        ]);
    }

    let mut gh = GhClient::new(&repo_root);
    let mut lines = Vec::new();

    for item in items {
        let status_path = item.kind.status_path(&item.path);
        let mut status = StatusFile::read(&status_path)?;
        if let Some(filter) = request.status_filter.as_deref() {
            if status.status() != filter {
                continue;
            }
        }

        let linkage = linkage_from_status(&status)?;
        let already_linked = match item.kind {
            TrackedKind::Workstream => linkage.parent_issue.is_some(),
            TrackedKind::Patch => linkage.issue.is_some(),
        };

        if matches!(request.mode, SyncMode::Changed) && !already_linked {
            lines.push(format!(
                "Skipped unlinked {} in --changed mode: {}",
                item.kind.singular(),
                item.kind.display_name(&item.path)
            ));
            continue;
        }

        let next_linkage = match item.kind {
            TrackedKind::Workstream => sync_workstream(
                &mut gh,
                &repo,
                &item.path,
                &status,
                linkage.clone(),
                request.dry_run,
            )?,
            TrackedKind::Patch => sync_patch(
                &mut gh,
                &repo,
                &item.path,
                &status,
                linkage.clone(),
                request.dry_run,
            )?,
        };

        if !request.dry_run && next_linkage != linkage {
            set_linkage_on_status(&mut status, &next_linkage)?;
            status.write(&status_path)?;
        }

        match item.kind {
            TrackedKind::Workstream => {
                if request.dry_run {
                    lines.push(format!(
                        "Would sync workstream: {}",
                        item.kind.display_name(&item.path)
                    ));
                } else if let Some(parent) = next_linkage.parent_issue {
                    lines.push(format!(
                        "Synced workstream: {} -> #{}",
                        item.kind.display_name(&item.path),
                        parent.number
                    ));
                }
            }
            TrackedKind::Patch => {
                if request.dry_run {
                    lines.push(format!(
                        "Would sync patch: {}",
                        item.kind.display_name(&item.path)
                    ));
                } else if let Some(issue) = next_linkage.issue {
                    lines.push(format!(
                        "Synced patch: {} -> #{}",
                        item.kind.display_name(&item.path),
                        issue.number
                    ));
                }
            }
        }
    }

    if lines.is_empty() {
        Ok(vec!["No tracked items required GitHub updates.".to_owned()])
    } else {
        Ok(lines)
    }
}

fn select_sync_items(repo_root: &Path, request: &SyncRequest) -> Result<Vec<SyncItem>> {
    let mut items = BTreeSet::new();

    match request.mode {
        SyncMode::Target => {
            let target = request
                .target
                .as_deref()
                .ok_or_else(|| anyhow!("A target is required for targeted GitHub sync."))?;
            let workstream = resolve_tracked_path(repo_root, target, TrackedKind::Workstream);
            let patch = resolve_tracked_path(repo_root, target, TrackedKind::Patch);
            match (workstream, patch) {
                (Ok(workstream), Err(_)) => {
                    items.insert(SyncItem {
                        kind: TrackedKind::Workstream,
                        path: workstream,
                    });
                }
                (Err(_), Ok(patch)) => {
                    items.insert(SyncItem {
                        kind: TrackedKind::Patch,
                        path: patch,
                    });
                }
                (Ok(_), Ok(_)) => {
                    bail!(
                        "Tracked item reference is ambiguous: {target}. Use the full workstream folder or patch file name."
                    );
                }
                (Err(workstream_error), Err(_patch_error)) => return Err(workstream_error),
            }
        }
        SyncMode::All => {
            for path in list_tracked_items(repo_root, TrackedKind::Workstream)? {
                items.insert(SyncItem {
                    kind: TrackedKind::Workstream,
                    path,
                });
            }
            for path in list_tracked_items(repo_root, TrackedKind::Patch)? {
                items.insert(SyncItem {
                    kind: TrackedKind::Patch,
                    path,
                });
            }
        }
        SyncMode::Changed => {
            for item in changed_items(repo_root)? {
                items.insert(item);
            }
        }
    }

    Ok(items.into_iter().collect())
}

fn changed_items(repo_root: &Path) -> Result<Vec<SyncItem>> {
    let changed = git_changed_paths(repo_root)?;
    let mut items = BTreeSet::new();

    for path in changed {
        if let Some(item) = changed_path_to_item(&path) {
            items.insert(item);
        }
    }

    Ok(items.into_iter().collect())
}

fn git_changed_paths(repo_root: &Path) -> Result<Vec<PathBuf>> {
    let event_before = github_event_before(repo_root)?;
    let mut command = Command::new("git");
    command.current_dir(repo_root);

    if let Some(before) = event_before {
        command.args(["diff", "--name-only", &before, "HEAD", "--", "workflow"]);
    } else {
        command.args(["diff", "--name-only", "HEAD~1", "HEAD", "--", "workflow"]);
    }

    let output = command
        .output()
        .with_context(|| format!("Failed to run git diff from {}", repo_root.display()))?;

    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(PathBuf::from)
            .collect());
    }

    let fallback = Command::new("git")
        .args([
            "diff-tree",
            "--no-commit-id",
            "--name-only",
            "-r",
            "HEAD",
            "--",
            "workflow",
        ])
        .current_dir(repo_root)
        .output()
        .with_context(|| format!("Failed to run git diff-tree from {}", repo_root.display()))?;

    if !fallback.status.success() {
        bail!(
            "Failed to determine changed workflow files: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    Ok(String::from_utf8_lossy(&fallback.stdout)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(PathBuf::from)
        .collect())
}

fn github_event_before(repo_root: &Path) -> Result<Option<String>> {
    let event_path = match std::env::var("GITHUB_EVENT_PATH") {
        Ok(path) => PathBuf::from(path),
        Err(_) => return Ok(None),
    };

    if !event_path.exists() {
        return Ok(None);
    }

    let payload = fs::read_to_string(&event_path)
        .with_context(|| format!("Failed to read {}", event_path.display()))?;
    let json: JsonValue =
        serde_json::from_str(&payload).context("Failed to parse GitHub event payload")?;

    let before = json
        .get("before")
        .and_then(|value| value.as_str())
        .map(str::to_owned);

    if before
        .as_deref()
        .is_some_and(|sha| sha.chars().all(|character| character == '0'))
    {
        return Ok(None);
    }

    if before.is_some() {
        return Ok(before);
    }

    let current_head = Command::new("git")
        .args(["rev-parse", "HEAD~1"])
        .current_dir(repo_root)
        .output()
        .with_context(|| format!("Failed to inspect git history in {}", repo_root.display()))?;

    if current_head.status.success() {
        return Ok(Some(
            String::from_utf8_lossy(&current_head.stdout)
                .trim()
                .to_owned(),
        ));
    }

    Ok(None)
}

fn changed_path_to_item(path: &Path) -> Option<SyncItem> {
    let parts = path
        .iter()
        .filter_map(|segment| segment.to_str())
        .collect::<Vec<_>>();

    match parts.as_slice() {
        ["workflow", "workstreams", folder, ..] => Some(SyncItem {
            kind: TrackedKind::Workstream,
            path: PathBuf::from("workflow").join("workstreams").join(folder),
        }),
        ["workflow", "patches", file] if file.ends_with(".md") => Some(SyncItem {
            kind: TrackedKind::Patch,
            path: PathBuf::from("workflow").join("patches").join(file),
        }),
        _ => None,
    }
}

fn sync_workstream(
    gh: &mut GhClient,
    repo: &RepoRef,
    workstream_path: &Path,
    status: &StatusFile,
    mut linkage: GithubLinkage,
    dry_run: bool,
) -> Result<GithubLinkage> {
    let workstream_name = workstream_path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| anyhow!("Invalid workstream path: {}", workstream_path.display()))?;
    let title = workstream_title(workstream_name);
    let desired_state = issue_state_for_status(status.status());

    let parent_body = render_workstream_parent_body(workstream_path, status)?;
    let parent = sync_issue(
        gh,
        repo,
        linkage.parent_issue.as_ref(),
        &title,
        &parent_body,
        desired_state,
        dry_run,
    )?;
    linkage.parent_issue = Some(parent.clone());

    for artifact in ["spec.md", "ux.md", "plan.md", "tasks.md"] {
        let artifact_path = workstream_path.join(artifact);
        let artifact_title = format!("{workstream_name}/{artifact}");
        let artifact_body = render_artifact_body(workstream_path, &artifact_path, status)?;
        let existing = linkage.sub_issues.get(artifact);
        let issue = sync_issue(
            gh,
            repo,
            existing,
            &artifact_title,
            &artifact_body,
            desired_state,
            dry_run,
        )?;
        if !dry_run {
            ensure_sub_issue(gh, repo, parent.number, issue.id)?;
        }
        linkage.sub_issues.insert(artifact.to_owned(), issue);
    }

    Ok(linkage)
}

fn sync_patch(
    gh: &mut GhClient,
    repo: &RepoRef,
    patch_path: &Path,
    status: &StatusFile,
    mut linkage: GithubLinkage,
    dry_run: bool,
) -> Result<GithubLinkage> {
    let patch_name = patch_path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| anyhow!("Invalid patch path: {}", patch_path.display()))?
        .trim_end_matches(".md")
        .to_owned();
    let title = patch_title(&patch_name);
    let desired_state = issue_state_for_status(status.status());
    let body = render_patch_body(patch_path, status)?;
    let issue = sync_issue(
        gh,
        repo,
        linkage.issue.as_ref(),
        &title,
        &body,
        desired_state,
        dry_run,
    )?;
    linkage.issue = Some(issue);
    Ok(linkage)
}

fn sync_issue(
    gh: &mut GhClient,
    repo: &RepoRef,
    existing: Option<&IssueLink>,
    title: &str,
    body: &str,
    desired_state: &str,
    dry_run: bool,
) -> Result<IssueLink> {
    if dry_run {
        if let Some(issue) = existing {
            return Ok(issue.clone());
        }
        return Ok(IssueLink {
            id: 0,
            number: 0,
            url: String::new(),
        });
    }

    let issue = match existing {
        Some(issue) => gh.update_issue(repo, issue.number, title, body, desired_state)?,
        None => gh.create_issue(repo, title, body, desired_state)?,
    };
    Ok(issue)
}

fn ensure_sub_issue(
    gh: &mut GhClient,
    repo: &RepoRef,
    parent_number: u64,
    child_id: u64,
) -> Result<()> {
    let current = gh.list_sub_issues(repo, parent_number)?;
    if current.iter().any(|issue| issue.id == child_id) {
        return Ok(());
    }
    gh.add_sub_issue(repo, parent_number, child_id)
}

fn issue_state_for_status(status: &str) -> &'static str {
    match status {
        "completed" => "closed",
        _ => "open",
    }
}

fn workstream_title(name: &str) -> String {
    if let Some((prefix, slug)) = name.split_once('-') {
        format!("Workstream {prefix}: {}", titleize(slug))
    } else {
        format!("Workstream: {}", titleize(name))
    }
}

fn patch_title(name: &str) -> String {
    if let Some((prefix, slug)) = name.split_once('-') {
        format!("Patch {prefix}: {}", titleize(slug))
    } else {
        format!("Patch: {}", titleize(name))
    }
}

fn render_workstream_parent_body(workstream_path: &Path, status: &StatusFile) -> Result<String> {
    let workstream_name = workstream_path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("<unknown-workstream>");
    Ok(format!(
        "{MANAGED_NOTE}\n\n- Workstream: `{workstream_name}`\n- Status: `{}`\n- Summary: {}\n- Path: `{}`\n- Synced files: `spec.md`, `ux.md`, `plan.md`, `tasks.md`\n",
        status.status(),
        status.summary(),
        workstream_path.display()
    ))
}

fn render_artifact_body(
    workstream_path: &Path,
    artifact_path: &Path,
    status: &StatusFile,
) -> Result<String> {
    let content = fs::read_to_string(artifact_path)
        .with_context(|| format!("Failed to read {}", artifact_path.display()))?;
    let artifact_name = artifact_path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("<unknown-artifact>");
    let workstream_name = workstream_path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("<unknown-workstream>");

    Ok(format!(
        "{MANAGED_NOTE}\n\n- Workstream: `{workstream_name}`\n- Artifact: `{artifact_name}`\n- Status: `{}`\n- Path: `{}`\n\n---\n\n{}",
        status.status(),
        artifact_path.display(),
        content.trim_end()
    ))
}

fn render_patch_body(patch_path: &Path, status: &StatusFile) -> Result<String> {
    let content = fs::read_to_string(patch_path)
        .with_context(|| format!("Failed to read {}", patch_path.display()))?;
    let patch_name = patch_path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("<unknown-patch>");

    Ok(format!(
        "{MANAGED_NOTE}\n\n- Patch: `{}`\n- Status: `{}`\n- Summary: {}\n- Path: `{}`\n\n---\n\n{}",
        patch_name.trim_end_matches(".md"),
        status.status(),
        status.summary(),
        patch_path.display(),
        content.trim_end()
    ))
}

fn linkage_from_status(status: &StatusFile) -> Result<GithubLinkage> {
    match status.extra_value("github") {
        Some(value) => serde_yaml::from_value(value.clone())
            .context("Failed to parse `github` linkage metadata."),
        None => Ok(GithubLinkage::default()),
    }
}

fn set_linkage_on_status(status: &mut StatusFile, linkage: &GithubLinkage) -> Result<()> {
    let value = serde_yaml::to_value(linkage).context("Failed to render GitHub linkage")?;
    status.set_extra_value("github", value);
    Ok(())
}

fn read_config(repo_root: &Path) -> Result<GithubConfig> {
    let path = repo_root.join(CONFIG_PATH);
    let content =
        fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;
    let config: GithubConfig =
        serde_yaml::from_str(&content).with_context(|| format!("Invalid {}", path.display()))?;
    if !config.enabled {
        bail!("GitHub issue support is disabled in {CONFIG_PATH}.");
    }
    if config.repo.trim().is_empty() {
        bail!("Missing `repo` in {CONFIG_PATH}.");
    }
    Ok(config)
}

fn write_config(repo_root: &Path, config: &GithubConfig) -> Result<()> {
    let path = repo_root.join(CONFIG_PATH);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }
    let content = serde_yaml::to_string(config).context("Failed to render GitHub config")?;
    fs::write(&path, content).with_context(|| format!("Failed to write {}", path.display()))
}

fn write_auto_sync_workflow(repo_root: &Path) -> Result<()> {
    let path = repo_root.join(AUTO_SYNC_WORKFLOW_PATH);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }
    fs::write(&path, auto_sync_workflow_contents())
        .with_context(|| format!("Failed to write {}", path.display()))
}

fn auto_sync_workflow_contents() -> String {
    "name: Mnemix GitHub Sync\n\non:\n  push:\n    branches: [main]\n    paths:\n      - \"workflow/**\"\n      - \"workflow/github.yml\"\n  workflow_dispatch:\n\njobs:\n  sync:\n    if: github.actor != 'github-actions[bot]'\n    runs-on: ubuntu-latest\n    permissions:\n      contents: read\n      issues: write\n    env:\n      GH_TOKEN: ${{ github.token }}\n    steps:\n      - name: Checkout\n        uses: actions/checkout@v4\n\n      - name: Install Rust toolchain\n        uses: dtolnay/rust-toolchain@stable\n        with:\n          toolchain: 1.85.0\n\n      - name: Cache cargo artifacts\n        uses: Swatinem/rust-cache@v2\n\n      - name: Sync changed GitHub issue mirrors\n        run: cargo run --bin mxw -- github sync --changed\n"
        .to_owned()
}

fn detect_repo_from_git_remote(repo_root: &Path) -> Result<RepoRef> {
    let output = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .current_dir(repo_root)
        .output()
        .with_context(|| format!("Failed to read git remotes from {}", repo_root.display()))?;

    if !output.status.success() {
        bail!(
            "Failed to detect GitHub repository from git remote origin. Re-run with `--repo owner/name`."
        );
    }

    let remote = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    parse_github_remote(&remote)
        .ok_or_else(|| anyhow!("Could not parse a GitHub owner/repo from `{remote}`."))
}

fn parse_repo_ref(value: &str) -> Result<RepoRef> {
    let trimmed = value.trim().trim_start_matches("https://github.com/");
    let trimmed = trimmed.trim_start_matches("git@github.com:");
    let trimmed = trimmed.trim_end_matches(".git");
    let mut parts = trimmed.split('/');
    let owner = parts.next().unwrap_or_default();
    let repo = parts.next().unwrap_or_default();
    if owner.is_empty() || repo.is_empty() || parts.next().is_some() {
        bail!("GitHub repository must use owner/name format.");
    }
    Ok(RepoRef {
        owner: owner.to_owned(),
        name: repo.to_owned(),
    })
}

fn parse_github_remote(value: &str) -> Option<RepoRef> {
    parse_repo_ref(value).ok()
}

impl RepoRef {
    fn slug(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
}

fn titleize(value: &str) -> String {
    value
        .split(|character: char| character == '-' || character == '_' || character.is_whitespace())
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut characters = segment.chars();
            match characters.next() {
                Some(first) => first.to_uppercase().chain(characters).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

struct GhClient {
    repo_root: PathBuf,
    binary: String,
}

impl GhClient {
    fn new(repo_root: &Path) -> Self {
        Self {
            repo_root: repo_root.to_path_buf(),
            binary: std::env::var("MNEMIX_WORKFLOW_GH_BIN").unwrap_or_else(|_| "gh".to_owned()),
        }
    }

    fn create_issue(
        &mut self,
        repo: &RepoRef,
        title: &str,
        body: &str,
        desired_state: &str,
    ) -> Result<IssueLink> {
        let payload = json!({
            "title": title,
            "body": body,
        });
        let endpoint = format!("/repos/{}/{}/issues", repo.owner, repo.name);
        let issue: IssueResponse = self.api("POST", &endpoint, Some(payload))?;
        let link = IssueLink {
            id: issue.id,
            number: issue.number,
            url: issue.html_url,
        };
        if desired_state == "closed" {
            self.update_issue(repo, link.number, title, body, desired_state)
        } else {
            Ok(link)
        }
    }

    fn update_issue(
        &mut self,
        repo: &RepoRef,
        number: u64,
        title: &str,
        body: &str,
        desired_state: &str,
    ) -> Result<IssueLink> {
        let payload = json!({
            "title": title,
            "body": body,
            "state": desired_state,
        });
        let endpoint = format!("/repos/{}/{}/issues/{number}", repo.owner, repo.name);
        let issue: IssueResponse = self.api("PATCH", &endpoint, Some(payload))?;
        Ok(IssueLink {
            id: issue.id,
            number: issue.number,
            url: issue.html_url,
        })
    }

    fn list_sub_issues(&mut self, repo: &RepoRef, parent_number: u64) -> Result<Vec<IssueLink>> {
        let endpoint = format!(
            "/repos/{}/{}/issues/{parent_number}/sub_issues",
            repo.owner, repo.name
        );
        let issues: Vec<IssueResponse> = self.api("GET", &endpoint, None)?;
        Ok(issues
            .into_iter()
            .map(|issue| IssueLink {
                id: issue.id,
                number: issue.number,
                url: issue.html_url,
            })
            .collect())
    }

    fn add_sub_issue(&mut self, repo: &RepoRef, parent_number: u64, child_id: u64) -> Result<()> {
        let endpoint = format!(
            "/repos/{}/{}/issues/{parent_number}/sub_issues",
            repo.owner, repo.name
        );
        let _response: JsonValue =
            self.api("POST", &endpoint, Some(json!({ "sub_issue_id": child_id })))?;
        Ok(())
    }

    fn api<T: for<'de> Deserialize<'de>>(
        &mut self,
        method: &str,
        endpoint: &str,
        body: Option<JsonValue>,
    ) -> Result<T> {
        let mut command = Command::new(&self.binary);
        command
            .current_dir(&self.repo_root)
            .arg("api")
            .arg(endpoint)
            .args(["-H", "Accept: application/vnd.github+json"])
            .args(["-H", "X-GitHub-Api-Version: 2022-11-28"]);

        if method != "GET" {
            command.args(["--method", method]);
        }

        if body.is_some() {
            command.args(["--input", "-"]);
            command.stdin(Stdio::piped());
        }

        command.stdout(Stdio::piped()).stderr(Stdio::piped());
        let mut child = command
            .spawn()
            .with_context(|| format!("Failed to spawn `{}`.", self.binary))?;

        if let Some(body) = body {
            let stdin = child
                .stdin
                .as_mut()
                .ok_or_else(|| anyhow!("Failed to open stdin for `{}`.", self.binary))?;
            stdin
                .write_all(body.to_string().as_bytes())
                .with_context(|| format!("Failed to write request body to `{}`.", self.binary))?;
        }

        let output = child
            .wait_with_output()
            .with_context(|| format!("Failed to wait for `{}`.", self.binary))?;

        if !output.status.success() {
            bail!(
                "GitHub API call failed for `{endpoint}`: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }

        serde_json::from_slice(&output.stdout)
            .with_context(|| format!("Failed to parse GitHub API response for `{endpoint}`."))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{GithubConfig, GithubLinkage, IssueLink, RepoRef, parse_repo_ref, titleize};

    #[test]
    fn parse_repo_ref_accepts_common_formats() {
        let repo = parse_repo_ref("micahcourey/mnemix-workflow").expect("repo");
        assert_eq!(repo.slug(), "micahcourey/mnemix-workflow");

        let repo =
            parse_repo_ref("https://github.com/micahcourey/mnemix-workflow.git").expect("repo");
        assert_eq!(repo.slug(), "micahcourey/mnemix-workflow");
    }

    #[test]
    fn titleize_formats_slugs() {
        assert_eq!(titleize("github-issue-support"), "Github Issue Support");
    }

    #[test]
    fn github_linkage_round_trips() {
        let linkage = GithubLinkage {
            issue: Some(IssueLink {
                id: 1,
                number: 2,
                url: "https://example.com/2".to_owned(),
            }),
            parent_issue: None,
            sub_issues: BTreeMap::new(),
        };

        let value = serde_yaml::to_value(&linkage).expect("value");
        let parsed: GithubLinkage = serde_yaml::from_value(value).expect("parse");
        assert_eq!(parsed.issue.expect("issue").number, 2);
    }

    #[test]
    fn config_defaults_auto_sync_to_changed() {
        let config = GithubConfig {
            enabled: true,
            repo: RepoRef {
                owner: "micahcourey".to_owned(),
                name: "mnemix-workflow".to_owned(),
            }
            .slug(),
            auto_sync: Default::default(),
        };

        assert!(!config.auto_sync.enabled);
        assert_eq!(config.auto_sync.mode, "changed");
    }
}

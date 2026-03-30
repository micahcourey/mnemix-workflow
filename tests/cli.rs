use std::{env, fs, path::PathBuf, process::Command};

use assert_cmd::Command as AssertCommand;
use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use chrono::Utc;
use predicates::prelude::PredicateBooleanExt;
use predicates::str::contains;
use serde_json::Value;
use tempfile::TempDir;

fn init_git_repo() -> TempDir {
    let temp_dir = TempDir::new().expect("tempdir");
    let status = Command::new("git")
        .arg("init")
        .arg("-b")
        .arg("main")
        .current_dir(temp_dir.path())
        .status()
        .expect("git init should run");
    assert!(status.success());
    let config_email = Command::new("git")
        .args(["config", "user.email", "codex@example.com"])
        .current_dir(temp_dir.path())
        .status()
        .expect("git config email should run");
    assert!(config_email.success());
    let config_name = Command::new("git")
        .args(["config", "user.name", "Codex"])
        .current_dir(temp_dir.path())
        .status()
        .expect("git config name should run");
    assert!(config_name.success());
    temp_dir
}

fn hook_path(name: &str) -> String {
    format!("{}/resources/hooks/{name}", env!("CARGO_MANIFEST_DIR"))
}

struct FakeGh {
    bin_dir: PathBuf,
    state_path: PathBuf,
    log_path: PathBuf,
}

impl FakeGh {
    fn install(temp_dir: &TempDir) -> Self {
        let bin_dir = temp_dir.path().join("fake-gh-bin");
        fs::create_dir_all(&bin_dir).expect("create fake gh bin dir");
        let script_path = bin_dir.join("gh");
        fs::write(&script_path, fake_gh_script()).expect("write fake gh script");

        let chmod_status = Command::new("chmod")
            .args(["+x", script_path.to_str().expect("script path")])
            .status()
            .expect("chmod fake gh");
        assert!(chmod_status.success());

        Self {
            bin_dir,
            state_path: temp_dir.path().join("gh-state.json"),
            log_path: temp_dir.path().join("gh-log.jsonl"),
        }
    }

    fn command(&self, bin: &str) -> AssertCommand {
        let mut command = AssertCommand::cargo_bin(bin).expect("binary");
        let existing_path = env::var_os("PATH").unwrap_or_default();
        let mut paths = vec![self.bin_dir.clone()];
        paths.extend(env::split_paths(&existing_path));
        let joined = env::join_paths(paths).expect("join paths");

        command.env("PATH", joined);
        command.env("GH_STATE_FILE", &self.state_path);
        command.env("GH_LOG_FILE", &self.log_path);
        command.env("MNEMIX_WORKFLOW_GH_BIN", "gh");
        command
    }

    fn state(&self) -> Value {
        if !self.state_path.exists() {
            return serde_json::json!({
                "next_id": 1000,
                "next_number": 1,
                "issues": {},
                "sub_issues": {}
            });
        }

        let content = fs::read_to_string(&self.state_path).expect("read fake gh state");
        serde_json::from_str(&content).expect("parse fake gh state")
    }
}

fn git_commit_all(repo_root: &PathBuf, message: &str) {
    let add_status = Command::new("git")
        .args(["add", "."])
        .current_dir(repo_root)
        .status()
        .expect("git add");
    assert!(add_status.success());

    let commit_status = Command::new("git")
        .args(["commit", "-m", message])
        .current_dir(repo_root)
        .status()
        .expect("git commit");
    assert!(commit_status.success());
}

fn fake_gh_script() -> &'static str {
    r#"#!/usr/bin/env python3
import json
import os
import sys
from pathlib import Path

state_path = Path(os.environ["GH_STATE_FILE"])
log_path = Path(os.environ["GH_LOG_FILE"])

if state_path.exists():
    state = json.loads(state_path.read_text())
else:
    state = {"next_id": 1000, "next_number": 1, "issues": {}, "sub_issues": {}}

args = sys.argv[1:]
if not args or args[0] != "api":
    print("fake gh only supports `gh api`", file=sys.stderr)
    sys.exit(1)

endpoint = None
method = "GET"
body = None
i = 1
while i < len(args):
    arg = args[i]
    if endpoint is None:
        endpoint = arg
        i += 1
        continue
    if arg in ("--method", "-X"):
        method = args[i + 1]
        i += 2
        continue
    if arg == "--input":
        source = args[i + 1]
        i += 2
        if source == "-":
            raw = sys.stdin.read()
            body = json.loads(raw) if raw.strip() else None
        else:
            body = json.loads(Path(source).read_text())
        continue
    if arg == "-H":
        i += 2
        continue
    i += 1

with log_path.open("a", encoding="utf-8") as handle:
    handle.write(json.dumps({"endpoint": endpoint, "method": method, "body": body}) + "\n")

def save_and_print(payload):
    state_path.write_text(json.dumps(state))
    print(json.dumps(payload))
    sys.exit(0)

def issue_by_id(issue_id):
    for issue in state["issues"].values():
        if issue["id"] == issue_id:
            return issue
    return None

endpoint = endpoint or ""
endpoint = endpoint.lstrip("/")
parts = endpoint.split("/")
if len(parts) < 4 or parts[0] != "repos":
    print(f"unsupported endpoint: {endpoint}", file=sys.stderr)
    sys.exit(1)

owner = parts[1]
repo = parts[2]

if parts[3] != "issues":
    print(f"unsupported endpoint: {endpoint}", file=sys.stderr)
    sys.exit(1)

if len(parts) == 4 and method == "POST":
    number = state["next_number"]
    issue_id = state["next_id"]
    state["next_number"] += 1
    state["next_id"] += 1
    issue = {
        "id": issue_id,
        "number": number,
        "html_url": f"https://github.com/{owner}/{repo}/issues/{number}",
        "title": body["title"],
        "body": body["body"],
        "state": "open",
    }
    state["issues"][str(number)] = issue
    save_and_print(issue)

if len(parts) == 5 and method == "PATCH":
    number = parts[4]
    issue = state["issues"][number]
    issue["title"] = body.get("title", issue["title"])
    issue["body"] = body.get("body", issue["body"])
    issue["state"] = body.get("state", issue["state"])
    save_and_print(issue)

if len(parts) == 6 and parts[5] == "sub_issues" and method == "GET":
    parent = parts[4]
    numbers = state["sub_issues"].get(parent, [])
    payload = [state["issues"][str(number)] for number in numbers]
    save_and_print(payload)

if len(parts) == 6 and parts[5] == "sub_issues" and method == "POST":
    parent = parts[4]
    child = issue_by_id(body["sub_issue_id"])
    if child is None:
        print("unknown child issue id", file=sys.stderr)
        sys.exit(1)
    numbers = state["sub_issues"].setdefault(parent, [])
    if child["number"] not in numbers:
        numbers.append(child["number"])
    save_and_print(child)

print(f"unsupported endpoint/method: {endpoint} {method}", file=sys.stderr)
sys.exit(1)
"#
}

#[test]
fn help_lists_ui_command() {
    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("ui"))
        .stdout(contains("github"))
        .stdout(contains("hooks"))
        .stdout(contains("validate"))
        .stdout(contains("openapi"))
        .stdout(contains("asyncapi"))
        .stdout(contains("schema"));
}

#[test]
fn mnx_help_describes_the_tui_shortcut() {
    Command::cargo_bin("mnx")
        .expect("binary")
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("interactive Mnemix Workflow TUI"))
        .stdout(contains("Usage:\n  mnx"));
}

#[test]
fn github_init_writes_config_and_optional_auto_sync_workflow() {
    let temp_dir = init_git_repo();
    let fake_gh = FakeGh::install(&temp_dir);

    fake_gh
        .command("mxw")
        .args([
            "github",
            "init",
            "--repo",
            "micahcourey/mnemix-workflow",
            "--enable-auto-sync",
        ])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("workflow/github.yml"))
        .stdout(contains(".github/workflows/mxw-github-sync.yml"));

    let config = fs::read_to_string(temp_dir.path().join("workflow/github.yml")).expect("config");
    assert!(config.contains("repo: micahcourey/mnemix-workflow"));
    assert!(config.contains("enabled: true"));

    let workflow = fs::read_to_string(
        temp_dir
            .path()
            .join(".github/workflows/mxw-github-sync.yml"),
    )
    .expect("workflow");
    assert!(workflow.contains("github sync --changed"));
}

#[test]
fn github_sync_workstream_creates_parent_and_sub_issues_and_metadata() {
    let temp_dir = init_git_repo();
    let fake_gh = FakeGh::install(&temp_dir);

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();
    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "github issue support"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    fake_gh
        .command("mxw")
        .args(["github", "init", "--repo", "octocat/hello-world"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    fake_gh
        .command("mxw")
        .args(["github", "sync", "001"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains(
            "Synced workstream: 001-github-issue-support -> #1",
        ));

    let status = fs::read_to_string(
        temp_dir
            .path()
            .join("workflow/workstreams/001-github-issue-support/STATUS.md"),
    )
    .expect("status");
    assert!(status.contains("parent_issue:"));
    assert!(status.contains("number: 1"));
    assert!(status.contains("spec.md:"));
    assert!(status.contains("number: 2"));
    assert!(status.contains("ux.md:"));
    assert!(status.contains("plan.md:"));
    assert!(status.contains("tasks.md:"));

    let state = fake_gh.state();
    assert_eq!(state["issues"].as_object().expect("issues").len(), 5);
    let parent_children = state["sub_issues"]["1"].as_array().expect("sub issues");
    assert_eq!(parent_children.len(), 4);
}

#[test]
fn init_creates_workflow_structure() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("Initialized Mnemix Workflow"));

    assert!(
        temp_dir
            .path()
            .join("workflow/decisions/README.md")
            .is_file()
    );
    assert!(temp_dir.path().join("workflow/workstreams").is_dir());
    assert!(temp_dir.path().join("workflow/patches").is_dir());
}

#[test]
fn init_is_safe_to_rerun() {
    let temp_dir = init_git_repo();

    fs::create_dir_all(temp_dir.path().join("workflow/decisions")).expect("decisions dir");
    fs::write(
        temp_dir.path().join("workflow/decisions/README.md"),
        "custom decisions readme",
    )
    .expect("custom decisions readme");
    fs::create_dir_all(temp_dir.path().join("workflow/workstreams")).expect("workstreams dir");
    fs::create_dir_all(temp_dir.path().join("workflow/patches")).expect("patches dir");

    Command::cargo_bin("mnemix-workflow")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("already initialized"));

    let readme = fs::read_to_string(temp_dir.path().join("workflow/decisions/README.md"))
        .expect("read decisions readme");
    assert_eq!(readme, "custom decisions readme");
}

#[test]
fn new_requires_init_first() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "user profile redesign"])
        .current_dir(temp_dir.path())
        .assert()
        .failure()
        .stderr(contains("Run `mxw init` first."));
}

#[test]
fn new_creates_first_workstream_after_init() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "user profile redesign"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("workflow/workstreams/001-user-profile-redesign"));

    let workstream_dir = temp_dir
        .path()
        .join("workflow/workstreams/001-user-profile-redesign");
    assert!(workstream_dir.join("STATUS.md").is_file());
    assert!(workstream_dir.join("spec.md").is_file());
    assert!(workstream_dir.join("ux.md").is_file());
    assert!(workstream_dir.join("plan.md").is_file());
    assert!(workstream_dir.join("tasks.md").is_file());
    assert!(workstream_dir.join("decisions/README.md").is_file());

    let status = fs::read_to_string(workstream_dir.join("STATUS.md")).expect("read status");
    assert!(status.contains("status: open"));
    assert!(
        status.contains("summary: User Profile Redesign is active and ready for implementation.")
    );
}

#[test]
fn new_rejects_invalid_names() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "!!!"])
        .current_dir(temp_dir.path())
        .assert()
        .failure()
        .stderr(contains("Name must contain at least one letter or digit."));
}

#[test]
fn patch_new_creates_first_patch_after_init() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["patch", "new", "fix status copy"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("workflow/patches/0001-fix-status-copy.md"));

    let patch_path = temp_dir
        .path()
        .join("workflow/patches/0001-fix-status-copy.md");
    assert!(patch_path.is_file());

    let patch = fs::read_to_string(&patch_path).expect("read patch");
    assert!(patch.contains("status: open"));
    assert!(patch.contains("summary: Fix Status Copy is active and ready for implementation."));
    assert!(patch.contains("# Patch: Fix Status Copy"));
}

#[test]
fn patch_new_backfills_missing_patches_dir() {
    let temp_dir = init_git_repo();

    fs::create_dir_all(temp_dir.path().join("workflow/decisions")).expect("decisions dir");
    fs::write(
        temp_dir.path().join("workflow/decisions/README.md"),
        "custom decisions readme",
    )
    .expect("custom decisions readme");
    fs::create_dir_all(temp_dir.path().join("workflow/workstreams")).expect("workstreams dir");

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["patch", "new", "fix status copy"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("workflow/patches/0001-fix-status-copy.md"));

    assert!(temp_dir.path().join("workflow/patches").is_dir());
}

#[test]
fn github_sync_patch_creates_issue_and_records_linkage() {
    let temp_dir = init_git_repo();
    let fake_gh = FakeGh::install(&temp_dir);

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();
    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["patch", "new", "fix status copy"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    fake_gh
        .command("mxw")
        .args(["github", "init", "--repo", "octocat/hello-world"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    fake_gh
        .command("mxw")
        .args(["github", "sync", "0001-fix-status-copy.md"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("Synced patch: 0001-fix-status-copy -> #1"));

    let patch = fs::read_to_string(
        temp_dir
            .path()
            .join("workflow/patches/0001-fix-status-copy.md"),
    )
    .expect("patch");
    assert!(patch.contains("github:"));
    assert!(patch.contains("issue:"));
    assert!(patch.contains("number: 1"));
}

#[test]
fn github_sync_all_and_filtered_status_handle_completed_items() {
    let temp_dir = init_git_repo();
    let fake_gh = FakeGh::install(&temp_dir);

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();
    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "open workstream"])
        .current_dir(temp_dir.path())
        .assert()
        .success();
    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "completed workstream"])
        .current_dir(temp_dir.path())
        .assert()
        .success();
    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["status", "set", "002", "completed"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    fake_gh
        .command("mxw")
        .args(["github", "init", "--repo", "octocat/hello-world"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    fake_gh
        .command("mxw")
        .args(["github", "sync", "--status", "open", "--all"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    let state = fake_gh.state();
    assert_eq!(state["issues"].as_object().expect("issues").len(), 5);
    assert!(
        state["issues"]["1"]["title"]
            .as_str()
            .expect("title")
            .contains("Open Workstream")
    );

    fake_gh
        .command("mxw")
        .args(["github", "sync", "--all"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    let state = fake_gh.state();
    assert_eq!(state["issues"].as_object().expect("issues").len(), 10);
    assert_eq!(
        state["issues"]["6"]["state"].as_str().expect("state"),
        "closed"
    );
}

#[test]
fn github_sync_changed_updates_only_linked_changed_items() {
    let temp_dir = init_git_repo();
    let fake_gh = FakeGh::install(&temp_dir);

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();
    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "linked workstream"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    fake_gh
        .command("mxw")
        .args(["github", "init", "--repo", "octocat/hello-world"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    fake_gh
        .command("mxw")
        .args(["github", "sync", "001"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    git_commit_all(&temp_dir.path().to_path_buf(), "initial state");

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "unlinked workstream"])
        .current_dir(temp_dir.path())
        .assert()
        .success();
    fs::write(
        temp_dir
            .path()
            .join("workflow/workstreams/001-linked-workstream/spec.md"),
        "# Feature Spec: Linked Workstream\n\nUpdated content.\n",
    )
    .expect("write linked spec");

    git_commit_all(&temp_dir.path().to_path_buf(), "changed items");

    fake_gh
        .command("mxw")
        .args(["github", "sync", "--changed"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("Synced workstream: 001-linked-workstream -> #1"))
        .stdout(contains(
            "Skipped unlinked workstream in --changed mode: 002-unlinked-workstream",
        ));

    let state = fake_gh.state();
    assert_eq!(state["issues"].as_object().expect("issues").len(), 5);
}

#[test]
fn openapi_init_and_validate_workstream_contract() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "api contracts"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["openapi", "init", "001"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains(
            "workflow/workstreams/001-api-contracts/contracts/openapi.yaml",
        ));

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["openapi", "validate", "001"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("Validated OpenAPI contract"));
}

#[test]
fn asyncapi_init_and_validate_workstream_contract() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "event contracts"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["asyncapi", "init", "001"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains(
            "workflow/workstreams/001-event-contracts/contracts/asyncapi.yaml",
        ));

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["asyncapi", "validate", "001"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("Validated AsyncAPI contract"));
}

#[test]
fn schema_new_and_validate_workstream_contracts() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "schema contracts"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["schema", "new", "001", "repository event"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains(
            "workflow/workstreams/001-schema-contracts/contracts/schemas/repository-event.schema.json",
        ));

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["schema", "validate", "001"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("Validated 1 JSON Schema file(s)."))
        .stdout(contains("repository-event.schema.json"));
}

#[test]
fn status_show_reads_workstream_metadata() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "user profile redesign"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["status", "001"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("Status: open"))
        .stdout(contains(
            "Summary: User Profile Redesign is active and ready for implementation.",
        ));
}

#[test]
fn patch_status_show_reads_patch_metadata() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["patch", "new", "fix status copy"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["patch", "status", "0001"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("Patch: 0001-fix-status-copy"))
        .stdout(contains("Status: open"));
}

#[test]
fn status_set_updates_status_summary_and_prs() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "user profile redesign"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args([
            "status",
            "set",
            "001",
            "completed",
            "--summary",
            "User profile redesign shipped.",
            "--pr",
            "12",
            "--pr",
            "13",
        ])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("Status: completed"))
        .stdout(contains("PRs: 12, 13"));

    let status = fs::read_to_string(
        temp_dir
            .path()
            .join("workflow/workstreams/001-user-profile-redesign/STATUS.md"),
    )
    .expect("status file");
    assert!(status.contains("status: completed"));
    assert!(status.contains("summary: User profile redesign shipped."));
    assert!(status.contains("prs:"));
    assert!(status.contains("- 12"));
    assert!(status.contains("- 13"));
}

#[test]
fn patch_status_set_updates_status_summary_and_prs() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["patch", "new", "fix status copy"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args([
            "patch",
            "status",
            "set",
            "0001",
            "completed",
            "--summary",
            "Fixed the status copy.",
            "--pr",
            "22",
        ])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("Updated status for patch: 0001-fix-status-copy"))
        .stdout(contains("Status: completed"))
        .stdout(contains("PRs: 22"));

    let patch = fs::read_to_string(
        temp_dir
            .path()
            .join("workflow/patches/0001-fix-status-copy.md"),
    )
    .expect("patch file");
    assert!(patch.contains("status: completed"));
    assert!(patch.contains("summary: Fixed the status copy."));
    assert!(patch.contains("prs:"));
    assert!(patch.contains("- 22"));
}

#[test]
fn status_set_rejects_invalid_status_values() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "user profile redesign"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["status", "set", "001", "done"])
        .current_dir(temp_dir.path())
        .assert()
        .failure()
        .stderr(contains("Unsupported status `done`"));
}

#[test]
fn status_list_filters_by_status() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "user profile redesign"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "status automation"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["status", "set", "002", "completed", "--pr", "7"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["status", "list", "--status", "completed"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("002-status-automation: completed"))
        .stdout(contains("PRs: 7"))
        .stdout(predicates::str::contains("001-user-profile-redesign").not());
}

#[test]
fn patch_status_list_filters_by_status() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["patch", "new", "fix status copy"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["patch", "new", "adjust hook copy"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["patch", "status", "set", "0002", "completed", "--pr", "33"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["patch", "status", "list", "--status", "completed"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("0002-adjust-hook-copy: completed"))
        .stdout(contains("PRs: 33"))
        .stdout(predicates::str::contains("0001-fix-status-copy").not());
}

#[test]
fn status_list_rejects_invalid_filter_values() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["status", "list", "--status", "done"])
        .current_dir(temp_dir.path())
        .assert()
        .failure()
        .stderr(contains("Unsupported status `done`"));
}

#[test]
fn status_show_rejects_malformed_frontmatter() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "user profile redesign"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    let status_path = temp_dir
        .path()
        .join("workflow/workstreams/001-user-profile-redesign/STATUS.md");
    fs::write(
        &status_path,
        "---\nstatus: open\nsummary: Bad\nupdated: 2026-03-28\nprs: nope\n---\n\n# Status\n",
    )
    .expect("write malformed status");

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["status", "001"])
        .current_dir(temp_dir.path())
        .assert()
        .failure()
        .stderr(contains("`prs` must be a list"));
}

#[test]
fn status_show_fails_when_status_file_is_missing() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "user profile redesign"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    let status_path = temp_dir
        .path()
        .join("workflow/workstreams/001-user-profile-redesign/STATUS.md");
    fs::remove_file(&status_path).expect("remove status");

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["status", "001"])
        .current_dir(temp_dir.path())
        .assert()
        .failure()
        .stderr(contains("Failed to read"));
}

#[test]
fn pre_commit_hook_refreshes_updated_field() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "user profile redesign"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    let status_path = temp_dir
        .path()
        .join("workflow/workstreams/001-user-profile-redesign/STATUS.md");
    let stale = fs::read_to_string(&status_path)
        .expect("read status")
        .replace(&Utc::now().date_naive().to_string(), "2020-01-01");
    fs::write(&status_path, stale).expect("write stale status");

    let spec_path = temp_dir
        .path()
        .join("workflow/workstreams/001-user-profile-redesign/spec.md");
    fs::write(&spec_path, "# Feature Spec: Updated\n").expect("write spec");

    let add_status = Command::new("git")
        .args([
            "add",
            "workflow/workstreams/001-user-profile-redesign/spec.md",
        ])
        .current_dir(temp_dir.path())
        .status()
        .expect("git add should run");
    assert!(add_status.success());

    Command::new("python3")
        .arg(hook_path("pre-commit-status-updated"))
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains(
            "Updated workflow/workstreams/001-user-profile-redesign/STATUS.md",
        ));

    let status = fs::read_to_string(&status_path).expect("read status");
    assert!(status.contains(&format!("updated: {}", Utc::now().date_naive())));

    let staged = Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .current_dir(temp_dir.path())
        .output()
        .expect("git diff cached");
    let staged = String::from_utf8_lossy(&staged.stdout);
    assert!(staged.contains("workflow/workstreams/001-user-profile-redesign/STATUS.md"));
}

#[test]
fn pre_commit_hook_refreshes_updated_field_for_patches() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["patch", "new", "fix status copy"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    let patch_path = temp_dir
        .path()
        .join("workflow/patches/0001-fix-status-copy.md");
    let stale = fs::read_to_string(&patch_path)
        .expect("read patch")
        .replace(&Utc::now().date_naive().to_string(), "2020-01-01");
    fs::write(&patch_path, stale).expect("write stale patch");

    let add_patch = Command::new("git")
        .args(["add", "workflow/patches/0001-fix-status-copy.md"])
        .current_dir(temp_dir.path())
        .status()
        .expect("git add patch should run");
    assert!(add_patch.success());

    Command::new("python3")
        .arg(hook_path("pre-commit-status-updated"))
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("Updated workflow/patches/0001-fix-status-copy.md"));

    let patch = fs::read_to_string(&patch_path).expect("read patch");
    assert!(patch.contains(&format!("updated: {}", Utc::now().date_naive())));
}

#[test]
fn pre_push_hook_warns_for_touched_workstreams() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "user profile redesign"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    let add_all = Command::new("git")
        .args(["add", "."])
        .current_dir(temp_dir.path())
        .status()
        .expect("git add should run");
    assert!(add_all.success());

    let first_commit = Command::new("git")
        .args(["commit", "-m", "initial workflow"])
        .current_dir(temp_dir.path())
        .status()
        .expect("git commit should run");
    assert!(first_commit.success());

    let spec_path = temp_dir
        .path()
        .join("workflow/workstreams/001-user-profile-redesign/spec.md");
    fs::write(&spec_path, "# Feature Spec: Updated Again\n").expect("write spec");

    let add_spec = Command::new("git")
        .args([
            "add",
            "workflow/workstreams/001-user-profile-redesign/spec.md",
        ])
        .current_dir(temp_dir.path())
        .status()
        .expect("git add spec should run");
    assert!(add_spec.success());

    let second_commit = Command::new("git")
        .args(["commit", "-m", "touch workstream"])
        .current_dir(temp_dir.path())
        .status()
        .expect("git commit should run");
    assert!(second_commit.success());

    let local_sha = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(temp_dir.path())
        .output()
        .expect("rev-parse head");
    let local_sha = String::from_utf8_lossy(&local_sha.stdout).trim().to_owned();

    let remote_sha = Command::new("git")
        .args(["rev-parse", "HEAD~1"])
        .current_dir(temp_dir.path())
        .output()
        .expect("rev-parse head~1");
    let remote_sha = String::from_utf8_lossy(&remote_sha.stdout)
        .trim()
        .to_owned();

    let mut command = AssertCommand::new("python3");
    command
        .arg(hook_path("pre-push-status-reminder"))
        .current_dir(temp_dir.path())
        .write_stdin(format!(
            "refs/heads/main {local_sha} refs/heads/main {remote_sha}\n"
        ))
        .assert()
        .success()
        .stdout(contains("Mnemix Workflow reminder:"))
        .stdout(contains("Review workflow/workstreams/001-user-profile-redesign/STATUS.md"))
        .stdout(contains(
            "If this push leads to a PR that completes the workstream, update STATUS.md to completed.",
        ));
}

#[test]
fn pre_push_hook_warns_for_touched_patches() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["patch", "new", "fix status copy"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    let add_all = Command::new("git")
        .args(["add", "."])
        .current_dir(temp_dir.path())
        .status()
        .expect("git add should run");
    assert!(add_all.success());

    let first_commit = Command::new("git")
        .args(["commit", "-m", "initial workflow"])
        .current_dir(temp_dir.path())
        .status()
        .expect("git commit should run");
    assert!(first_commit.success());

    let patch_path = temp_dir
        .path()
        .join("workflow/patches/0001-fix-status-copy.md");
    let updated_patch = fs::read_to_string(&patch_path)
        .expect("read patch")
        .replace("## Validation", "## Validation\n\n- Confirmed locally.\n");
    fs::write(&patch_path, updated_patch).expect("write patch");

    let add_patch = Command::new("git")
        .args(["add", "workflow/patches/0001-fix-status-copy.md"])
        .current_dir(temp_dir.path())
        .status()
        .expect("git add patch should run");
    assert!(add_patch.success());

    let second_commit = Command::new("git")
        .args(["commit", "-m", "touch patch"])
        .current_dir(temp_dir.path())
        .status()
        .expect("git commit should run");
    assert!(second_commit.success());

    let local_sha = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(temp_dir.path())
        .output()
        .expect("rev-parse head");
    let local_sha = String::from_utf8_lossy(&local_sha.stdout).trim().to_owned();

    let remote_sha = Command::new("git")
        .args(["rev-parse", "HEAD~1"])
        .current_dir(temp_dir.path())
        .output()
        .expect("rev-parse head~1");
    let remote_sha = String::from_utf8_lossy(&remote_sha.stdout)
        .trim()
        .to_owned();

    let mut command = AssertCommand::new("python3");
    command
        .arg(hook_path("pre-push-status-reminder"))
        .current_dir(temp_dir.path())
        .write_stdin(format!(
            "refs/heads/main {local_sha} refs/heads/main {remote_sha}\n"
        ))
        .assert()
        .success()
        .stdout(contains("Review workflow/patches/0001-fix-status-copy.md"))
        .stdout(contains(
            "If this push leads to a PR that completes the patch, update its status to completed.",
        ));
}

#[test]
fn hooks_install_writes_bundled_git_hooks() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["hooks", "install"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("Installed Mnemix Workflow git hooks."));

    let pre_commit = temp_dir.path().join(".git/hooks/pre-commit");
    let pre_push = temp_dir.path().join(".git/hooks/pre-push");
    assert!(pre_commit.is_file());
    assert!(pre_push.is_file());

    let installed_pre_commit = fs::read_to_string(&pre_commit).expect("read pre-commit");
    let bundled_pre_commit = fs::read_to_string(hook_path("pre-commit-status-updated"))
        .expect("read bundled pre-commit");
    assert_eq!(installed_pre_commit, bundled_pre_commit);

    let installed_pre_push = fs::read_to_string(&pre_push).expect("read pre-push");
    let bundled_pre_push =
        fs::read_to_string(hook_path("pre-push-status-reminder")).expect("read bundled pre-push");
    assert_eq!(installed_pre_push, bundled_pre_push);
}

#[test]
fn validate_checks_repository_status_and_contracts() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "contract sweep"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["patch", "new", "fix docs copy"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["openapi", "init", "001"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["schema", "new", "001", "payload"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("validate")
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("Repository validation passed."))
        .stdout(contains("Workstreams checked: 1"))
        .stdout(contains("Patches checked: 1"))
        .stdout(contains("Contracts checked: 2"));
}

#[test]
fn validate_accepts_specific_workstream_and_patch_targets() {
    let temp_dir = init_git_repo();

    Command::cargo_bin("mxw")
        .expect("binary")
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["new", "contract sweep"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["patch", "new", "fix docs copy"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["openapi", "init", "001"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["validate", "001"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("Validated workstream: 001-contract-sweep"))
        .stdout(contains("Contracts checked: 1"));

    Command::cargo_bin("mxw")
        .expect("binary")
        .args(["validate", "0001"])
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("Validated patch: 0001-fix-docs-copy"));
}

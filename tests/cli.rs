use std::{fs, process::Command};

use assert_cmd::assert::OutputAssertExt;
use assert_cmd::Command as AssertCommand;
use assert_cmd::cargo::CommandCargoExt;
use chrono::Utc;
use predicates::prelude::PredicateBooleanExt;
use predicates::str::contains;
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
    format!(
        "{}/resources/hooks/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
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
    assert!(status.contains("summary: User Profile Redesign is active and ready for implementation."));
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
        .stdout(contains("Summary: User Profile Redesign is active and ready for implementation."));
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
        .args(["add", "workflow/workstreams/001-user-profile-redesign/spec.md"])
        .current_dir(temp_dir.path())
        .status()
        .expect("git add should run");
    assert!(add_status.success());

    Command::new("python3")
        .arg(hook_path("pre-commit-status-updated"))
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("Updated workflow/workstreams/001-user-profile-redesign/STATUS.md"));

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
        .args(["add", "workflow/workstreams/001-user-profile-redesign/spec.md"])
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
    let remote_sha = String::from_utf8_lossy(&remote_sha.stdout).trim().to_owned();

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

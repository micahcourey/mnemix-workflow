use std::{fs, process::Command};

use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
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
    temp_dir
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
    assert!(workstream_dir.join("spec.md").is_file());
    assert!(workstream_dir.join("ux.md").is_file());
    assert!(workstream_dir.join("plan.md").is_file());
    assert!(workstream_dir.join("tasks.md").is_file());
    assert!(workstream_dir.join("decisions/README.md").is_file());
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

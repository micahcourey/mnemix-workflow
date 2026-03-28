use std::path::Path;

use anyhow::Result;

use crate::scaffold::{find_repo_root, init_repository};

pub(crate) fn run(cwd: &Path, program: &str) -> Result<Vec<String>> {
    let repo_root = find_repo_root(cwd)?;
    let created = init_repository(&repo_root)?;
    let action = if created {
        "Initialized Mnemix Workflow"
    } else {
        "Mnemix Workflow already initialized"
    };

    Ok(vec![
        format!("{action} in: {}", repo_root.display()),
        format!("Next step: run {program} new \"<workstream name>\""),
    ])
}

use std::path::Path;

use anyhow::Result;

use crate::scaffold::{create_workstream, ensure_initialized, find_repo_root};

pub(crate) fn run(cwd: &Path, program: &str, name: &str) -> Result<Vec<String>> {
    let repo_root = find_repo_root(cwd)?;
    ensure_initialized(&repo_root, program)?;
    let destination = create_workstream(&repo_root, name)?;

    Ok(vec![
        format!("Created workstream: {}", destination.display()),
        "Next step: fill in STATUS.md, spec.md, ux.md, plan.md, and tasks.md".to_owned(),
    ])
}

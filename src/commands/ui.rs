use std::path::Path;

use anyhow::Result;

use crate::{
    scaffold::{ensure_initialized, find_repo_root},
    tui,
};

pub(crate) fn run(cwd: &Path, program: &str) -> Result<Vec<String>> {
    let repo_root = find_repo_root(cwd)?;
    ensure_initialized(&repo_root, program)?;
    tui::run(&repo_root)?;
    Ok(Vec::new())
}

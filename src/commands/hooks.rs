use std::path::Path;

use anyhow::Result;

use crate::{
    cli::{HooksAction, HooksArgs},
    hooks::install_hooks,
    scaffold::{ensure_initialized, find_repo_root},
};

pub(crate) fn run(cwd: &Path, program: &str, args: HooksArgs) -> Result<Vec<String>> {
    let repo_root = find_repo_root(cwd)?;
    ensure_initialized(&repo_root, program)?;

    match args.action {
        HooksAction::Install(args) => {
            let installed = install_hooks(&repo_root, args.force)?;
            let mut lines = vec!["Installed Mnemix Workflow git hooks.".to_owned()];
            for hook in installed {
                let state = if hook.installed {
                    "installed"
                } else {
                    "already current"
                };
                lines.push(format!("- {} ({state})", hook.path.display()));
            }
            lines.push(
                "These hooks refresh `updated` metadata on commit and remind you to review status before pushing."
                    .to_owned(),
            );
            Ok(lines)
        }
    }
}

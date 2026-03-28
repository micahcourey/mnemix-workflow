use std::path::Path;

use anyhow::Result;

use crate::{
    cli::{PatchAction, PatchArgs},
    scaffold::{create_patch, ensure_initialized, find_repo_root},
    status::TrackedKind,
};

use super::status;

pub(crate) fn run(cwd: &Path, program: &str, args: PatchArgs) -> Result<Vec<String>> {
    let repo_root = find_repo_root(cwd)?;
    ensure_initialized(&repo_root, program)?;

    match args.action {
        PatchAction::New(args) => {
            let destination = create_patch(&repo_root, &args.name)?;
            Ok(vec![
                format!("Created patch: {}", destination.display()),
                "Next step: fill in the patch summary, scope, implementation notes, and validation."
                    .to_owned(),
            ])
        }
        PatchAction::Status(args) => {
            status::run_for_kind(cwd, program, args.patch, args.action, TrackedKind::Patch)
        }
    }
}

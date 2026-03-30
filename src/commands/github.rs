use std::path::Path;

use anyhow::{Result, bail};

use crate::{
    cli::{GithubAction, GithubArgs, GithubSyncArgs},
    github::{SyncMode, SyncRequest, init_support, sync_support},
};

pub(crate) fn run(cwd: &Path, _program: &str, args: GithubArgs) -> Result<Vec<String>> {
    match args.action {
        GithubAction::Init(args) => init_support(cwd, args.repo.as_deref(), args.enable_auto_sync),
        GithubAction::Sync(args) => sync(cwd, args),
    }
}

fn sync(cwd: &Path, args: GithubSyncArgs) -> Result<Vec<String>> {
    if args.target.is_some() && (args.all || args.changed) {
        bail!("Use either a specific target, `--all`, or `--changed`, not multiple sync scopes.");
    }
    if args.target.is_some() && args.status.is_some() {
        bail!("`--status` only applies to `--all` or `--changed` sync scopes.");
    }
    if args.all && args.changed {
        bail!("Use either `--all` or `--changed`, not both.");
    }

    let mode = if args.changed {
        SyncMode::Changed
    } else if args.all {
        SyncMode::All
    } else {
        SyncMode::Target
    };

    sync_support(
        cwd,
        SyncRequest {
            target: args.target,
            mode,
            status_filter: args.status,
            dry_run: args.dry_run,
        },
    )
}

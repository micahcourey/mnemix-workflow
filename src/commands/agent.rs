use std::path::Path;

use anyhow::Result;

use crate::{
    agent,
    cli::{AgentAction, AgentArgs},
    scaffold::find_repo_root,
};

pub(crate) fn run(cwd: &Path, _program: &str, args: AgentArgs) -> Result<Vec<String>> {
    let repo_root = find_repo_root(cwd)?;

    match args.action {
        AgentAction::Install(args) => agent::install(&repo_root, &args.tools, false),
        AgentAction::Update(args) => agent::install(&repo_root, &args.tools, true),
        AgentAction::Tools => agent::list_supported_tools(&repo_root),
    }
}

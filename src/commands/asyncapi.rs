use std::path::Path;

use anyhow::Result;

use crate::{
    cli::{AsyncApiArgs, ContractAction},
    contracts::{scaffold_asyncapi, validate_asyncapi},
    scaffold::{ensure_initialized, find_repo_root},
};

pub(crate) fn run(cwd: &Path, program: &str, args: AsyncApiArgs) -> Result<Vec<String>> {
    let repo_root = find_repo_root(cwd)?;
    ensure_initialized(&repo_root, program)?;

    match args.action {
        ContractAction::Init(args) => {
            let destination = scaffold_asyncapi(&repo_root, &args.workstream)?;
            Ok(vec![
                format!("Created AsyncAPI contract: {}", destination.display()),
                "Next step: edit the contract and run `mxw asyncapi validate <workstream-or-path>`."
                    .to_owned(),
            ])
        }
        ContractAction::Validate(args) => {
            let validated = validate_asyncapi(&repo_root, &args.target)?;
            Ok(vec![format!(
                "Validated AsyncAPI contract: {}",
                validated[0].display()
            )])
        }
    }
}

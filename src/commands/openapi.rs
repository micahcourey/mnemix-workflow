use std::path::Path;

use anyhow::Result;

use crate::{
    cli::{ContractAction, OpenApiArgs},
    contracts::{scaffold_openapi, validate_openapi},
    scaffold::{ensure_initialized, find_repo_root},
};

pub(crate) fn run(cwd: &Path, program: &str, args: OpenApiArgs) -> Result<Vec<String>> {
    let repo_root = find_repo_root(cwd)?;
    ensure_initialized(&repo_root, program)?;

    match args.action {
        ContractAction::Init(args) => {
            let destination = scaffold_openapi(&repo_root, &args.workstream)?;
            Ok(vec![
                format!("Created OpenAPI contract: {}", destination.display()),
                "Next step: edit the contract and run `mxw openapi validate <workstream-or-path>`."
                    .to_owned(),
            ])
        }
        ContractAction::Validate(args) => {
            let validated = validate_openapi(&repo_root, &args.target)?;
            Ok(vec![format!(
                "Validated OpenAPI contract: {}",
                validated[0].display()
            )])
        }
    }
}

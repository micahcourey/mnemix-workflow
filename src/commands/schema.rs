use std::path::Path;

use anyhow::Result;

use crate::{
    cli::{SchemaAction, SchemaArgs},
    contracts::{scaffold_schema, validate_schema},
    scaffold::{ensure_initialized, find_repo_root},
};

pub(crate) fn run(cwd: &Path, program: &str, args: SchemaArgs) -> Result<Vec<String>> {
    let repo_root = find_repo_root(cwd)?;
    ensure_initialized(&repo_root, program)?;

    match args.action {
        SchemaAction::New(args) => {
            let destination = scaffold_schema(&repo_root, &args.workstream, &args.name)?;
            Ok(vec![
                format!("Created JSON Schema: {}", destination.display()),
                "Next step: edit the schema and run `mxw schema validate <workstream-or-path>`."
                    .to_owned(),
            ])
        }
        SchemaAction::Validate(args) => {
            let validated = validate_schema(&repo_root, &args.target)?;
            let mut lines = vec![format!(
                "Validated {} JSON Schema file(s).",
                validated.len()
            )];
            for path in validated {
                lines.push(format!("- {}", path.display()));
            }
            Ok(lines)
        }
    }
}

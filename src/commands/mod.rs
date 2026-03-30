use std::path::Path;

use anyhow::Result;

use crate::cli::Command;

mod asyncapi;
mod github;
mod hooks;
mod init;
mod new;
mod openapi;
mod patch;
mod schema;
mod status;
mod ui;
mod validate;

pub(crate) fn execute(command: Command, program: &str, cwd: &Path) -> Result<Vec<String>> {
    match command {
        Command::Init => init::run(cwd, program),
        Command::New(args) => new::run(cwd, program, &args.name),
        Command::Openapi(args) => openapi::run(cwd, program, args),
        Command::Asyncapi(args) => asyncapi::run(cwd, program, args),
        Command::Schema(args) => schema::run(cwd, program, args),
        Command::Ui => ui::run(cwd, program),
        Command::Patch(args) => patch::run(cwd, program, args),
        Command::Status(args) => status::run(cwd, program, args),
        Command::Hooks(args) => hooks::run(cwd, program, args),
        Command::Github(args) => github::run(cwd, program, args),
        Command::Validate(args) => validate::run(cwd, program, args),
    }
}

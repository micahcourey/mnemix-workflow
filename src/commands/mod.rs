use std::path::Path;

use anyhow::Result;

use crate::cli::Command;

mod init;
mod new;
mod patch;
mod status;
mod ui;

pub(crate) fn execute(command: Command, program: &str, cwd: &Path) -> Result<Vec<String>> {
    match command {
        Command::Init => init::run(cwd, program),
        Command::New(args) => new::run(cwd, program, &args.name),
        Command::Ui => ui::run(cwd, program),
        Command::Patch(args) => patch::run(cwd, program, args),
        Command::Status(args) => status::run(cwd, program, args),
    }
}

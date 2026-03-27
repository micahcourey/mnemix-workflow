use std::path::Path;

use anyhow::Result;

use crate::cli::Command;

mod init;
mod new;

pub(crate) fn execute(command: Command, program: &str, cwd: &Path) -> Result<Vec<String>> {
    match command {
        Command::Init => init::run(cwd, program),
        Command::New(args) => new::run(cwd, program, &args.name),
    }
}

//! Shared CLI runtime for Mnemix Workflow.

mod cli;
mod commands;
mod contracts;
mod hooks;
mod scaffold;
mod status;
mod tui;

use std::{path::Path, process::ExitCode};

use clap::Parser;

use crate::cli::Cli;

fn program_name() -> String {
    std::env::args()
        .next()
        .as_deref()
        .and_then(|value| Path::new(value).file_name())
        .and_then(|value| value.to_str())
        .unwrap_or("mnemix-workflow")
        .to_owned()
}

/// Run the CLI using the current process arguments and working directory.
pub fn run() -> ExitCode {
    let cli = Cli::parse();
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(error) => {
            eprintln!("Failed to determine the current working directory: {error}");
            return ExitCode::FAILURE;
        }
    };

    match commands::execute(cli.command, &program_name(), &cwd) {
        Ok(lines) => {
            for line in lines {
                println!("{line}");
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

/// Run the interactive TUI directly, as used by the `mnx` shortcut binary.
pub fn run_ui_shortcut() -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(error) => {
            eprintln!("Failed to determine the current working directory: {error}");
            return ExitCode::FAILURE;
        }
    };

    match commands::execute(crate::cli::Command::Ui, "mnx", &cwd) {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

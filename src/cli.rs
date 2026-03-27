use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Lightweight workflow CLI for planning with AI assistance"
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    /// Initialize the minimum workflow structure in the current repository
    Init,
    /// Create a new workstream in an initialized repository
    New(NewArgs),
}

#[derive(Args, Debug)]
pub(crate) struct NewArgs {
    /// Human-readable workstream name
    pub(crate) name: String,
}

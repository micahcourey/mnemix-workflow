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
    /// Open the interactive terminal UI
    Ui,
    /// Create or inspect lightweight tracked patches
    Patch(PatchArgs),
    /// Read or update workstream status metadata
    Status(StatusArgs),
}

#[derive(Args, Debug)]
pub(crate) struct NewArgs {
    /// Human-readable workstream name
    pub(crate) name: String,
}

#[derive(Args, Debug)]
pub(crate) struct PatchArgs {
    #[command(subcommand)]
    pub(crate) action: PatchAction,
}

#[derive(Subcommand, Debug)]
pub(crate) enum PatchAction {
    /// Create a new patch in an initialized repository
    New(PatchNewArgs),
    /// Read or update patch status metadata
    Status(PatchStatusArgs),
}

#[derive(Args, Debug)]
pub(crate) struct PatchNewArgs {
    /// Human-readable patch name
    pub(crate) name: String,
}

#[derive(Args, Debug)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct PatchStatusArgs {
    /// Patch id or file name to inspect
    pub(crate) patch: Option<String>,
    #[command(subcommand)]
    pub(crate) action: Option<StatusAction>,
}

#[derive(Args, Debug)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct StatusArgs {
    /// Workstream id or folder name to inspect
    pub(crate) workstream: Option<String>,
    #[command(subcommand)]
    pub(crate) action: Option<StatusAction>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum StatusAction {
    /// Update workstream status metadata
    Set(StatusSetArgs),
    /// List workstreams, optionally filtered by status
    List(StatusListArgs),
}

#[derive(Args, Debug)]
pub(crate) struct StatusSetArgs {
    /// Workstream id or folder name to update
    pub(crate) workstream: String,
    /// New status value
    pub(crate) status: String,
    /// Optional replacement summary
    #[arg(long)]
    pub(crate) summary: Option<String>,
    /// Related pull request number; repeat to set multiple PRs
    #[arg(long = "pr")]
    pub(crate) prs: Vec<u64>,
    /// Remove existing linked pull request numbers
    #[arg(long)]
    pub(crate) clear_prs: bool,
}

#[derive(Args, Debug)]
pub(crate) struct StatusListArgs {
    /// Optional status value to filter by
    #[arg(long)]
    pub(crate) status: Option<String>,
}

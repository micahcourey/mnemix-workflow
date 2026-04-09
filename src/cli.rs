use clap::{Args, Parser, Subcommand, ValueEnum};

const AFTER_HELP: &str = "\
Install:
  pipx install mnemix-workflow
  pip install mnemix-workflow

Common commands:
  mxw init
  mxw new \"feature name\"
  mxw patch new \"small fix\"
  mxw agent install
  mxw validate
  mxw hooks install
  mnx
";

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Lightweight workflow CLI for planning with AI assistance",
    after_help = AFTER_HELP
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    /// Initialize the minimum workflow structure in the current repository
    Init,
    /// Install or refresh assistant slash-command integrations
    Agent(AgentArgs),
    /// Create a new workstream in an initialized repository
    New(NewArgs),
    /// Scaffold or validate OpenAPI contracts
    Openapi(OpenApiArgs),
    /// Scaffold or validate AsyncAPI contracts
    Asyncapi(AsyncApiArgs),
    /// Scaffold or validate JSON Schema artifacts
    Schema(SchemaArgs),
    /// Open the interactive terminal UI
    Ui,
    /// Create or inspect lightweight tracked patches
    Patch(PatchArgs),
    /// Read or update workstream status metadata
    Status(StatusArgs),
    /// Install bundled git hooks for status reminders and metadata refreshes
    Hooks(HooksArgs),
    /// Configure and sync optional GitHub issue mirrors
    Github(GithubArgs),
    /// Run umbrella validation across tracked workflow artifacts
    Validate(ValidateArgs),
}

#[derive(Args, Debug)]
pub(crate) struct AgentArgs {
    #[command(subcommand)]
    pub(crate) action: AgentAction,
}

#[derive(Subcommand, Debug)]
pub(crate) enum AgentAction {
    /// Install assistant slash-command files into the current repository
    Install(AgentInstallArgs),
    /// Refresh assistant slash-command files in the current repository
    Update(AgentInstallArgs),
    /// List supported assistant integrations
    Tools,
}

#[derive(Args, Debug)]
pub(crate) struct AgentInstallArgs {
    /// Assistant integration to configure; repeat to select multiple tools
    #[arg(long = "tool", value_enum)]
    pub(crate) tools: Vec<AssistantTool>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub(crate) enum AssistantTool {
    Claude,
    Cursor,
}

impl AssistantTool {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Claude => "claude",
            Self::Cursor => "cursor",
        }
    }

    pub(crate) fn display_name(self) -> &'static str {
        match self {
            Self::Claude => "Claude Code",
            Self::Cursor => "Cursor",
        }
    }
}

#[derive(Args, Debug)]
pub(crate) struct NewArgs {
    /// Human-readable workstream name
    pub(crate) name: String,
}

#[derive(Args, Debug)]
pub(crate) struct OpenApiArgs {
    #[command(subcommand)]
    pub(crate) action: ContractAction,
}

#[derive(Args, Debug)]
pub(crate) struct AsyncApiArgs {
    #[command(subcommand)]
    pub(crate) action: ContractAction,
}

#[derive(Subcommand, Debug)]
pub(crate) enum ContractAction {
    /// Scaffold the standard contract artifact for a workstream
    Init(ContractInitArgs),
    /// Validate a contract file or the default contract artifact for a workstream
    Validate(ContractValidateArgs),
}

#[derive(Args, Debug)]
pub(crate) struct ContractInitArgs {
    /// Workstream id or folder name
    pub(crate) workstream: String,
}

#[derive(Args, Debug)]
pub(crate) struct ContractValidateArgs {
    /// Workstream id, folder name, or explicit contract file path
    pub(crate) target: String,
}

#[derive(Args, Debug)]
pub(crate) struct SchemaArgs {
    #[command(subcommand)]
    pub(crate) action: SchemaAction,
}

#[derive(Subcommand, Debug)]
pub(crate) enum SchemaAction {
    /// Create a new JSON Schema artifact for a workstream
    New(SchemaNewArgs),
    /// Validate one schema file or all schemas for a workstream
    Validate(ContractValidateArgs),
}

#[derive(Args, Debug)]
pub(crate) struct SchemaNewArgs {
    /// Workstream id or folder name
    pub(crate) workstream: String,
    /// Human-readable schema name
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

#[derive(Args, Debug)]
pub(crate) struct HooksArgs {
    #[command(subcommand)]
    pub(crate) action: HooksAction,
}

#[derive(Subcommand, Debug)]
pub(crate) enum HooksAction {
    /// Install the bundled git hooks into the current repository
    Install(HooksInstallArgs),
}

#[derive(Args, Debug)]
pub(crate) struct HooksInstallArgs {
    /// Overwrite existing hook files when they differ
    #[arg(long)]
    pub(crate) force: bool,
}

#[derive(Args, Debug)]
pub(crate) struct GithubArgs {
    #[command(subcommand)]
    pub(crate) action: GithubAction,
}

#[derive(Subcommand, Debug)]
pub(crate) enum GithubAction {
    /// Initialize optional GitHub issue support for this repository
    Init(GithubInitArgs),
    /// Create or update mirrored GitHub issues from repo artifacts
    Sync(GithubSyncArgs),
}

#[derive(Args, Debug)]
pub(crate) struct GithubInitArgs {
    /// Explicit GitHub repository in owner/name format
    #[arg(long)]
    pub(crate) repo: Option<String>,
    /// Enable the bundled GitHub Actions auto-sync workflow
    #[arg(long)]
    pub(crate) enable_auto_sync: bool,
}

#[derive(Args, Debug)]
#[command(
    group = clap::ArgGroup::new("scope")
        .args(["target", "all", "changed"])
        .multiple(false)
)]
pub(crate) struct GithubSyncArgs {
    /// Workstream or patch reference to sync
    pub(crate) target: Option<String>,
    /// Sync all tracked workstreams and patches
    #[arg(long)]
    pub(crate) all: bool,
    /// Sync only tracked items changed in the current git diff range
    #[arg(long)]
    pub(crate) changed: bool,
    /// Optional status filter
    #[arg(long)]
    pub(crate) status: Option<String>,
    /// Print planned operations without mutating GitHub or repo metadata
    #[arg(long)]
    pub(crate) dry_run: bool,
}

#[derive(Args, Debug)]
pub(crate) struct ValidateArgs {
    /// Optional workstream or patch reference to validate instead of the whole repository
    pub(crate) target: Option<String>,
}

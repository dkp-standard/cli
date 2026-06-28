use clap::{Parser, Subcommand};
use dkp_core::config::DkpConfig;

use crate::output::OutputFormat;

/// Global flags extracted from `Cli` and passed into every command handler.
/// Separating this from `Cli` avoids borrow-after-partial-move in the dispatch match.
#[allow(dead_code)]
pub struct CmdCtx {
    pub output: OutputFormat,
    pub quiet: bool,
    pub verbose: bool,
    pub audience: Option<String>,
    pub config: DkpConfig,
}

/// dkp — Domain Knowledge Pack management CLI
#[derive(Parser, Debug)]
#[command(
    name = "dkp",
    version,
    about = "Manage, inspect, search, and deploy Domain Knowledge Packs",
    propagate_version = true
)]
pub struct Cli {
    /// Output format
    #[arg(long, global = true, default_value = "plain", value_enum)]
    pub output: OutputFormat,

    /// Suppress informational output; print only results
    #[arg(long, short = 'q', global = true)]
    pub quiet: bool,

    /// Print debug info (schema paths, provider calls, etc.)
    #[arg(long, short = 'v', global = true)]
    pub verbose: bool,

    /// Filter content to assets tagged for a specific audience profile
    #[arg(long, global = true, value_name = "ID")]
    pub audience: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    /// Extract a `CmdCtx` from the parsed CLI, consuming the global flags.
    /// Called before matching on `command` to avoid partial-move issues.
    pub fn ctx(&self) -> CmdCtx {
        CmdCtx {
            output: self.output,
            quiet: self.quiet,
            verbose: self.verbose,
            audience: self.audience.clone(),
            config: DkpConfig::load(),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    // ── Producer: Pack authoring ────────────────────────────────────────────
    /// Scaffold a new DKP pack directory with all required files
    Init(crate::cmd::init::InitArgs),

    // ── Phase 1: Offline ────────────────────────────────────────────────────
    /// Print a summary of a pack (name, version, asset counts, compliance)
    Info(crate::cmd::info::InfoArgs),

    /// List all packs under a root directory
    List(crate::cmd::list::ListArgs),

    /// Run schema and compliance checks; exit non-zero on failure
    Validate(crate::cmd::validate::ValidateArgs),

    /// Full-text BM25 search across machine assets (or registry with --registry)
    Search(crate::cmd::search::SearchArgs),

    /// Retrieve a specific asset or all assets of a type
    Get(crate::cmd::get::GetArgs),

    /// Print a ready-to-inject LLM context block
    Inject(crate::cmd::inject::InjectArgs),

    /// Convert machine assets to another format (okf, langchain, llamaindex, …)
    Export(crate::cmd::export::ExportArgs),

    /// OKF-specific operations (export, validate, stats, links, browse)
    Okf(crate::cmd::okf::OkfArgs),

    /// Retrieve the top-N most relevant retrieval chunks for a query
    Chunk(crate::cmd::chunk::ChunkArgs),

    // ── Phase 2: LLM-assisted ───────────────────────────────────────────────
    /// Run eval set against baseline and grounded prompts; print delta report
    Eval(crate::cmd::eval::EvalArgs),

    /// Interactive grounded prompt REPL for testing a pack
    Prompt(crate::cmd::prompt::PromptArgs),

    /// Compare two pack versions and report what changed
    Diff(crate::cmd::diff::DiffArgs),

    // ── Phase 3: Producer tooling ───────────────────────────────────────────
    /// Package a pack into a versioned archive with checksums.json
    Build(crate::cmd::build::BuildArgs),

    /// Pre-release compliance checklist (runs all gates, checks human fields)
    #[command(name = "release-check")]
    ReleaseCheck(crate::cmd::release_check::ReleaseCheckArgs),

    /// Source and rights log operations
    Rights(crate::cmd::rights::RightsArgs),

    // ── Phase 4: MCP + TUI ──────────────────────────────────────────────────
    /// Generate or regenerate machine/mcp_manifest.json
    #[command(name = "mcp-manifest")]
    McpManifest(crate::cmd::mcp_manifest::McpManifestArgs),

    /// Start the pack as an MCP server (requires --features mcp)
    #[cfg(feature = "mcp")]
    Serve(crate::cmd::serve::ServeArgs),

    /// Interactive TUI browser (requires --features tui)
    #[cfg(feature = "tui")]
    Tui(crate::cmd::tui::TuiArgs),

    /// Browse a pack in a local web UI (requires --features webui)
    #[cfg(feature = "webui")]
    Webui(crate::cmd::webui::WebuiArgs),

    // ── Procedures ─────────────────────────────────────────────────────────
    /// Invoke a WASM/WASI procedure from machine/procedures/
    Run(crate::cmd::run::RunArgs),

    /// List, validate, and scaffold executable procedures
    Procedures(crate::cmd::procedures::ProceduresArgs),

    // ── Phase 5: Spec-derived ───────────────────────────────────────────────
    /// Inspect and validate knowledge_graph.json
    Graph(crate::cmd::graph::GraphArgs),

    /// Inspect and validate cross_refs.json
    #[command(name = "cross-refs")]
    CrossRefs(crate::cmd::cross_refs::CrossRefsArgs),

    /// Manage and validate the skills/ layer
    Skills(crate::cmd::skills::SkillsArgs),

    /// Manage and validate the l10n/ localization layer
    L10n(crate::cmd::l10n::L10nArgs),

    // ── Generation ─────────────────────────────────────────────────────────
    /// Scaffold + LLM-generate a complete pack in one command
    New(crate::cmd::new::NewArgs),

    /// Run (or re-run) LLM generation on an existing pack
    Generate(crate::cmd::generate::GenerateArgs),

    /// Failure-aware chunk regeneration using eval results
    Fix(crate::cmd::fix::FixArgs),

    /// Generate evidence drafts for manual review gates
    Review(crate::cmd::review::ReviewArgs),

    // ── Phase 6: Registry ───────────────────────────────────────────────────
    /// Generate an Ed25519 keypair for signing packs
    Keygen(crate::cmd::keygen::KeygenArgs),

    /// Sign a built archive with an Ed25519 private key
    Sign(crate::cmd::sign::SignArgs),

    /// Install a pack from the registry
    Install(crate::cmd::registry::install::InstallArgs),

    /// Remove an installed pack
    Uninstall(crate::cmd::registry::uninstall::UninstallArgs),

    /// Re-resolve and update installed packs to satisfy lock-file constraints
    Update(crate::cmd::registry::update::UpdateArgs),

    /// Publish a built and signed pack to the registry
    Publish(crate::cmd::registry::publish::PublishArgs),

    /// Mark a published version as yanked
    Yank(crate::cmd::registry::yank::YankArgs),

    /// Registry account and pack management (login, logout, keys, access)
    Registry(crate::cmd::registry::account::RegistryArgs),
}

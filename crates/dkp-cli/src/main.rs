use anyhow::Result;
use clap::Parser;
use dkp::cli::{Cli, Commands};
use dkp::cmd;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    run(cli).await
}

async fn run(cli: Cli) -> Result<()> {
    let ctx = cli.ctx();
    match cli.command {
        Commands::Init(mut args) => {
            args.stubs = true;
            cmd::init::run(args, &ctx).await
        }
        // Phase 1
        Commands::Info(args) => cmd::info::run(args, &ctx).await,
        Commands::List(args) => cmd::list::run(args, &ctx).await,
        Commands::Validate(args) => cmd::validate::run(args, &ctx).await,
        Commands::Search(args) => cmd::search::run(args, &ctx).await,
        Commands::Get(args) => cmd::get::run(args, &ctx).await,
        Commands::Inject(args) => cmd::inject::run(args, &ctx).await,
        Commands::Export(args) => cmd::export::run(args, &ctx).await,
        Commands::Okf(args) => cmd::okf::run(args, &ctx).await,
        Commands::Chunk(args) => cmd::chunk::run(args, &ctx).await,
        // Phase 2
        Commands::Eval(args) => cmd::eval::run(args, &ctx).await,
        Commands::Prompt(args) => cmd::prompt::run(args, &ctx).await,
        Commands::Diff(args) => cmd::diff::run(args, &ctx).await,
        // Phase 3
        Commands::Build(args) => cmd::build::run(args, &ctx).await,
        Commands::ReleaseCheck(args) => cmd::release_check::run(args, &ctx).await,
        Commands::Rights(args) => cmd::rights::run(args, &ctx).await,
        // Phase 4
        Commands::McpManifest(args) => cmd::mcp_manifest::run(args, &ctx).await,
        #[cfg(feature = "mcp")]
        Commands::Serve(args) => cmd::serve::run(args, &ctx).await,
        #[cfg(feature = "tui")]
        Commands::Tui(args) => cmd::tui::run(args, &ctx).await,
        #[cfg(feature = "webui")]
        Commands::Webui(args) => cmd::webui::run(args, &ctx).await,
        // Procedures
        Commands::Run(args) => cmd::run::run(args, &ctx).await,
        Commands::Procedures(args) => cmd::procedures::run(args, &ctx).await,
        // Phase 5
        Commands::Graph(args) => cmd::graph::run(args, &ctx).await,
        Commands::CrossRefs(args) => cmd::cross_refs::run(args, &ctx).await,
        Commands::Skills(args) => cmd::skills::run(args, &ctx).await,
        Commands::L10n(args) => cmd::l10n::run(args, &ctx).await,
        // Generation
        Commands::New(args) => cmd::new::run(args, &ctx).await,
        Commands::Generate(args) => cmd::generate::run(args, &ctx).await,
        Commands::Fix(args) => cmd::fix::run(args, &ctx).await,
        Commands::Review(args) => cmd::review::run(args, &ctx).await,
        // Phase 6
        Commands::Keygen(args) => cmd::keygen::run(args, &ctx).await,
        Commands::Sign(args) => cmd::sign::run(args, &ctx).await,
        Commands::Install(args) => cmd::registry::install::run(args, &ctx).await,
        Commands::Uninstall(args) => cmd::registry::uninstall::run(args, &ctx).await,
        Commands::Update(args) => cmd::registry::update::run(args, &ctx).await,
        Commands::Publish(args) => cmd::registry::publish::run(args, &ctx).await,
        Commands::Yank(args) => cmd::registry::yank::run(args, &ctx).await,
        Commands::Registry(args) => cmd::registry::account::run(args, &ctx).await,
    }
}

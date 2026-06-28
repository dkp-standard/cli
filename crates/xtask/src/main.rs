use std::{env, fs, path::PathBuf};

use anyhow::{Context, Result};
use clap::CommandFactory;
use clap_complete::{generate_to, Shell};
use dkp::Cli;

fn main() -> Result<()> {
    let task = env::args().nth(1).unwrap_or_else(|| "docs".to_string());
    match task.as_str() {
        "docs" => generate_docs(),
        "sync-readme" => sync_readme(),
        other => anyhow::bail!("unknown xtask: {other}. Available: docs, sync-readme"),
    }
}

fn generate_docs() -> Result<()> {
    let workspace_root = workspace_root();
    let docs_root = workspace_root.join("docs");

    let dist_man = docs_root.join("man");
    let dist_completions = docs_root.join("completions");
    let dist_reference = docs_root.join("src").join("reference");

    fs::create_dir_all(&dist_man).context("create docs/man")?;
    fs::create_dir_all(&dist_completions).context("create docs/completions")?;
    fs::create_dir_all(&dist_reference).context("create docs/src/reference")?;

    let mut cmd = Cli::command();
    cmd.set_bin_name("dkp");

    // 1. Man pages — clap_mangen::generate_to handles subcommands recursively
    println!("Generating man pages → docs/man/");
    clap_mangen::generate_to(cmd.clone(), &dist_man).context("generate man pages")?;

    // 2. Shell completions
    println!("Generating shell completions → docs/completions/");
    for shell in [Shell::Bash, Shell::Zsh, Shell::Fish, Shell::PowerShell] {
        let path = generate_to(shell, &mut cmd.clone(), "dkp", &dist_completions)
            .context("generate completions")?;
        println!("  {}", path.display());
    }

    // 3. Markdown reference (overwrites the committed placeholder)
    println!("Generating CLI reference → docs/src/reference/cli-reference.md");
    let md = clap_markdown::help_markdown::<Cli>();
    fs::write(dist_reference.join("cli-reference.md"), md).context("write cli-reference.md")?;

    sync_readme()?;
    println!("Done. Run `cd docs && mdbook build` for HTML output.");
    Ok(())
}

fn sync_readme() -> Result<()> {
    let cli_root = workspace_root();

    // Prefer the monorepo sibling docs/README.md when present (local dev).
    // Fall back to cli/README.md when checked out standalone (CI).
    let monorepo_src = cli_root
        .parent()
        .context("cli/ has no parent")?
        .join("docs")
        .join("README.md");
    let local_src = cli_root.join("README.md");

    let src = if monorepo_src.exists() {
        monorepo_src
    } else {
        local_src
    };

    let content = fs::read(&src).with_context(|| format!("read {}", src.display()))?;

    let targets = [
        cli_root.join("README.md"),
        cli_root.join("crates/dkp-cli/README.md"),
        cli_root.join("crates/dkp-core/README.md"),
        cli_root.join("crates/dkp-gen-core/README.md"),
    ];

    for dest in &targets {
        // Skip writing the source file back to itself.
        if dest == &src {
            continue;
        }
        fs::write(dest, &content).with_context(|| format!("write {}", dest.display()))?;
        println!("Wrote {}", dest.display());
    }

    Ok(())
}

fn workspace_root() -> PathBuf {
    // CARGO_MANIFEST_DIR is crates/xtask/ — go up two levels to cli/
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

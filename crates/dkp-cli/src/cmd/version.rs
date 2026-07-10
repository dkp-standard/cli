use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};
use clap::Args;

use dkp_core::pack::version::{VersionBump, bump_manifest_version};

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct VersionArgs {
    /// New version: major|minor|patch|premajor|preminor|prepatch|prerelease|<x.y.z>
    pub bump: String,

    /// Path to the DKP pack directory (defaults to the current directory)
    pub pack: Option<PathBuf>,

    /// Skip git commit/tag even if the pack is inside a git repository
    #[arg(long)]
    pub no_git_tag_version: bool,

    /// Allow bumping to the same version as the current one
    #[arg(long)]
    pub allow_same_version: bool,

    /// Commit message template; %s is replaced with the new version
    #[arg(long, short = 'm', default_value = "chore: version %s")]
    pub message: String,
}

pub async fn run(args: VersionArgs, ctx: &CmdCtx) -> Result<()> {
    let pack_root = args.pack.clone().unwrap_or_else(|| PathBuf::from("."));
    let bump: VersionBump = args
        .bump
        .parse()
        .map_err(|e: dkp_core::error::DkpError| anyhow::anyhow!(e))?;

    let is_major_bump = matches!(bump, VersionBump::Major | VersionBump::Premajor);

    let git = if args.no_git_tag_version {
        None
    } else {
        GitContext::detect(&pack_root)?
    };

    if let Some(git) = &git {
        git.ensure_clean(&pack_root)?;
    }

    let (old, new) = bump_manifest_version(&pack_root, &bump, args.allow_same_version)?;

    if let Some(git) = &git {
        let message = args.message.replace("%s", &new.to_string());
        git.commit_and_tag(&pack_root, &new.to_string(), &message)?;
    }

    if !ctx.quiet {
        println!("{new}");
        if is_major_bump {
            eprintln!(
                "Note: MAJOR bundle version bumps (per DKP spec §8.2) should reflect domain \
                 scope changes, structural restructuring, audience changes, or source-rights-\
                 driven chunk removal — not merely large edits. This does not change the \
                 pack's 'spec' field."
            );
        }
        let _ = old; // retained for potential future verbose diff output
    }

    Ok(())
}

/// Best-effort git integration: git is shelled out to (matching npm's own
/// approach), and any absence of the `git` binary or of a repo is treated as
/// "skip git steps," never a hard failure.
struct GitContext;

impl GitContext {
    fn detect(pack_root: &Path) -> Result<Option<Self>> {
        match Command::new("git")
            .args(["rev-parse", "--is-inside-work-tree"])
            .current_dir(pack_root)
            .output()
        {
            Ok(output) if output.status.success() => Ok(Some(GitContext)),
            Ok(_) => Ok(None),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                eprintln!("Note: git not found on PATH; skipping git commit/tag.");
                Ok(None)
            }
            Err(e) => Err(e).context("failed to invoke git"),
        }
    }

    fn ensure_clean(&self, pack_root: &Path) -> Result<()> {
        let output = Command::new("git")
            .args(["status", "--porcelain", "--", "."])
            .current_dir(pack_root)
            .output()
            .context("failed to run `git status`")?;

        if !output.stdout.is_empty() {
            anyhow::bail!(
                "pack directory has uncommitted changes; commit or stash them before running \
                 `dkp version` (or pass --no-git-tag-version to skip git entirely)"
            );
        }
        Ok(())
    }

    fn commit_and_tag(&self, pack_root: &Path, new_version: &str, message: &str) -> Result<()> {
        run_git(pack_root, &["add", "--", "manifest.json"])?;
        run_git(pack_root, &["commit", "-m", message])?;
        run_git(
            pack_root,
            &["tag", "-a", &format!("v{new_version}"), "-m", &format!("v{new_version}")],
        )?;
        Ok(())
    }
}

fn run_git(pack_root: &Path, args: &[&str]) -> Result<()> {
    let output = Command::new("git")
        .args(args)
        .current_dir(pack_root)
        .output()
        .with_context(|| format!("failed to run `git {}`", args.join(" ")))?;

    if !output.status.success() {
        anyhow::bail!(
            "`git {}` failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

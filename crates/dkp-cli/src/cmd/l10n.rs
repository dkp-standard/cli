use anyhow::Result;
use clap::{Args, Subcommand};
use std::path::PathBuf;

use comfy_table::{presets::UTF8_FULL, Table};
use dkp_core::Pack;

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct L10nArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,

    #[command(subcommand)]
    pub command: L10nCommands,
}

#[derive(Subcommand, Debug)]
pub enum L10nCommands {
    /// List available locales
    List,
    /// Check locale content doesn't contradict the base pack
    Validate,
    /// Export a locale-specific bundle
    Export {
        locale: String,
        #[arg(long, value_name = "DIR")]
        out: Option<PathBuf>,
    },
}

/// Return (locale_tag, locale_dir) pairs sorted alphabetically.
fn list_locales(l10n_dir: &std::path::Path) -> anyhow::Result<Vec<(String, PathBuf)>> {
    let mut locales = Vec::new();
    for entry in std::fs::read_dir(l10n_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let tag = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
            locales.push((tag, path));
        }
    }
    locales.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(locales)
}

/// Count files recursively under a directory.
fn count_files(dir: &std::path::Path) -> usize {
    walkdir(dir, &mut |_| {}).unwrap_or(0)
}

fn walkdir(dir: &std::path::Path, _cb: &mut dyn FnMut(&std::path::Path)) -> anyhow::Result<usize> {
    let mut count = 0;
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let p = entry.path();
        if p.is_dir() {
            count += walkdir(&p, _cb)?;
        } else {
            count += 1;
        }
    }
    Ok(count)
}

/// Check that a locale's concept files carry `dkp_locale` frontmatter.
fn validate_locale(locale_dir: &std::path::Path, tag: &str) -> Vec<String> {
    let mut errors = Vec::new();
    check_dir(locale_dir, tag, &mut errors);
    errors
}

#[allow(clippy::only_used_in_recursion)]
fn check_dir(dir: &std::path::Path, tag: &str, errors: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            check_dir(&p, tag, errors);
        } else if p.extension().is_some_and(|e| e == "md") {
            let content = match std::fs::read_to_string(&p) {
                Ok(c) => c,
                Err(_) => continue,
            };
            if !content.starts_with("---") {
                continue; // not a frontmatter file
            }
            let has_locale_field = content
                .strip_prefix("---")
                .and_then(|s| s.split_once("---"))
                .map(|(yaml, _)| yaml.contains("dkp_locale"))
                .unwrap_or(false);
            if !has_locale_field {
                errors.push(format!(
                    "{}: missing 'dkp_locale' frontmatter field",
                    p.display()
                ));
            }
        }
    }
}

/// Copy locale subtree to output directory, merging with base pack.
fn export_locale(pack: &Pack, locale: &str, out: &std::path::Path) -> anyhow::Result<()> {
    let locale_dir = pack.l10n_dir().join(locale);
    if !locale_dir.exists() {
        anyhow::bail!("locale '{}' not found in l10n/", locale);
    }

    // Start with the full base pack
    copy_dir_all(&pack.root, out)?;

    // Overlay locale-specific files
    overlay_dir(&locale_dir, out)?;

    Ok(())
}

fn copy_dir_all(src: &std::path::Path, dst: &std::path::Path) -> anyhow::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

fn overlay_dir(src: &std::path::Path, dst: &std::path::Path) -> anyhow::Result<()> {
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            overlay_dir(&src_path, &dst_path)?;
        } else {
            if let Some(parent) = dst_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

pub async fn run(args: L10nArgs, _cli: &CmdCtx) -> Result<()> {
    let pack = Pack::open(&args.pack)?;

    if !pack.has_l10n() {
        println!("l10n/ directory not present in this pack.");
        return Ok(());
    }

    let l10n_dir = pack.l10n_dir();

    match args.command {
        L10nCommands::List => {
            let locales = list_locales(&l10n_dir)?;
            let base = pack.manifest.base_locale.as_deref().unwrap_or("en-US");

            let mut table = Table::new();
            table.load_preset(UTF8_FULL);
            table.set_header(["Locale", "Base?", "Files"]);
            for (tag, dir) in &locales {
                let is_base = if tag == base { "yes" } else { "" };
                let file_count = count_files(dir).to_string();
                table.add_row([tag.as_str(), is_base, &file_count]);
            }
            println!("{table}");
            println!("\n{} locale(s)  (base: {base})", locales.len());
        }

        L10nCommands::Validate => {
            let locales = list_locales(&l10n_dir)?;
            let mut all_errors: Vec<String> = Vec::new();

            for (tag, dir) in &locales {
                let errs = validate_locale(dir, tag);
                all_errors.extend(errs);
            }

            if all_errors.is_empty() {
                println!(
                    "[PASS] l10n — {} locale(s), all concept files carry dkp_locale",
                    locales.len()
                );
            } else {
                println!("[FAIL] l10n — {} error(s):", all_errors.len());
                for e in &all_errors {
                    println!("  {e}");
                }
                anyhow::bail!("l10n validation failed");
            }
        }

        L10nCommands::Export { locale, out } => {
            let out_dir = out.unwrap_or_else(|| {
                args.pack.parent().unwrap_or(&args.pack).join(format!(
                    "{}-{}-{}",
                    pack.manifest.name.replace(' ', "-").to_lowercase(),
                    locale,
                    pack.manifest.version
                ))
            });
            eprintln!("Exporting locale '{locale}' to {} ...", out_dir.display());
            export_locale(&pack, &locale, &out_dir)?;
            println!("Export complete → {}", out_dir.display());
        }
    }

    Ok(())
}

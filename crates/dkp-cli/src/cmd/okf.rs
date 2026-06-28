use anyhow::Result;
use clap::{Args, Subcommand};
use std::path::PathBuf;

use dkp_core::{okf::exporter::export_okf, okf::parser::parse_okf_dir, Pack};

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct OkfArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,

    #[command(subcommand)]
    pub command: OkfCommands,
}

#[derive(Subcommand, Debug)]
pub enum OkfCommands {
    /// Generate okf/ from machine/
    Export {
        #[arg(long, value_name = "DIR")]
        out: Option<PathBuf>,
    },
    /// Check OKF conformance (Gate 8)
    Validate,
    /// Print concept count by type
    Stats,
    /// Check cross-link integrity
    Links,
    /// Interactive terminal browser
    Browse,
}

pub async fn run(args: OkfArgs, _cli: &CmdCtx) -> Result<()> {
    let pack = Pack::open(&args.pack)?;

    match args.command {
        OkfCommands::Export { out } => {
            let out_dir = out.unwrap_or_else(|| args.pack.join("okf"));
            eprintln!("Exporting OKF bundle to {} ...", out_dir.display());
            let stats = export_okf(&pack, &out_dir)?;
            println!("OKF export complete → {}", out_dir.display());
            println!("  Terms:       {}", stats.terms_written);
            println!("  Rules:       {}", stats.rules_written);
            println!("  Constraints: {}", stats.constraints_written);
            println!("  Chunks:      {}", stats.chunks_written);
            println!("  Entities:    {}", stats.ontology_written);
        }

        OkfCommands::Validate => {
            let okf_dir = pack.okf_dir();
            if !okf_dir.exists() {
                println!("OKF layer not present — skipping Gate 8 checks.");
                return Ok(());
            }

            let concepts: Vec<_> = parse_okf_dir(&okf_dir)?
                .into_iter()
                .filter(|c| c.path.file_name().is_none_or(|n| n != "index.md"))
                .collect();
            let total = concepts.len();
            let mut errors: Vec<String> = Vec::new();

            for concept in &concepts {
                let path = concept.path.display().to_string();
                match concept.frontmatter.get("type") {
                    None => errors.push(format!("{}: missing 'type' field in frontmatter", path)),
                    Some(t) if t.as_str().is_none() => {
                        errors.push(format!("{}: 'type' field is not a string", path))
                    }
                    _ => {}
                }
                if concept.frontmatter.get("id").is_none() {
                    errors.push(format!("{}: missing 'id' field in frontmatter", path));
                }
            }

            let bundle_sig = okf_dir.join("bundle.sig");

            if errors.is_empty() {
                println!("[PASS] OKF validate — {total} concepts, all frontmatter valid");
                if bundle_sig.exists() {
                    println!("[INFO] bundle.sig present");
                } else {
                    println!("[WARN] bundle.sig not present");
                }
            } else {
                println!(
                    "[FAIL] OKF validate — {total} concepts, {} errors:",
                    errors.len()
                );
                for e in &errors {
                    println!("  {e}");
                }
                anyhow::bail!("OKF validation failed");
            }
        }

        OkfCommands::Stats => {
            let okf_dir = pack.okf_dir();
            if !okf_dir.exists() {
                println!("OKF layer not present.");
                return Ok(());
            }

            let concepts: Vec<_> = parse_okf_dir(&okf_dir)?
                .into_iter()
                .filter(|c| c.path.file_name().is_none_or(|n| n != "index.md"))
                .collect();
            let mut counts: std::collections::BTreeMap<String, usize> = Default::default();
            for c in &concepts {
                let type_name = c
                    .frontmatter
                    .get("type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                *counts.entry(type_name).or_insert(0) += 1;
            }

            println!("OKF Stats — {} concepts total\n", concepts.len());
            for (t, n) in &counts {
                println!("  {t:<16} {n}");
            }
        }

        OkfCommands::Links => {
            let okf_dir = pack.okf_dir();
            if !okf_dir.exists() {
                println!("OKF layer not present.");
                return Ok(());
            }

            let concepts: Vec<_> = parse_okf_dir(&okf_dir)?
                .into_iter()
                .filter(|c| c.path.file_name().is_none_or(|n| n != "index.md"))
                .collect();
            // Collect all concept IDs
            let known_ids: std::collections::HashSet<String> = concepts
                .iter()
                .filter_map(|c| {
                    c.frontmatter
                        .get("id")
                        .and_then(|v| v.as_str())
                        .map(String::from)
                })
                .collect();

            // Check markdown links in body for [[id]] style cross-links
            let link_re = regex_lite(&known_ids, &concepts);
            if link_re.is_empty() {
                println!("[PASS] No cross-links found (or all links resolve).");
            } else {
                println!("[WARN] Broken cross-links:");
                for broken in &link_re {
                    println!("  {broken}");
                }
            }
        }

        OkfCommands::Browse => {
            anyhow::bail!("dkp okf browse requires --features tui (not yet implemented)");
        }
    }

    Ok(())
}

fn regex_lite(
    known_ids: &std::collections::HashSet<String>,
    concepts: &[dkp_core::okf::parser::OkfConcept],
) -> Vec<String> {
    // Simple scan for [[id]] wiki-style links in body text
    let mut broken = Vec::new();
    for c in concepts {
        let body = &c.body;
        let mut rest = body.as_str();
        while let Some(start) = rest.find("[[") {
            rest = &rest[start + 2..];
            if let Some(end) = rest.find("]]") {
                let link_id = &rest[..end];
                if !known_ids.contains(link_id) {
                    broken.push(format!("{}: broken link [[{}]]", c.path.display(), link_id));
                }
                rest = &rest[end + 2..];
            } else {
                break;
            }
        }
    }
    broken
}

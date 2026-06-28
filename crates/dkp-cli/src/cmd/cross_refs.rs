use anyhow::Result;
use clap::{Args, Subcommand};
use std::collections::HashSet;
use std::path::PathBuf;

use comfy_table::{presets::UTF8_FULL, Table};
use dkp_core::{types::cross_refs::CrossRefsFile, Pack};

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct CrossRefsArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,

    #[command(subcommand)]
    pub command: CrossRefsCommands,
}

#[derive(Subcommand, Debug)]
pub enum CrossRefsCommands {
    /// List all declared pack dependencies
    List,
    /// Check local_id values resolve to concepts in this bundle (Gate 4)
    Validate,
}

pub async fn run(args: CrossRefsArgs, _cli: &CmdCtx) -> Result<()> {
    let pack = Pack::open(&args.pack)?;

    if !pack.has_cross_refs() {
        println!("cross_refs.json not present in this pack.");
        return Ok(());
    }

    let bytes = std::fs::read(pack.machine_file("cross_refs.json"))?;
    let cross_refs_file: CrossRefsFile = serde_json::from_slice(&bytes)?;
    let refs = &cross_refs_file.cross_refs;

    match args.command {
        CrossRefsCommands::List => {
            let mut table = Table::new();
            table.load_preset(UTF8_FULL);
            table.set_header([
                "Local ID",
                "Remote Pack",
                "Version",
                "Remote ID",
                "Relation",
            ]);
            for r in refs {
                table.add_row([
                    &r.local_id,
                    &r.pack_name,
                    &r.pack_version,
                    &r.remote_id,
                    &r.relation,
                ]);
            }
            println!("{table}");
            println!("\n{} cross-reference(s)", refs.len());
        }

        CrossRefsCommands::Validate => {
            let declared_deps: HashSet<&str> = pack
                .manifest
                .dependencies
                .iter()
                .map(|d| d.name.as_str())
                .collect();

            let mut errors: Vec<String> = Vec::new();

            for r in refs {
                if !declared_deps.contains(r.pack_name.as_str()) {
                    errors.push(format!(
                        "{}: remote_pack '{}' not listed in manifest.dependencies",
                        r.local_id, r.pack_name
                    ));
                }
            }

            // Collect all known local concept IDs from machine-layer assets
            let mut known_ids: HashSet<String> = HashSet::new();
            if let Ok(Some(gf)) = pack.load_glossary() {
                known_ids.extend(gf.terms.iter().map(|t| t.id.clone()));
            }
            if let Ok(Some(rf)) = pack.load_rules() {
                known_ids.extend(rf.rules.iter().map(|r| r.id.clone()));
            }
            if let Ok(Some(cf)) = pack.load_constraints() {
                for c in cf.all_constraints() {
                    known_ids.insert(c.id.clone());
                }
            }
            if let Ok(Some(of)) = pack.load_ontology() {
                known_ids.extend(of.entity_types.iter().map(|e| e.id.clone()));
            }
            if let Ok(chunks) = pack.load_chunks() {
                known_ids.extend(chunks.iter().map(|c| c.id.clone()));
            }

            for r in refs {
                if !known_ids.contains(&r.local_id) {
                    errors.push(format!(
                        "local_id '{}' does not resolve to any concept in this bundle",
                        r.local_id
                    ));
                }
            }

            if errors.is_empty() {
                println!(
                    "[PASS] cross_refs — {} ref(s), all local_ids resolve, all remote_packs declared",
                    refs.len()
                );
            } else {
                println!("[FAIL] cross_refs — {} error(s):", errors.len());
                for e in &errors {
                    println!("  {e}");
                }
                anyhow::bail!("cross_refs validation failed");
            }
        }
    }

    Ok(())
}

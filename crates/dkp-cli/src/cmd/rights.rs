use anyhow::Result;
use clap::{Args, Subcommand};
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

use comfy_table::{presets::UTF8_FULL, Cell, Color, Table};
use dkp_core::{
    types::evidence::{RightsRecord, SourceRecord},
    Pack,
};

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct RightsArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,

    #[command(subcommand)]
    pub command: RightsCommands,
}

#[derive(Subcommand, Debug)]
pub enum RightsCommands {
    /// Summary of sources and rights coverage
    Status,
    /// Flag entries with missing fields or expired rights
    Check,
    /// Interactive prompt to add a source entry
    AddSource,
    /// Formatted compliance report for human review
    Report,
}

fn load_sources(pack: &Pack) -> anyhow::Result<Vec<SourceRecord>> {
    let path = pack.evidence_file("sources.csv");
    if !path.exists() {
        return Ok(vec![]);
    }
    let mut rdr = csv::Reader::from_path(&path)?;
    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: SourceRecord = result?;
        records.push(record);
    }
    Ok(records)
}

fn load_rights(pack: &Pack) -> anyhow::Result<Vec<RightsRecord>> {
    let path = pack.evidence_file("rights_log.csv");
    if !path.exists() {
        return Ok(vec![]);
    }
    let mut rdr = csv::Reader::from_path(&path)?;
    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: RightsRecord = result?;
        records.push(record);
    }
    Ok(records)
}

fn is_expired(expiry: &str) -> bool {
    if expiry.eq_ignore_ascii_case("perpetual") || expiry.is_empty() {
        return false;
    }
    // Compare as string; ISO 8601 dates sort lexicographically
    let today = chrono_today();
    expiry < today.as_str()
}

fn chrono_today() -> String {
    // Use only std — format YYYY-MM-DD from SystemTime
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let days = secs / 86400;
    // Zeller / proleptic Gregorian
    let mut y = 1970i64;
    let mut remaining = days as i64;
    loop {
        let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
        let days_in_year = if leap { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        y += 1;
    }
    let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let month_days: &[i64] = if leap {
        &[31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        &[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut m = 1i64;
    for &md in month_days {
        if remaining < md {
            break;
        }
        remaining -= md;
        m += 1;
    }
    let d = remaining + 1;
    format!("{y:04}-{m:02}-{d:02}")
}

pub async fn run(args: RightsArgs, _cli: &CmdCtx) -> Result<()> {
    let pack = Pack::open(&args.pack)?;

    match args.command {
        RightsCommands::Status => {
            let sources = load_sources(&pack)?;
            let rights = load_rights(&pack)?;

            let rights_by_source: HashMap<&str, &RightsRecord> =
                rights.iter().map(|r| (r.source_id.as_str(), r)).collect();

            let covered = sources
                .iter()
                .filter(|s| rights_by_source.contains_key(s.id.as_str()))
                .count();

            println!("Evidence layer status");
            println!("  sources.csv      {} source(s)", sources.len());
            println!("  rights_log.csv   {} record(s)", rights.len());
            println!(
                "  Rights coverage  {covered}/{} source(s) have rights records",
                sources.len()
            );

            let expired: Vec<_> = rights
                .iter()
                .filter(|r| r.expiry_date.as_deref().map(is_expired).unwrap_or(false))
                .collect();
            if !expired.is_empty() {
                println!("\n  ⚠  {} expired rights record(s):", expired.len());
                for r in &expired {
                    println!(
                        "     {} (expired {})",
                        r.source_id,
                        r.expiry_date.as_deref().unwrap_or("?")
                    );
                }
            }
        }

        RightsCommands::Check => {
            let sources = load_sources(&pack)?;
            let rights = load_rights(&pack)?;
            let rights_by_source: HashMap<&str, &RightsRecord> =
                rights.iter().map(|r| (r.source_id.as_str(), r)).collect();

            let mut warnings: Vec<String> = Vec::new();

            for s in &sources {
                if s.url.is_empty() {
                    warnings.push(format!("source {}: missing url", s.id));
                }
                if s.license.is_empty() {
                    warnings.push(format!("source {}: missing license", s.id));
                }
                if !rights_by_source.contains_key(s.id.as_str()) {
                    warnings.push(format!("source {}: no rights_log entry", s.id));
                }
            }

            for r in &rights {
                if let Some(expiry) = &r.expiry_date {
                    if is_expired(expiry) {
                        warnings.push(format!(
                            "rights {} (source {}): expired on {expiry}",
                            r.rights_holder, r.source_id
                        ));
                    }
                }
            }

            if warnings.is_empty() {
                println!(
                    "[PASS] rights check — {} source(s), no issues",
                    sources.len()
                );
            } else {
                println!("[WARN] rights check — {} issue(s):", warnings.len());
                for w in &warnings {
                    println!("  {w}");
                }
            }
        }

        RightsCommands::AddSource => {
            let stdin = io::stdin();
            let mut stdout = io::stdout();

            macro_rules! prompt {
                ($label:expr) => {{
                    print!("{}: ", $label);
                    stdout.flush()?;
                    let mut line = String::new();
                    stdin.lock().read_line(&mut line)?;
                    line.trim().to_string()
                }};
            }

            let id = prompt!("Source ID (e.g. src-001)");
            let title = prompt!("Title");
            let url = prompt!("URL");
            let retrieved_date = prompt!("Retrieved date (YYYY-MM-DD)");
            let license = prompt!("License (SPDX or prose)");
            let notes = prompt!("Notes (optional)");

            let sources_path = pack.evidence_file("sources.csv");
            let write_header =
                !sources_path.exists() || std::fs::metadata(&sources_path)?.len() == 0;

            let file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&sources_path)?;
            let mut wtr = csv::WriterBuilder::new()
                .has_headers(write_header)
                .from_writer(file);

            if write_header {
                wtr.write_record(["id", "title", "url", "retrieved_date", "license", "notes"])?;
            }
            wtr.write_record([&id, &title, &url, &retrieved_date, &license, &notes])?;
            wtr.flush()?;

            println!("\nAdded source '{id}' to evidence/sources.csv");
        }

        RightsCommands::Report => {
            let sources = load_sources(&pack)?;
            let rights = load_rights(&pack)?;
            let rights_by_source: HashMap<&str, &RightsRecord> =
                rights.iter().map(|r| (r.source_id.as_str(), r)).collect();

            println!("# Rights & Provenance Report\n");
            println!("Pack:    {} v{}", pack.manifest.name, pack.manifest.version);
            println!("Sources: {}", sources.len());
            println!();

            let mut table = Table::new();
            table.load_preset(UTF8_FULL);
            table.set_header([
                "Source ID",
                "Title",
                "License",
                "Rights Holder",
                "Expiry",
                "Status",
            ]);

            for s in &sources {
                let (rights_holder, expiry, status, color) =
                    match rights_by_source.get(s.id.as_str()) {
                        None => ("—", "—", "NO RIGHTS RECORD", Color::Red),
                        Some(r) => {
                            let expiry = r.expiry_date.as_deref().unwrap_or("perpetual");
                            let (status, color) = if is_expired(expiry) {
                                ("EXPIRED", Color::Red)
                            } else {
                                ("OK", Color::Green)
                            };
                            (r.rights_holder.as_str(), expiry, status, color)
                        }
                    };
                let title_short: String = s.title.chars().take(40).collect();
                table.add_row([
                    Cell::new(&s.id),
                    Cell::new(title_short),
                    Cell::new(&s.license),
                    Cell::new(rights_holder),
                    Cell::new(expiry),
                    Cell::new(status).fg(color),
                ]);
            }
            println!("{table}");
        }
    }

    Ok(())
}

use anyhow::Result;
use clap::Args;
use serde::Serialize;
use std::path::PathBuf;

use dkp_core::{procedures, Pack};

use comfy_table::{presets::UTF8_FULL, Table};

use crate::{cli::CmdCtx, output::Render};

#[derive(Args, Debug)]
pub struct InfoArgs {
    /// Path to the DKP bundle directory or .zip archive
    pub pack: PathBuf,

    /// Exit non-zero if any required machine asset is missing
    #[arg(long)]
    pub check: bool,
}

#[derive(Debug, Serialize)]
struct PackInfo {
    name: String,
    domain: String,
    version: String,
    updated: String,
    asset_counts: AssetCounts,
    compliance: ComplianceStatus,
}

#[derive(Debug, Serialize, Default)]
struct AssetCounts {
    terms: Option<usize>,
    rules: Option<usize>,
    chunks: Option<usize>,
    eval_pairs: Option<usize>,
    graph: bool,
    taxonomy: bool,
    assets: bool,
    cross_refs: bool,
    okf: bool,
    procedures_total: usize,
    procedures_with_wasm: usize,
}

#[derive(Debug, Serialize)]
struct ComplianceStatus {
    sources_csv: Option<usize>,
    rights_log_csv: Option<usize>,
    review_notes: bool,
    checksums_json: bool,
}

impl Render for PackInfo {
    fn render_plain(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("Pack:     {}\n", self.name));
        out.push_str(&format!("Domain:   {}\n", self.domain));
        out.push_str(&format!("Version:  {}\n", self.version));
        out.push_str(&format!("Updated:  {}\n", self.updated));
        out.push('\n');
        out.push_str("Assets\n");
        fmt_asset(
            &mut out,
            "Terms",
            self.asset_counts
                .terms
                .map(|n| format!("{n} glossary entries")),
        );
        fmt_asset(
            &mut out,
            "Rules",
            self.asset_counts.rules.map(|n| format!("{n} domain rules")),
        );
        fmt_asset(
            &mut out,
            "Chunks",
            self.asset_counts
                .chunks
                .map(|n| format!("{n} retrieval chunks")),
        );
        fmt_asset(
            &mut out,
            "Eval",
            self.asset_counts
                .eval_pairs
                .map(|n| format!("{n} Q/A pairs")),
        );
        fmt_flag(&mut out, "Graph", self.asset_counts.graph);
        fmt_flag(&mut out, "Taxonomy", self.asset_counts.taxonomy);
        fmt_flag(&mut out, "Assets", self.asset_counts.assets);
        fmt_flag(&mut out, "CrossRefs", self.asset_counts.cross_refs);
        fmt_flag(&mut out, "OKF", self.asset_counts.okf);
        let c = &self.asset_counts;
        if c.procedures_total > 0 {
            out.push_str(&format!(
                "  {:<12}{} ({} with WASM binary)\n",
                "Procedures", c.procedures_total, c.procedures_with_wasm
            ));
        } else {
            out.push_str(&format!("  {:<12}none\n", "Procedures"));
        }
        out.push('\n');
        out.push_str("Compliance\n");
        fmt_asset(
            &mut out,
            "sources.csv",
            self.compliance.sources_csv.map(|n| format!("{n} entries")),
        );
        fmt_asset(
            &mut out,
            "rights_log.csv",
            self.compliance
                .rights_log_csv
                .map(|n| format!("{n} entries")),
        );
        fmt_flag(&mut out, "review_notes", self.compliance.review_notes);
        fmt_flag(&mut out, "checksums.json", self.compliance.checksums_json);
        out
    }

    fn render_table(&self) -> String {
        let mut meta = Table::new();
        meta.load_preset(UTF8_FULL);
        meta.set_header(["Field", "Value"]);
        meta.add_row(["Name", &self.name]);
        meta.add_row(["Domain", &self.domain]);
        meta.add_row(["Version", &self.version]);
        meta.add_row(["Updated", &self.updated]);

        let mut assets = Table::new();
        assets.load_preset(UTF8_FULL);
        assets.set_header(["Asset", "Count / Present"]);
        let c = &self.asset_counts;
        assets.add_row([
            "Terms",
            &c.terms
                .map_or("not present".into(), |n| format!("{n} glossary entries")),
        ]);
        assets.add_row([
            "Rules",
            &c.rules
                .map_or("not present".into(), |n| format!("{n} domain rules")),
        ]);
        assets.add_row([
            "Chunks",
            &c.chunks
                .map_or("not present".into(), |n| format!("{n} retrieval chunks")),
        ]);
        assets.add_row([
            "Eval",
            &c.eval_pairs
                .map_or("not present".into(), |n| format!("{n} Q/A pairs")),
        ]);
        assets.add_row(["Graph", if c.graph { "present" } else { "not present" }]);
        assets.add_row([
            "Taxonomy",
            if c.taxonomy { "present" } else { "not present" },
        ]);
        assets.add_row(["Assets", if c.assets { "present" } else { "not present" }]);
        assets.add_row([
            "CrossRefs",
            if c.cross_refs {
                "present"
            } else {
                "not present"
            },
        ]);
        assets.add_row(["OKF", if c.okf { "present" } else { "not present" }]);
        assets.add_row([
            "Procedures",
            &if c.procedures_total > 0 {
                format!(
                    "{} ({} with WASM)",
                    c.procedures_total, c.procedures_with_wasm
                )
            } else {
                "none".to_string()
            },
        ]);

        let mut compliance = Table::new();
        compliance.load_preset(UTF8_FULL);
        compliance.set_header(["File", "Status"]);
        let cp = &self.compliance;
        compliance.add_row([
            "sources.csv",
            &cp.sources_csv
                .map_or("not present".into(), |n| format!("{n} entries")),
        ]);
        compliance.add_row([
            "rights_log.csv",
            &cp.rights_log_csv
                .map_or("not present".into(), |n| format!("{n} entries")),
        ]);
        compliance.add_row([
            "review_notes",
            if cp.review_notes {
                "present"
            } else {
                "not present"
            },
        ]);
        compliance.add_row([
            "checksums.json",
            if cp.checksums_json {
                "present"
            } else {
                "not present"
            },
        ]);

        format!("{meta}\n\nAssets\n{assets}\n\nCompliance\n{compliance}\n")
    }
}

fn fmt_asset(out: &mut String, label: &str, value: Option<String>) {
    let v = value.as_deref().unwrap_or("not present");
    out.push_str(&format!("  {label:<12}{v}\n"));
}

fn fmt_flag(out: &mut String, label: &str, present: bool) {
    let v = if present { "present" } else { "not present" };
    out.push_str(&format!("  {label:<12}{v}\n"));
}

pub async fn run(args: InfoArgs, cli: &CmdCtx) -> Result<()> {
    let pack = Pack::open(&args.pack)?;

    let info = PackInfo {
        name: pack.manifest.name.clone(),
        domain: pack.manifest.domain.clone(),
        version: pack.manifest.version.clone(),
        updated: pack.manifest.update_date.clone(),
        asset_counts: AssetCounts {
            terms: count_json_array(&pack, "glossary.json", "terms"),
            rules: count_json_array(&pack, "rules.json", "rules"),
            chunks: count_jsonl(&pack, "retrieval_chunks.jsonl"),
            eval_pairs: count_jsonl(&pack, "eval_set.jsonl"),
            graph: pack.has_knowledge_graph(),
            taxonomy: pack.machine_file("ontology.json").exists(),
            assets: pack.has_assets(),
            cross_refs: pack.has_cross_refs(),
            okf: pack.has_okf(),
            procedures_total: procedures::count_total(&pack),
            procedures_with_wasm: procedures::count_with_wasm(&pack),
        },
        compliance: ComplianceStatus {
            sources_csv: count_csv_rows(pack.evidence_file("sources.csv")),
            rights_log_csv: count_csv_rows(pack.evidence_file("rights_log.csv")),
            review_notes: pack.evidence_file("review_notes.md").exists(),
            checksums_json: pack.has_checksums(),
        },
    };

    info.print(cli.output);

    if args.check {
        let missing = required_missing(&pack);
        if !missing.is_empty() {
            anyhow::bail!("missing required assets: {}", missing.join(", "));
        }
    }

    Ok(())
}

fn count_json_array(pack: &Pack, file: &str, key: &str) -> Option<usize> {
    let bytes = std::fs::read(pack.machine_file(file)).ok()?;
    let v: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
    v[key].as_array().map(|a| a.len())
}

fn count_jsonl(pack: &Pack, file: &str) -> Option<usize> {
    let path = pack.machine_file(file);
    if !path.exists() {
        return None;
    }
    let content = std::fs::read_to_string(path).ok()?;
    Some(content.lines().filter(|l| !l.trim().is_empty()).count())
}

fn count_csv_rows(path: PathBuf) -> Option<usize> {
    let mut rdr = csv::Reader::from_path(path).ok()?;
    Some(rdr.records().filter_map(|r| r.ok()).count())
}

fn required_missing(pack: &Pack) -> Vec<String> {
    [
        "glossary.json",
        "rules.json",
        "ontology.json",
        "constraints.json",
        "system_prompt.md",
    ]
    .iter()
    .filter(|f| !pack.machine_file(f).exists())
    .map(|f| format!("machine/{f}"))
    .collect()
}

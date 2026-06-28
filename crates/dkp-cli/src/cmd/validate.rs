use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use dkp_core::{
    validate::{
        gate4, gate7, gate8,
        gates::{ConformanceLevel, GateStatus, ValidationReport},
    },
    Pack,
};

use comfy_table::{presets::UTF8_FULL, Cell, Color, Table};

use crate::{cli::CmdCtx, output::Render};

#[derive(Args, Debug)]
pub struct ValidateArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,

    /// Treat warnings as errors
    #[arg(long)]
    pub strict: bool,

    /// Run only a specific quality gate (4, 7, or 8)
    #[arg(long, value_name = "N")]
    pub gate: Option<u8>,
}

impl Render for ValidationReport {
    fn render_plain(&self) -> String {
        let mut out = String::new();
        for gate in &self.gates {
            out.push_str(&format!("\n[Gate {}]\n", gate.gate));
            for check in &gate.checks {
                let icon = match check.status {
                    GateStatus::Pass => "OK      ",
                    GateStatus::Fail => "FAIL    ",
                    GateStatus::Skipped => "SKIPPED ",
                    GateStatus::NotApplicable => "N/A     ",
                };
                out.push_str(&format!("  {}  {}\n", icon, check.description));
                if let Some(ref d) = check.detail {
                    out.push_str(&format!("          {d}\n"));
                }
            }
        }
        out.push('\n');
        let status = match self.conformance {
            ConformanceLevel::DkpConformant => "DKP-Conformant",
            ConformanceLevel::DkpReviewed => "DKP-Reviewed",
            ConformanceLevel::NonConformant => "Non-Conformant",
        };
        out.push_str(&format!("Conformance: {status}\n"));
        out.push_str(&format!(
            "Reviewed:    {}\n",
            if self.reviewed {
                "Yes"
            } else {
                "No (no sign-off in evidence/review_notes.md)"
            }
        ));
        out
    }

    fn render_table(&self) -> String {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        table.set_header(["Gate", "Status", "Check", "Detail"]);
        for gate in &self.gates {
            for check in &gate.checks {
                let (label, color) = match check.status {
                    GateStatus::Pass => ("OK", Color::Green),
                    GateStatus::Fail => ("FAIL", Color::Red),
                    GateStatus::Skipped => ("SKIPPED", Color::Yellow),
                    GateStatus::NotApplicable => ("N/A", Color::DarkGrey),
                };
                table.add_row([
                    Cell::new(gate.gate),
                    Cell::new(label).fg(color),
                    Cell::new(&check.description),
                    Cell::new(check.detail.as_deref().unwrap_or("")),
                ]);
            }
        }
        let status = match self.conformance {
            ConformanceLevel::DkpConformant => "DKP-Conformant",
            ConformanceLevel::DkpReviewed => "DKP-Reviewed",
            ConformanceLevel::NonConformant => "Non-Conformant",
        };
        let reviewed = if self.reviewed { "Yes" } else { "No" };
        format!("{table}\n\nConformance: {status}    Reviewed: {reviewed}\n")
    }
}

pub async fn run(args: ValidateArgs, cli: &CmdCtx) -> Result<()> {
    let pack = Pack::open(&args.pack)?;

    let mut gate_results = Vec::new();

    let run_gate = |n: u8| args.gate.is_none_or(|g| g == n);

    if run_gate(4) {
        gate_results.push(gate4::run(&pack));
    }
    if run_gate(7) {
        gate_results.push(gate7::run(&pack));
    }
    if run_gate(8) {
        gate_results.push(gate8::run(&pack));
    }

    let any_failed = gate_results.iter().any(|g| g.status == GateStatus::Fail);
    let reviewed = has_review_signoff(&pack.evidence_file("review_notes.md"));

    let conformance = if any_failed {
        ConformanceLevel::NonConformant
    } else if reviewed {
        ConformanceLevel::DkpReviewed
    } else {
        ConformanceLevel::DkpConformant
    };

    let report = ValidationReport {
        pack_name: pack.manifest.name.clone(),
        pack_version: pack.manifest.version.clone(),
        gates: gate_results,
        overall: if any_failed {
            GateStatus::Fail
        } else {
            GateStatus::Pass
        },
        conformance,
        reviewed,
    };

    report.print(cli.output);

    if any_failed {
        anyhow::bail!("validation failed");
    }

    Ok(())
}

/// Returns true only if review_notes.md contains the canonical sign-off line.
/// Mere file presence is not enough — the author must explicitly attest.
fn has_review_signoff(path: &std::path::Path) -> bool {
    std::fs::read_to_string(path)
        .map(|s| s.lines().any(|l| l.trim() == "- **Status:** DKP-Reviewed"))
        .unwrap_or(false)
}

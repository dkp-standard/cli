use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use dkp_core::{
    validate::{gate4, gate7, gate8, gates::GateStatus},
    Pack,
};

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct ReleaseCheckArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,

    /// Fail on warnings
    #[arg(long)]
    pub strict: bool,
}

struct Item {
    label: &'static str,
    pass: bool,
    detail: String,
    is_warning: bool,
}

impl Item {
    fn pass(label: &'static str) -> Self {
        Self {
            label,
            pass: true,
            detail: String::new(),
            is_warning: false,
        }
    }
    fn fail(label: &'static str, detail: impl Into<String>) -> Self {
        Self {
            label,
            pass: false,
            detail: detail.into(),
            is_warning: false,
        }
    }
    fn warn(label: &'static str, detail: impl Into<String>) -> Self {
        Self {
            label,
            pass: false,
            detail: detail.into(),
            is_warning: true,
        }
    }
}

pub async fn run(args: ReleaseCheckArgs, _cli: &CmdCtx) -> Result<()> {
    let pack = Pack::open(&args.pack)?;
    let mut items: Vec<Item> = Vec::new();

    // ── Gate 4: Machine Usability ────────────────────────────────────────────
    let g4 = gate4::run(&pack);
    if g4.status == GateStatus::Pass {
        items.push(Item::pass("Gate 4 (machine usability)"));
    } else {
        let failures: Vec<_> = g4
            .checks
            .iter()
            .filter(|c| c.status == GateStatus::Fail)
            .map(|c| c.description.as_str())
            .collect();
        items.push(Item::fail(
            "Gate 4 (machine usability)",
            failures.join("; "),
        ));
    }

    // ── Gate 8: OKF Conformance ──────────────────────────────────────────────
    let g8 = gate8::run(&pack);
    if g8.status == GateStatus::Pass || g8.status == GateStatus::Skipped {
        items.push(Item::pass("Gate 8 (OKF conformance)"));
    } else {
        let failures: Vec<_> = g8
            .checks
            .iter()
            .filter(|c| c.status == GateStatus::Fail)
            .map(|c| c.description.as_str())
            .collect();
        items.push(Item::fail("Gate 8 (OKF conformance)", failures.join("; ")));
    }

    // ── Gate 7: Evaluation ───────────────────────────────────────────────────
    let g7 = gate7::run(&pack);
    match g7.status {
        GateStatus::Pass => items.push(Item::pass("Gate 7 (evaluation)")),
        GateStatus::Skipped => {
            items.push(Item::warn(
                "Gate 7 (evaluation)",
                "eval_set.jsonl absent — DKP-Evaluated tier not achievable",
            ));
        }
        _ => {
            let failures: Vec<_> = g7
                .checks
                .iter()
                .filter(|c| c.status == GateStatus::Fail)
                .map(|c| c.description.as_str())
                .collect();
            items.push(Item::fail("Gate 7 (evaluation)", failures.join("; ")));
        }
    }

    // ── Review sign-off ──────────────────────────────────────────────────────
    let review_notes = pack.evidence_file("review_notes.md");
    if review_notes.exists() {
        let has_signoff = std::fs::read_to_string(&review_notes)
            .map(|s| s.lines().any(|l| l.trim() == "- **Status:** DKP-Reviewed"))
            .unwrap_or(false);
        if has_signoff {
            items.push(Item::pass("review_notes.md sign-off"));
        } else {
            items.push(Item::warn(
                "review_notes.md sign-off",
                "file present but missing '- **Status:** DKP-Reviewed' attestation",
            ));
        }
    } else {
        items.push(Item::warn(
            "review_notes.md sign-off",
            "evidence/review_notes.md not present",
        ));
    }

    // ── Bundle signature ─────────────────────────────────────────────────────
    if pack.has_bundle_sig() {
        items.push(Item::pass("bundle.sig present"));
    } else {
        items.push(Item::warn(
            "bundle.sig present",
            "run 'dkp sign' before publishing",
        ));
    }

    // ── Publisher identity ───────────────────────────────────────────────────
    if pack.manifest.publisher.is_some() {
        items.push(Item::pass("manifest.publisher set"));
    } else {
        items.push(Item::warn(
            "manifest.publisher set",
            "recommended before registry publish",
        ));
    }

    // ── Print results ────────────────────────────────────────────────────────
    println!(
        "Release check — {} v{}\n",
        pack.manifest.name, pack.manifest.version
    );

    let mut any_fail = false;
    let mut any_warn = false;

    for item in &items {
        if item.pass {
            println!("  [OK]   {}", item.label);
        } else if item.is_warning {
            println!("  [WARN] {}  —  {}", item.label, item.detail);
            any_warn = true;
        } else {
            println!("  [FAIL] {}  —  {}", item.label, item.detail);
            any_fail = true;
        }
    }

    println!();
    if any_fail || (args.strict && any_warn) {
        anyhow::bail!("release check failed");
    }

    if any_warn {
        println!("Release check passed with warnings.");
    } else {
        println!("Release check passed — pack is ready to publish.");
    }

    Ok(())
}

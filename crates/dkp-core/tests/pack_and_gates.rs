use dkp_core::pack::loader::Pack;
use dkp_core::validate::gates::GateStatus;
use dkp_core::validate::{gate4, gate7, gate8};

fn fixture_path(name: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn minimal_valid_pack_passes_gate4_and_gate8() {
    let pack = Pack::open(fixture_path("minimal_valid_pack")).expect("pack should open");

    let g4 = gate4::run(&pack);
    assert_eq!(
        g4.status,
        GateStatus::Pass,
        "gate4 checks: {:#?}",
        g4.checks
    );

    let g7 = gate7::run(&pack);
    assert_eq!(
        g7.status,
        GateStatus::Skipped,
        "gate7 should skip when no eval_set.jsonl is present"
    );

    let g8 = gate8::run(&pack);
    assert_eq!(
        g8.status,
        GateStatus::Pass,
        "gate8 checks: {:#?}",
        g8.checks
    );
}

#[test]
fn broken_pack_fails_gate4() {
    let pack = Pack::open(fixture_path("broken_pack"))
        .expect("pack should open (manifest itself is valid)");

    let g4 = gate4::run(&pack);
    assert_eq!(g4.status, GateStatus::Fail);

    // Missing glossary.json should be flagged.
    assert!(g4
        .checks
        .iter()
        .any(|c| c.description.contains("glossary.json") && c.status == GateStatus::Fail));

    // Invalid rules.json should be flagged as a parse failure.
    assert!(g4
        .checks
        .iter()
        .any(|c| c.description.contains("rules.json") && c.status == GateStatus::Fail));
}

#[test]
fn broken_pack_gate8_not_applicable_without_okf_layer() {
    let pack = Pack::open(fixture_path("broken_pack")).expect("pack should open");
    let g8 = gate8::run(&pack);
    assert_eq!(g8.status, GateStatus::NotApplicable);
}

use std::path::{Path, PathBuf};

use assert_cmd::Command;

/// Returns a `Command` for the `dkp` binary under test.
pub fn dkp_cmd() -> Command {
    Command::cargo_bin("dkp").expect("dkp binary should build")
}

/// Runs `dkp init` to scaffold a minimal, fully valid pack directory under
/// `parent`, returning the resulting pack path. Using the real `init` command
/// (rather than hand-written fixtures) keeps fixtures automatically in sync
/// with whatever scaffolding logic ships.
#[allow(dead_code)]
pub fn scaffold_pack(parent: &Path, name: &str, domain: &str) -> PathBuf {
    let out = parent.join(name);
    dkp_cmd()
        .arg("init")
        .arg(name)
        .arg("--domain")
        .arg(domain)
        .arg("--out")
        .arg(&out)
        .assert()
        .success();
    out
}

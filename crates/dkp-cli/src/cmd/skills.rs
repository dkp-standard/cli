use anyhow::Result;
use clap::{Args, Subcommand};
use std::path::PathBuf;

use comfy_table::{presets::UTF8_FULL, Table};
use dkp_core::Pack;

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct SkillsArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,

    #[command(subcommand)]
    pub command: SkillsCommands,
}

#[derive(Subcommand, Debug)]
pub enum SkillsCommands {
    /// List all skills in skills/
    List,
    /// Check SKILL.md format conformance
    Validate,
    /// Print a specific skill's SKILL.md
    Show { name: String },
}

/// Extract SKILL.md frontmatter as key→value string pairs.
/// Only handles simple `key: value` lines (no nesting needed for SKILL.md).
fn read_skill_frontmatter(
    skill_md: &std::path::Path,
) -> anyhow::Result<std::collections::HashMap<String, String>> {
    let content = std::fs::read_to_string(skill_md)?;
    let inner = content
        .strip_prefix("---")
        .and_then(|s| s.split_once("---"))
        .map(|(yaml, _)| yaml)
        .ok_or_else(|| anyhow::anyhow!("missing YAML frontmatter"))?;
    let mut map = std::collections::HashMap::new();
    for line in inner.lines() {
        if let Some((k, v)) = line.split_once(':') {
            let key = k.trim().to_string();
            let val = v.trim().trim_matches('"').to_string();
            if !key.is_empty() {
                map.insert(key, val);
            }
        }
    }
    Ok(map)
}

/// Enumerate skill subdirectories and their SKILL.md paths.
fn list_skills(skills_dir: &std::path::Path) -> anyhow::Result<Vec<(String, PathBuf)>> {
    let mut skills = Vec::new();
    for entry in std::fs::read_dir(skills_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
            let skill_md = path.join("SKILL.md");
            skills.push((name, skill_md));
        }
    }
    skills.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(skills)
}

pub async fn run(args: SkillsArgs, _cli: &CmdCtx) -> Result<()> {
    let pack = Pack::open(&args.pack)?;

    if !pack.has_skills() {
        println!("skills/ directory not present in this pack.");
        return Ok(());
    }

    let skills_dir = pack.skills_dir();

    match args.command {
        SkillsCommands::List => {
            let skills = list_skills(&skills_dir)?;
            let mut table = Table::new();
            table.load_preset(UTF8_FULL);
            table.set_header(["Name", "SKILL.md", "Description"]);
            for (name, skill_md) in &skills {
                let (present, description) = if skill_md.exists() {
                    let desc = read_skill_frontmatter(skill_md)
                        .ok()
                        .and_then(|fm| fm.get("description").cloned())
                        .map(|s| s.chars().take(60).collect::<String>())
                        .unwrap_or_default();
                    ("yes", desc)
                } else {
                    ("missing", String::new())
                };
                table.add_row([name.as_str(), present, &description]);
            }
            println!("{table}");
            println!("\n{} skill(s)", skills.len());
        }

        SkillsCommands::Validate => {
            let skills = list_skills(&skills_dir)?;
            let mut errors: Vec<String> = Vec::new();

            for (name, skill_md) in &skills {
                if !skill_md.exists() {
                    errors.push(format!("{name}: missing SKILL.md"));
                    continue;
                }
                match read_skill_frontmatter(skill_md) {
                    Err(e) => errors.push(format!("{name}/SKILL.md: {e}")),
                    Ok(fm) => {
                        for field in ["name", "description", "dkp_pack"] {
                            if !fm.contains_key(field) {
                                errors.push(format!(
                                    "{name}/SKILL.md: missing required field '{field}'"
                                ));
                            }
                        }
                    }
                }
            }

            if errors.is_empty() {
                println!(
                    "[PASS] skills — {} skill(s), all SKILL.md files valid",
                    skills.len()
                );
            } else {
                println!("[FAIL] skills — {} error(s):", errors.len());
                for e in &errors {
                    println!("  {e}");
                }
                anyhow::bail!("skills validation failed");
            }
        }

        SkillsCommands::Show { name } => {
            let skill_md = skills_dir.join(&name).join("SKILL.md");
            if !skill_md.exists() {
                anyhow::bail!("skill '{}' not found (no skills/{}/SKILL.md)", name, name);
            }
            print!("{}", std::fs::read_to_string(&skill_md)?);
        }
    }

    Ok(())
}

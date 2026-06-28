use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use dkp_core::{
    procedures::scaffold::{self, scaffold_script},
    Pack,
};

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct NewArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,

    /// Procedure ID — used as the file stem and Rust crate name (e.g. "macro-calc")
    pub id: String,

    /// Human-readable title for the procedure
    #[arg(long, default_value = "")]
    pub title: String,

    /// One-sentence description of what the procedure does
    #[arg(long, default_value = "")]
    pub description: String,

    /// Procedure language/runtime: wasm | python | javascript
    #[arg(long, default_value = "wasm", value_name = "LANG")]
    pub lang: String,
}

pub async fn run(args: NewArgs, _ctx: &CmdCtx) -> Result<()> {
    let pack = Pack::open(&args.pack)?;

    let title = if args.title.is_empty() {
        args.id
            .replace('-', " ")
            .split_whitespace()
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().to_string() + c.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        args.title.clone()
    };

    let description = if args.description.is_empty() {
        format!("Executable procedure: {title}")
    } else {
        args.description.clone()
    };

    let (created, next_steps) = match args.lang.as_str() {
        "python" | "javascript" => {
            let files = scaffold_script(&pack.root, &args.id, &title, &description, &args.lang)?;
            let ext = if args.lang == "javascript" {
                "js"
            } else {
                "py"
            };
            let steps = format!(
                "Next steps:\n\
                 1. Edit machine/procedures/{id}.schema.json to describe your input/output shapes\n\
                 2. Implement logic in machine/procedures/{id}.{ext}\n\
                 3. Run `dkp run {pack} {id} --allow-unsigned` to test locally\n\
                 4. Run `dkp procedures validate {pack}` to check completeness",
                id = args.id,
                ext = ext,
                pack = args.pack.display(),
            );
            (files, steps)
        }
        _ => {
            let files = scaffold::scaffold(&pack.root, &args.id, &title, &description)?;
            let steps = format!(
                "Next steps:\n\
                 1. Edit machine/procedures/{id}.schema.json to describe your input/output shapes\n\
                 2. Implement logic in machine/procedures/src/{id}/src/main.rs\n\
                 3. Run machine/procedures/src/build.sh to compile all procedures\n\
                 4. Run `dkp procedures validate {pack}` to check completeness",
                id = args.id,
                pack = args.pack.display(),
            );
            (files, steps)
        }
    };

    println!("Scaffolded procedure '{}' (lang: {}):", args.id, args.lang);
    for path in &created {
        println!(
            "  {}",
            path.strip_prefix(&pack.root).unwrap_or(path).display()
        );
    }
    println!();
    println!("{next_steps}");

    Ok(())
}

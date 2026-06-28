use anyhow::{bail, Result};
use clap::Args;
use std::path::PathBuf;
use std::sync::Arc;

use dkp_core::Pack;
use dkp_gen_core::prompt::templates::prompt_eval_answer;
use dkp_gen_core::{CliOverrides, GenConfig, LlmClient, OpenAiClient};

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct PromptArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,

    /// Optional question (omit to enter interactive REPL)
    pub question: Option<String>,

    #[arg(long, value_name = "PROVIDER")]
    pub provider: Option<String>,

    #[arg(long, value_name = "MODEL")]
    pub model: Option<String>,

    #[arg(
        long,
        value_name = "KEY",
        env = "DKP_GEN_API_KEY",
        hide_env_values = true
    )]
    pub api_key: Option<String>,

    /// Token budget for injected pack context
    #[arg(long, default_value = "4000", value_name = "N")]
    pub max_tokens: u32,

    /// Context scope: system-prompt | full | chunks | minimal
    #[arg(long, default_value = "system-prompt", value_name = "SCOPE")]
    pub scope: String,
}

// ── Context builders (mirrored from inject.rs) ────────────────────────────────

fn build_context(pack: &Pack, scope: &str, max_tokens: u32) -> Result<String> {
    let content = match scope {
        "minimal" => pack
            .load_system_prompt()?
            .unwrap_or_else(|| format!("# {}\n\nNo system prompt present.", pack.manifest.name)),
        "system-prompt" => {
            let mut out = String::new();
            if let Some(sp) = pack.load_system_prompt()? {
                out.push_str(&sp);
                out.push_str("\n\n");
            }
            if let Some(gf) = pack.load_glossary()? {
                if !gf.terms.is_empty() {
                    out.push_str("## Key Terms\n\n");
                    for t in &gf.terms {
                        out.push_str(&format!("**{}**: {}\n", t.term, t.definition));
                    }
                }
            }
            out
        }
        "chunks" => build_chunks(pack, max_tokens)?,
        "full" => {
            let mut out = String::new();
            if let Some(sp) = pack.load_system_prompt()? {
                out.push_str(&sp);
                out.push_str("\n\n");
            }
            if let Some(gf) = pack.load_glossary()? {
                if !gf.terms.is_empty() {
                    out.push_str("## Glossary\n\n");
                    for t in &gf.terms {
                        out.push_str(&format!("**{}** ({}): {}\n", t.term, t.id, t.definition));
                    }
                    out.push('\n');
                }
            }
            if let Some(rf) = pack.load_rules()? {
                if !rf.rules.is_empty() {
                    out.push_str("## Domain Rules\n\n");
                    for r in &rf.rules {
                        out.push_str(&format!("**{}** [{}]: {}\n", r.title, r.id, r.description));
                    }
                    out.push('\n');
                }
            }
            if let Some(cf) = pack.load_constraints()? {
                let all: Vec<_> = cf.all_constraints().collect();
                if !all.is_empty() {
                    out.push_str("## Constraints\n\n");
                    for c in all {
                        out.push_str(&format!("**{}** [{}]: {}\n", c.title, c.id, c.description));
                    }
                    out.push('\n');
                }
            }
            out.push_str(&build_chunks(pack, max_tokens)?);
            out
        }
        other => bail!(
            "unknown scope '{}'. Valid scopes: system-prompt, full, minimal, chunks",
            other
        ),
    };

    // Hard truncate to budget
    let limit_chars = (max_tokens as usize) * 4;
    if content.len() <= limit_chars {
        return Ok(content);
    }
    let truncated = &content[..limit_chars];
    let cut = truncated.rfind("\n\n").unwrap_or(limit_chars);
    Ok(format!(
        "{}\n\n[...context truncated to token budget...]",
        &content[..cut]
    ))
}

fn build_chunks(pack: &Pack, max_tokens: u32) -> Result<String> {
    let mut chunks = pack.load_chunks()?;
    chunks.sort_by(|a, b| {
        b.confidence
            .unwrap_or(0.5)
            .partial_cmp(&a.confidence.unwrap_or(0.5))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut out = String::new();
    out.push_str("## Retrieval Chunks\n\n");
    for c in &chunks {
        let entry = format!("### {}\n\n{}\n\n", c.title, c.chunk_text);
        let current = out.len().div_ceil(4);
        let entry_toks = entry.len().div_ceil(4);
        if current as u32 + entry_toks as u32 > max_tokens {
            break;
        }
        out.push_str(&entry);
    }
    Ok(out)
}

// ── REPL commands ────────────────────────────────────────────────────────────

fn print_help() {
    println!(
        "\
Commands:
  /help               Show this help
  /procs              List procedures available in this pack
  /proc <id>          Show a procedure's description and input/output schema
  /run <id> [json]    Run a procedure (json defaults to {{}})
  /quit               Exit the REPL

Any other input is sent to the LLM as a grounded question."
    );
    println!();
}

fn list_procedures(pack: &Pack) {
    use dkp_core::procedures;
    match procedures::list(pack) {
        Err(e) => eprintln!("Error listing procedures: {e}"),
        Ok(defs) if defs.is_empty() => println!("No procedures in this pack."),
        Ok(defs) => {
            println!("{} procedure(s):\n", defs.len());
            for d in &defs {
                let kind = if d.wasm_path.is_some() {
                    "wasm"
                } else if d.entry_point.is_some() {
                    "native"
                } else {
                    "no-exec"
                };
                println!("  {:20}  [{}]  {}", d.id, kind, d.schema.description);
            }
            println!();
        }
    }
}

fn show_procedure(pack: &Pack, id: &str) {
    use dkp_core::procedures;
    let defs = match procedures::list(pack) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error: {e}");
            return;
        }
    };
    let Some(def) = defs.into_iter().find(|d| d.id == id) else {
        eprintln!("Procedure '{id}' not found. Use /procs to list available procedures.");
        return;
    };
    println!("\n{} — {}", def.schema.id, def.schema.title);
    println!("{}\n", def.schema.description);
    println!("Input schema:");
    println!(
        "{}",
        serde_json::to_string_pretty(&def.schema.input).unwrap_or_default()
    );
    println!("\nOutput schema:");
    println!(
        "{}\n",
        serde_json::to_string_pretty(&def.schema.output).unwrap_or_default()
    );
    if def.doc_path.exists() {
        if let Ok(doc) = std::fs::read_to_string(&def.doc_path) {
            println!("{doc}");
        }
    }
}

// ── Procedure runner ─────────────────────────────────────────────────────────

fn run_procedure(pack: &Pack, args: &str) {
    #[cfg(not(feature = "procedures"))]
    {
        let _ = (pack, args);
        eprintln!("Error: procedure execution requires the 'procedures' feature.");
        return;
    }

    #[cfg(feature = "procedures")]
    {
        use dkp_core::procedures;

        // Split "procedure-id [optional json]"
        let (id, json_str) = match args.split_once(char::is_whitespace) {
            Some((id, rest)) => (id.trim(), rest.trim()),
            None => (args.trim(), ""),
        };

        if id.is_empty() {
            eprintln!("Usage: /run <procedure-id> [json-input]");
            return;
        }

        let input: serde_json::Value = if json_str.is_empty() {
            serde_json::Value::Object(Default::default())
        } else {
            match serde_json::from_str(json_str) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error: invalid JSON input: {e}");
                    return;
                }
            }
        };

        let defs = match procedures::list(pack) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Error listing procedures: {e}");
                return;
            }
        };

        let def = match defs.into_iter().find(|d| d.id == id) {
            Some(d) => d,
            None => {
                eprintln!("Error: procedure '{id}' not found in this pack.");
                return;
            }
        };

        let opts = procedures::executor::RunOptions {
            input,
            timeout_ms: None,
            allow_unsigned: false,
        };

        let result = tokio::task::block_in_place(|| procedures::executor::run(pack, &def, opts));

        match result {
            Ok(output) => match serde_json::to_string_pretty(&output) {
                Ok(s) => println!("\n{s}\n"),
                Err(e) => eprintln!("Error serializing output: {e}"),
            },
            Err(e) => eprintln!("Error: {e}"),
        }
    }
}

// ── Main ─────────────────────────────────────────────────────────────────────

pub async fn run(args: PromptArgs, _cli: &CmdCtx) -> Result<()> {
    let pack = Pack::open(&args.pack)?;
    let domain = pack.manifest.domain.clone();
    let pack_name = pack.manifest.name.clone();

    let config = GenConfig::load(CliOverrides {
        base_url: None,
        api_key: args.api_key,
        model: args.model,
        overwrite: false,
    })?;
    if config.api_key.is_empty() {
        bail!(
            "API key required: pass --api-key, set DKP_GEN_API_KEY, \
             or add api_key to ~/.dkp/gen.toml"
        );
    }

    let client = Arc::new(OpenAiClient::new(&config)?);
    let context = build_context(&pack, &args.scope, args.max_tokens)?;

    if let Some(question) = args.question {
        // Single-shot mode
        let (system, user) = prompt_eval_answer(&domain, &pack_name, &question, &context);
        let answer = client
            .complete(&system, &user)
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        println!("{answer}");
        return Ok(());
    }

    // Interactive REPL
    println!(
        "DKP Prompt — {} v{}  [model: {}  scope: {}]",
        pack_name, pack.manifest.version, config.model, args.scope
    );
    print_help();

    let stdin = tokio::io::stdin();
    let mut reader = tokio::io::BufReader::new(stdin);
    let mut input = String::new();

    loop {
        use std::io::Write;
        print!("> ");
        std::io::stdout().flush()?;

        input.clear();
        let n = tokio::io::AsyncBufReadExt::read_line(&mut reader, &mut input).await?;
        if n == 0 {
            break; // EOF
        }

        let question = input.trim().to_string();
        if question.is_empty() {
            continue;
        }
        if question == "/quit" || question == "/exit" {
            break;
        }

        if question == "/help" {
            print_help();
            continue;
        }

        if question == "/procs" {
            list_procedures(&pack);
            continue;
        }

        if let Some(id) = question.strip_prefix("/proc ") {
            show_procedure(&pack, id.trim());
            continue;
        }

        if let Some(rest) = question.strip_prefix("/run ") {
            run_procedure(&pack, rest.trim());
            continue;
        }

        if question.starts_with('/') {
            eprintln!("Unknown command. Type /help for available commands.");
            continue;
        }

        eprintln!("Thinking...");
        let (system, user) = prompt_eval_answer(&domain, &pack_name, &question, &context);
        match client.complete(&system, &user).await {
            Ok(answer) => println!("\n{answer}\n"),
            Err(e) => eprintln!("Error: {e}"),
        }
    }

    Ok(())
}

use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use crate::cli::CmdCtx;

#[derive(Args, Debug)]
pub struct McpManifestArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,

    /// Print what would be written without writing
    #[arg(long)]
    pub dry_run: bool,

    /// Write to a custom path instead of machine/mcp_manifest.json
    #[arg(long, value_name = "PATH")]
    pub out: Option<PathBuf>,
}

pub async fn run(args: McpManifestArgs, _cli: &CmdCtx) -> Result<()> {
    use dkp_core::{
        types::mcp_manifest::{McpManifest, McpResource, McpTool},
        Pack,
    };

    let pack = Pack::open(&args.pack)?;

    let pack_slug = pack.manifest.name.to_lowercase().replace(' ', "-");
    let uri_scheme = pack
        .manifest
        .mcp
        .as_ref()
        .and_then(|m| m.resource_server.as_ref())
        .and_then(|rs| rs.uri_scheme.as_deref())
        .unwrap_or("dkp")
        .to_string();

    // Build resources from whatever assets are present
    let mut resources: Vec<McpResource> = Vec::new();

    // Always include chunks (core asset)
    let chunk_count = pack.load_chunks()?.len();
    resources.push(McpResource {
        uri_template: format!("{uri_scheme}://{pack_slug}/chunks{{?cursor,limit}}"),
        resource_type: "dkp/chunks".to_string(),
        description: "Retrieval chunk listing (id, title, token_count). Fetch individual chunks at /chunks/{id}.".to_string(),
        count: chunk_count,
    });

    if let Some(g) = pack.load_glossary()? {
        let n = g.terms.len();
        resources.push(McpResource {
            uri_template: format!("{uri_scheme}://{pack_slug}/terms/{{id}}"),
            resource_type: "dkp/terms".to_string(),
            description: "Glossary terms".to_string(),
            count: n,
        });
    }

    if let Some(r) = pack.load_rules()? {
        let n = r.rules.len();
        resources.push(McpResource {
            uri_template: format!("{uri_scheme}://{pack_slug}/rules/{{id}}"),
            resource_type: "dkp/rules".to_string(),
            description: "Domain rules".to_string(),
            count: n,
        });
    }

    if let Some(c) = pack.load_constraints()? {
        let n = c.edge_cases.len() + c.anti_patterns.len() + c.hard_limits.len();
        resources.push(McpResource {
            uri_template: format!("{uri_scheme}://{pack_slug}/constraints/{{id}}"),
            resource_type: "dkp/constraints".to_string(),
            description: "Constraints".to_string(),
            count: n,
        });
    }

    if let Some(o) = pack.load_ontology()? {
        let n = o.entity_types.len();
        resources.push(McpResource {
            uri_template: format!("{uri_scheme}://{pack_slug}/entity-types/{{id}}"),
            resource_type: "dkp/entity-types".to_string(),
            description: "Ontology entity types".to_string(),
            count: n,
        });
    }

    if let Some(dt) = pack.load_decision_trees()? {
        resources.push(McpResource {
            uri_template: format!("{uri_scheme}://{pack_slug}/decision-trees/{{id}}"),
            resource_type: "dkp/decision-trees".to_string(),
            description: "Decision trees".to_string(),
            count: dt.trees.len(),
        });
    }

    if pack.has_knowledge_graph() {
        resources.push(McpResource {
            uri_template: format!("{uri_scheme}://{pack_slug}/graph"),
            resource_type: "dkp/graph".to_string(),
            description: "Knowledge graph".to_string(),
            count: 1,
        });
    }

    if pack.load_system_prompt()?.is_some() {
        resources.push(McpResource {
            uri_template: format!("{uri_scheme}://{pack_slug}/system-prompt"),
            resource_type: "dkp/system-prompt".to_string(),
            description: "System prompt".to_string(),
            count: 1,
        });
    }

    let expose_eval = pack
        .manifest
        .mcp
        .as_ref()
        .and_then(|m| m.resource_server.as_ref())
        .and_then(|rs| rs.expose_eval_cases)
        .unwrap_or(false);

    if expose_eval && pack.has_eval_set() {
        let n = pack.load_eval_set()?.len();
        resources.push(McpResource {
            uri_template: format!("{uri_scheme}://{pack_slug}/eval-cases/{{hash}}"),
            resource_type: "dkp/eval-cases".to_string(),
            description: "Eval cases (exposed via mcp.resource_server.expose_eval_cases)"
                .to_string(),
            count: n,
        });
    }

    // Determine which tools to expose
    let allowed_tools: Option<Vec<String>> = pack
        .manifest
        .mcp
        .as_ref()
        .and_then(|m| m.tool_provider.as_ref())
        .map(|tp| tp.tools.clone());

    let all_tools = vec![
        McpTool {
            name: "dkp_inject".to_string(),
            description: "Return the system prompt and optional context block for this pack"
                .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "audience": { "type": "string", "description": "Audience profile ID to filter content" },
                    "include_context_block": { "type": "boolean", "default": true },
                    "max_tokens": { "type": "integer", "description": "Token budget cap" }
                }
            }),
        },
        McpTool {
            name: "dkp_search".to_string(),
            description:
                "BM25 full-text search over all machine-layer assets; returns ranked results"
                    .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["query"],
                "properties": {
                    "query": { "type": "string" },
                    "limit": { "type": "integer", "default": 10 },
                    "asset_types": { "type": "array", "items": { "type": "string" } }
                }
            }),
        },
        McpTool {
            name: "dkp_chunk".to_string(),
            description: "Retrieve top-N retrieval chunks for a query string".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["query"],
                "properties": {
                    "query": { "type": "string" },
                    "top_n": { "type": "integer", "default": 5 }
                }
            }),
        },
        McpTool {
            name: "dkp_get".to_string(),
            description: "Fetch a specific asset by resource type and ID".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["resource_type", "id"],
                "properties": {
                    "resource_type": { "type": "string" },
                    "id": { "type": "string" }
                }
            }),
        },
        McpTool {
            name: "dkp_validate".to_string(),
            description: "Run quality gates and return structured results".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "gates": { "type": "array", "items": { "type": "string" } }
                }
            }),
        },
        McpTool {
            name: "dkp_eval".to_string(),
            description:
                "Run the eval set against a provider model (requires expose_eval_cases: true)"
                    .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "model": { "type": "string" },
                    "limit": { "type": "integer" }
                }
            }),
        },
    ];

    let tools: Vec<McpTool> = match &allowed_tools {
        Some(names) => all_tools
            .into_iter()
            .filter(|t| names.contains(&t.name))
            .collect(),
        None => all_tools,
    };

    let manifest = McpManifest {
        pack_name: pack.manifest.name.clone(),
        pack_version: pack.manifest.version.clone(),
        uri_scheme,
        resources,
        tools,
    };

    let json = serde_json::to_string_pretty(&manifest)?;

    if args.dry_run {
        println!("{json}");
        return Ok(());
    }

    let out_path = match args.out {
        Some(p) => p,
        None => pack.machine_dir().join("mcp_manifest.json"),
    };

    std::fs::write(&out_path, &json)?;
    println!("Written: {}", out_path.display());

    Ok(())
}

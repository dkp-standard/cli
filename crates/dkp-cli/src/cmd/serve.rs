use anyhow::Result;
use clap::Args;
use rmcp::{
    model::{
        Annotated, CallToolRequestParam, CallToolResult, Content, Implementation,
        ListResourcesResult, ListToolsResult, PaginatedRequestParam, RawResource,
        ReadResourceRequestParam, ReadResourceResult, Resource, ResourceContents,
        ResourcesCapability, ServerCapabilities, ServerInfo, Tool, ToolsCapability,
    },
    service::RequestContext,
    transport::sse_server::SseServer,
    ServerHandler, ServiceExt,
};
use std::{path::PathBuf, sync::Arc};

#[cfg(feature = "procedures")]
use dkp_core::procedures;
use dkp_core::{search::SearchIndex, Pack};

use crate::cli::CmdCtx;
use crate::cmd::get::get_assets;
use crate::cmd::inject::{build_inject_content, wrap_inject_output};

#[derive(Args, Debug)]
pub struct ServeArgs {
    /// Path to the DKP bundle directory
    pub pack: PathBuf,

    /// Transport mechanism: stdio | http
    #[arg(long, default_value = "stdio", value_name = "TRANSPORT")]
    pub transport: String,

    /// HTTP server port (when --transport http)
    #[arg(long, default_value = "8734")]
    pub port: u16,

    /// Expose resources only; do not expose any tools
    #[arg(long)]
    pub readonly: bool,

    /// Static bearer token for authentication
    #[arg(long, value_name = "TOKEN")]
    pub auth_token: Option<String>,

    /// Server log verbosity: debug | info | warn
    #[arg(long, default_value = "info", value_name = "LEVEL")]
    pub log_level: String,

    /// Allow running non-WASM procedures from unsigned bundles (dev/testing only)
    #[arg(long)]
    pub allow_unsigned: bool,
}

struct LoadedResource {
    uri: String,
    name: String,
    description: String,
    content: String,
}

#[derive(Clone)]
struct DkpServer {
    pack_name: String,
    pack_version: String,
    slug: String,
    pack: Arc<Pack>,
    resources: Arc<Vec<LoadedResource>>,
    index: Arc<SearchIndex>,
    readonly: bool,
    allow_unsigned: bool,
}

impl DkpServer {
    fn new(pack: Pack, readonly: bool, allow_unsigned: bool) -> Result<Self> {
        let manifest = &pack.manifest;
        let pack_name = manifest.name.clone();
        let pack_version = manifest.version.clone();
        let slug = pack_name.to_lowercase().replace(' ', "-");

        let mut resources: Vec<LoadedResource> = Vec::new();

        // System prompt
        match pack.load_system_prompt() {
            Ok(Some(prompt)) => {
                resources.push(LoadedResource {
                    uri: format!("dkp://{}/system-prompt", slug),
                    name: "System Prompt".into(),
                    description: "System-level instructions for LLM consumers".into(),
                    content: prompt,
                });
            }
            Ok(None) => {}
            Err(e) => eprintln!("Warning: could not load system prompt: {e}"),
        }

        // Retrieval chunks
        match pack.load_chunks() {
            Ok(chunks) => {
                for chunk in chunks {
                    resources.push(LoadedResource {
                        uri: format!("dkp://{}/chunk/{}", slug, chunk.id),
                        name: chunk.title.clone(),
                        description: format!("Retrieval chunk from {}", chunk.source_ref),
                        content: chunk.chunk_text,
                    });
                }
            }
            Err(e) => eprintln!("Warning: could not load chunks: {e}"),
        }

        // Glossary terms
        match pack.load_glossary() {
            Ok(Some(glossary)) => {
                for term in glossary.terms {
                    resources.push(LoadedResource {
                        uri: format!("dkp://{}/term/{}", slug, term.id),
                        name: term.term.clone(),
                        description: format!("Glossary definition for '{}'", term.term),
                        content: term.definition,
                    });
                }
            }
            Ok(None) => {}
            Err(e) => eprintln!("Warning: could not load glossary: {e}"),
        }

        // Rules
        match pack.load_rules() {
            Ok(Some(rules_file)) => {
                for rule in rules_file.rules {
                    resources.push(LoadedResource {
                        uri: format!("dkp://{}/rule/{}", slug, rule.id),
                        name: rule.title.clone(),
                        description: format!("Rule: {}", rule.title),
                        content: rule.description,
                    });
                }
            }
            Ok(None) => {}
            Err(e) => eprintln!("Warning: could not load rules: {e}"),
        }

        // Constraints
        match pack.load_constraints() {
            Ok(Some(constraints_file)) => {
                for constraint in constraints_file.all_constraints() {
                    resources.push(LoadedResource {
                        uri: format!("dkp://{}/constraint/{}", slug, constraint.id),
                        name: constraint.title.clone(),
                        description: format!("Constraint: {}", constraint.title),
                        content: constraint.description.clone(),
                    });
                }
            }
            Ok(None) => {}
            Err(e) => eprintln!("Warning: could not load constraints: {e}"),
        }

        let index = SearchIndex::build(&pack)?;
        let pack = Arc::new(pack);

        Ok(DkpServer {
            pack_name,
            pack_version,
            slug,
            pack,
            resources: Arc::new(resources),
            index: Arc::new(index),
            readonly,
            allow_unsigned,
        })
    }

    fn tool_definitions(&self) -> Vec<Tool> {
        let mut tools = vec![
            Tool::new(
                "inject",
                "Return a formatted context block from this pack for injection into an LLM prompt",
                Arc::new(rmcp::model::object(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "scope": {
                            "type": "string",
                            "enum": ["system-prompt", "full", "minimal", "chunks"],
                            "default": "system-prompt",
                            "description": "Content scope: system-prompt (default), full, minimal, or chunks"
                        },
                        "format": {
                            "type": "string",
                            "enum": ["markdown", "xml", "json"],
                            "default": "markdown",
                            "description": "Wrapping format: markdown (default), xml, or json"
                        },
                        "max_tokens": {
                            "type": "integer",
                            "description": "Truncate output to fit within this token budget"
                        }
                    }
                }))),
            ),
            Tool::new(
                "search",
                "Full-text search across all pack assets (chunks, terms, rules, constraints)",
                Arc::new(rmcp::model::object(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "description": "Full-text search query" },
                        "limit": { "type": "integer", "default": 10, "description": "Max results to return" }
                    },
                    "required": ["query"]
                }))),
            ),
            Tool::new(
                "chunk",
                "Retrieve a specific retrieval chunk by its ID",
                Arc::new(rmcp::model::object(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "id": { "type": "string", "description": "Chunk ID to retrieve" }
                    },
                    "required": ["id"]
                }))),
            ),
            Tool::new(
                "get",
                "Fetch assets from the pack by type, optionally filtered by ID or title",
                Arc::new(rmcp::model::object(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "asset_type": {
                            "type": "string",
                            "enum": ["term", "rule", "chunk", "constraint", "entity", "eval", "graph", "cross-ref", "system-prompt"],
                            "description": "Asset type to retrieve"
                        },
                        "id": {
                            "type": "string",
                            "description": "Asset ID or title substring to filter by (omit to get all assets of this type)"
                        },
                        "by_id": {
                            "type": "boolean",
                            "default": false,
                            "description": "When true, match id exactly instead of substring-matching on title"
                        }
                    },
                    "required": ["asset_type"]
                }))),
            ),
        ];

        #[cfg(feature = "procedures")]
        if self.pack.has_procedures() {
            tools.push(Tool::new(
                "list_procedures",
                "List all available procedures in this pack, including their IDs, titles, descriptions, and whether a WASM binary is present.",
                Arc::new(rmcp::model::object(serde_json::json!({
                    "type": "object",
                    "properties": {}
                }))),
            ));
            tools.push(Tool::new(
                "run_procedure",
                "Execute a WASM procedure from this pack by ID. Input and output are JSON objects.",
                Arc::new(rmcp::model::object(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "procedure_id": { "type": "string", "description": "Procedure ID (stem name, e.g. \"file-chapter-7\")" },
                        "input":        { "type": "object", "description": "JSON input passed to the procedure on stdin (default: {})" },
                        "timeout_ms":   { "type": "integer", "description": "Wall-clock timeout override in milliseconds" }
                    },
                    "required": ["procedure_id"]
                }))),
            ));
        }

        tools
    }
}

impl ServerHandler for DkpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: Implementation {
                name: self.pack_name.clone(),
                version: self.pack_version.clone(),
            },
            capabilities: ServerCapabilities {
                resources: Some(ResourcesCapability::default()),
                tools: if self.readonly {
                    None
                } else {
                    Some(ToolsCapability::default())
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn list_resources(
        &self,
        _request: PaginatedRequestParam,
        _context: RequestContext<rmcp::service::RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListResourcesResult, rmcp::Error>> + Send + '_
    {
        let resources: Vec<Resource> = self
            .resources
            .iter()
            .map(|r| {
                Annotated::new(
                    RawResource {
                        uri: r.uri.clone(),
                        name: r.name.clone(),
                        description: Some(r.description.clone()),
                        mime_type: Some("text/plain".into()),
                        size: None,
                    },
                    None,
                )
            })
            .collect();
        std::future::ready(Ok(ListResourcesResult {
            resources,
            next_cursor: None,
        }))
    }

    fn read_resource(
        &self,
        request: ReadResourceRequestParam,
        _context: RequestContext<rmcp::service::RoleServer>,
    ) -> impl std::future::Future<Output = Result<ReadResourceResult, rmcp::Error>> + Send + '_
    {
        let result = self
            .resources
            .iter()
            .find(|r| r.uri == request.uri)
            .map(|r| ReadResourceResult {
                contents: vec![ResourceContents::text(r.content.clone(), r.uri.clone())],
            })
            .ok_or_else(|| {
                rmcp::Error::invalid_params(format!("resource not found: {}", request.uri), None)
            });
        std::future::ready(result)
    }

    fn list_tools(
        &self,
        _request: PaginatedRequestParam,
        _context: RequestContext<rmcp::service::RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListToolsResult, rmcp::Error>> + Send + '_ {
        let tools = if self.readonly {
            vec![]
        } else {
            self.tool_definitions()
        };
        std::future::ready(Ok(ListToolsResult {
            tools,
            next_cursor: None,
        }))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        _context: RequestContext<rmcp::service::RoleServer>,
    ) -> Result<CallToolResult, rmcp::Error> {
        if self.readonly {
            return Err(rmcp::Error::invalid_params(
                "server is in readonly mode — tools are disabled",
                None,
            ));
        }

        let args = request.arguments.unwrap_or_default();

        match request.name.as_ref() {
            "inject" => {
                let scope = args
                    .get("scope")
                    .and_then(|v| v.as_str())
                    .unwrap_or("system-prompt")
                    .to_string();
                let format = args
                    .get("format")
                    .and_then(|v| v.as_str())
                    .unwrap_or("markdown")
                    .to_string();
                let max_tokens = args
                    .get("max_tokens")
                    .and_then(|v| v.as_u64())
                    .map(|n| n as u32);

                let pack = Arc::clone(&self.pack);
                let result = tokio::task::spawn_blocking(move || {
                    build_inject_content(&pack, &scope, max_tokens)
                        .and_then(|content| wrap_inject_output(&pack, &scope, &format, content))
                })
                .await
                .map_err(|e| rmcp::Error::internal_error(format!("task panicked: {e}"), None))?;

                match result {
                    Ok(output) => Ok(CallToolResult::success(vec![Content::text(output)])),
                    Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                }
            }

            "search" => {
                let query = args.get("query").and_then(|v| v.as_str()).ok_or_else(|| {
                    rmcp::Error::invalid_params("missing required argument: query", None)
                })?;
                let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(10) as usize;

                let results = self.index.search(query, limit).map_err(|e| {
                    rmcp::Error::internal_error(format!("search failed: {e}"), None)
                })?;

                let json = serde_json::to_string_pretty(&results).map_err(|e| {
                    rmcp::Error::internal_error(format!("serialization failed: {e}"), None)
                })?;

                Ok(CallToolResult::success(vec![Content::text(json)]))
            }

            "chunk" => {
                let id = args.get("id").and_then(|v| v.as_str()).ok_or_else(|| {
                    rmcp::Error::invalid_params("missing required argument: id", None)
                })?;

                let uri = format!("dkp://{}/chunk/{}", self.slug, id);
                let resource = self
                    .resources
                    .iter()
                    .find(|r| r.uri == uri)
                    .ok_or_else(|| {
                        rmcp::Error::invalid_params(format!("chunk not found: {id}"), None)
                    })?;

                Ok(CallToolResult::success(vec![Content::text(
                    resource.content.clone(),
                )]))
            }

            "get" => {
                let asset_type = args
                    .get("asset_type")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        rmcp::Error::invalid_params("missing required argument: asset_type", None)
                    })?
                    .to_string();
                let id = args.get("id").and_then(|v| v.as_str()).map(String::from);
                let by_id = args.get("by_id").and_then(|v| v.as_bool()).unwrap_or(false);

                let pack = Arc::clone(&self.pack);
                let result = tokio::task::spawn_blocking(move || {
                    get_assets(&pack, &asset_type, id.as_deref(), by_id)
                })
                .await
                .map_err(|e| rmcp::Error::internal_error(format!("task panicked: {e}"), None))?;

                match result {
                    Ok(items) => {
                        let json = serde_json::to_string_pretty(&items)
                            .map_err(|e| rmcp::Error::internal_error(e.to_string(), None))?;
                        Ok(CallToolResult::success(vec![Content::text(json)]))
                    }
                    Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                }
            }

            #[cfg(feature = "procedures")]
            "list_procedures" => {
                let pack = Arc::clone(&self.pack);
                let result = tokio::task::spawn_blocking(move || {
                    procedures::list(&pack).map_err(|e| e.to_string())
                })
                .await
                .map_err(|e| rmcp::Error::internal_error(format!("task panicked: {e}"), None))?;

                match result {
                    Ok(defs) => {
                        let summary: Vec<serde_json::Value> = defs
                            .iter()
                            .map(|d| {
                                serde_json::json!({
                                    "id":          d.id,
                                    "title":       d.schema.title,
                                    "description": d.schema.description,
                                    "has_wasm":    d.wasm_path.is_some(),
                                })
                            })
                            .collect();
                        let json = serde_json::to_string_pretty(&summary)
                            .map_err(|e| rmcp::Error::internal_error(e.to_string(), None))?;
                        Ok(CallToolResult::success(vec![Content::text(json)]))
                    }
                    Err(msg) => Ok(CallToolResult::error(vec![Content::text(msg)])),
                }
            }

            #[cfg(feature = "procedures")]
            "run_procedure" => {
                let procedure_id = args
                    .get("procedure_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        rmcp::Error::invalid_params("missing required argument: procedure_id", None)
                    })?
                    .to_string();

                let input = args
                    .get("input")
                    .cloned()
                    .unwrap_or(serde_json::Value::Object(Default::default()));

                let timeout_ms = args.get("timeout_ms").and_then(|v| v.as_u64());

                let pack = Arc::clone(&self.pack);
                let allow_unsigned = self.allow_unsigned;
                let result = tokio::task::spawn_blocking(move || {
                    let defs = procedures::list(&pack)
                        .map_err(|e| format!("failed to list procedures: {e}"))?;
                    let def = defs
                        .into_iter()
                        .find(|d| d.id == procedure_id)
                        .ok_or_else(|| format!("procedure not found: {procedure_id}"))?;
                    let opts = procedures::executor::RunOptions {
                        input,
                        timeout_ms,
                        allow_unsigned,
                    };
                    procedures::executor::run(&pack, &def, opts).map_err(|e| e.to_string())
                })
                .await
                .map_err(|e| rmcp::Error::internal_error(format!("task panicked: {e}"), None))?;

                match result {
                    Ok(output) => {
                        let json = serde_json::to_string_pretty(&output)
                            .map_err(|e| rmcp::Error::internal_error(e.to_string(), None))?;
                        Ok(CallToolResult::success(vec![Content::text(json)]))
                    }
                    Err(msg) => Ok(CallToolResult::error(vec![Content::text(msg)])),
                }
            }

            other => Err(rmcp::Error::invalid_params(
                format!("unknown tool: {other}"),
                None,
            )),
        }
    }
}

pub async fn run(args: ServeArgs, _cli: &CmdCtx) -> Result<()> {
    let pack_dir = args.pack.canonicalize()?;
    let pack = Pack::open(&pack_dir)?;

    eprintln!("DKP MCP Server");
    eprintln!(
        "  Pack:      {} v{}",
        pack.manifest.name, pack.manifest.version
    );
    eprintln!("  Transport: {}", args.transport);
    eprintln!("  Readonly:  {}", args.readonly);

    if args.auth_token.is_some() && args.transport != "http" {
        eprintln!(
            "Warning: --auth-token has no effect in {} transport",
            args.transport
        );
    }
    if args.auth_token.is_some() && args.transport == "http" {
        eprintln!(
            "Warning: --auth-token is not enforced by the SSE transport layer; use a reverse proxy for bearer auth"
        );
    }

    let server = DkpServer::new(pack, args.readonly, args.allow_unsigned)?;
    eprintln!("  Resources: {} loaded", server.resources.len());

    match args.transport.as_str() {
        "stdio" => {
            eprintln!("Listening on stdio...");
            let transport = (tokio::io::stdin(), tokio::io::stdout());
            let service = server.serve(transport).await?;
            service.waiting().await?;
        }
        "http" => {
            let addr = std::net::SocketAddr::from(([0, 0, 0, 0], args.port));
            eprintln!("Listening on http://{}  (SSE transport)", addr);
            eprintln!("  SSE endpoint:  http://{}/sse", addr);
            eprintln!("  POST endpoint: http://{}/message", addr);
            let sse = SseServer::serve(addr).await?;
            let ct = sse.with_service(move || server.clone());
            ct.cancelled().await;
        }
        other => {
            anyhow::bail!("unknown transport '{}' — supported: stdio, http", other);
        }
    }

    Ok(())
}

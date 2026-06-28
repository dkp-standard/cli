use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    // Required
    pub spec: String,
    pub name: String,
    pub version: String,
    pub domain: String,
    pub audience: String,
    pub intended_use: String,
    pub known_limitations: String,
    pub update_date: String,
    #[serde(default)]
    pub compatibility: Vec<String>,

    // Recommended
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audience_profiles: Vec<AudienceProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_hints: Option<RetrievalHints>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_eval_delta: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locales: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_locale: Option<String>,

    // Registry fields (optional; required only for dkp publish)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Author>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub maintainers: Vec<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bugs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,

    // Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Publisher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control: Option<AccessControl>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<Dependency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedure_capabilities: Option<ProcedureCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp: Option<McpConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudienceProfile {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalHints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_top_k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_context_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_reranker: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Publisher {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pgp_fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    #[serde(default)]
    pub required_roles: Vec<String>,
    #[serde(default)]
    pub export_restrictions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pii_present: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gdpr_scope: Option<bool>,
    #[serde(default)]
    pub mcp_scopes_required: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_audience: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[serde(default)]
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_runtime_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_access: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesystem_access: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_server: Option<McpResourceServer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_provider: Option<McpToolProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResourceServer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri_scheme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expose_eval_cases: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolProvider {
    #[serde(default)]
    pub tools: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<McpAuth>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpAuth {
    pub scheme: String,
}

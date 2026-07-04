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

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal_manifest() -> Manifest {
        serde_json::from_str(
            r#"{
                "spec": "1.0.0",
                "name": "test-pack",
                "version": "1.0.0",
                "domain": "testing",
                "audience": "internal",
                "intended_use": "unit tests",
                "known_limitations": "none",
                "update_date": "2026-01-01"
            }"#,
        )
        .unwrap()
    }

    #[test]
    fn minimal_manifest_round_trips() {
        let m = minimal_manifest();
        let json = serde_json::to_string(&m).unwrap();
        let back: Manifest = serde_json::from_str(&json).unwrap();
        assert_eq!(back.name, m.name);
        assert_eq!(back.domain, m.domain);
        assert_eq!(back.version, m.version);
    }

    #[test]
    fn minimal_manifest_omits_optional_fields() {
        let m = minimal_manifest();
        let json = serde_json::to_string(&m).unwrap();
        assert!(!json.contains("\"description\""));
        assert!(!json.contains("\"license\""));
        assert!(!json.contains("\"mcp\""));
    }

    #[test]
    fn full_manifest_round_trips() {
        let mut m = minimal_manifest();
        m.description = Some("A test pack".to_string());
        m.tags = vec!["a".to_string(), "b".to_string()];
        m.license = Some("MIT".to_string());
        m.min_eval_delta = Some(0.1);
        m.author = Some(Author {
            name: "Jane".to_string(),
            email: Some("jane@example.com".to_string()),
            url: None,
        });
        m.mcp = Some(McpConfig {
            resource_server: None,
            tool_provider: Some(McpToolProvider {
                tools: vec!["tool1".to_string()],
                auth: Some(McpAuth {
                    scheme: "bearer".to_string(),
                }),
            }),
            transport: Some("stdio".to_string()),
        });

        let json = serde_json::to_string(&m).unwrap();
        let back: Manifest = serde_json::from_str(&json).unwrap();
        assert_eq!(back.description, m.description);
        assert_eq!(back.tags, m.tags);
        assert_eq!(back.min_eval_delta, m.min_eval_delta);
        assert_eq!(back.author.unwrap().name, "Jane");
        assert_eq!(
            back.mcp.unwrap().tool_provider.unwrap().tools,
            vec!["tool1".to_string()]
        );
    }
}

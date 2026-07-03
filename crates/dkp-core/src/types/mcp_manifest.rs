use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpManifest {
    pub pack_name: String,
    pub pack_version: String,
    pub uri_scheme: String,
    pub resources: Vec<McpResource>,
    pub tools: Vec<McpTool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResource {
    pub uri_template: String,
    pub resource_type: String,
    pub description: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mcp_manifest_round_trips() {
        let m = McpManifest {
            pack_name: "test-pack".to_string(),
            pack_version: "1.0.0".to_string(),
            uri_scheme: "dkp".to_string(),
            resources: vec![McpResource {
                uri_template: "dkp://{id}".to_string(),
                resource_type: "term".to_string(),
                description: "Glossary term".to_string(),
                count: 3,
            }],
            tools: vec![McpTool {
                name: "search".to_string(),
                description: "Search the pack".to_string(),
                input_schema: serde_json::json!({"type": "object"}),
            }],
        };
        let json = serde_json::to_string(&m).unwrap();
        let back: McpManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(back.pack_name, "test-pack");
        assert_eq!(back.resources[0].count, 3);
        assert_eq!(back.tools[0].name, "search");
    }
}

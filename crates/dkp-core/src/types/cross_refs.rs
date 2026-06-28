use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossRefsFile {
    pub cross_refs: Vec<CrossRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossRef {
    pub pack_name: String,
    pub pack_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    pub local_id: String,
    pub remote_id: String,
    pub relation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

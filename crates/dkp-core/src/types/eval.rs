use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalCase {
    pub query: String,
    pub expected_dimensions: Vec<String>,
    pub critical_must_include: Vec<String>,
    pub scoring_rubric: String,
    pub version_meta: EvalVersionMeta,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audience: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalVersionMeta {
    pub prompt_hash: String,
    pub model_version: String,
    pub dataset_version: String,
}

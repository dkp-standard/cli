use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintsFile {
    pub edge_cases: Vec<Constraint>,
    pub anti_patterns: Vec<Constraint>,
    pub hard_limits: Vec<Constraint>,
}

impl ConstraintsFile {
    pub fn all_constraints(&self) -> impl Iterator<Item = &Constraint> {
        self.edge_cases
            .iter()
            .chain(self.anti_patterns.iter())
            .chain(self.hard_limits.iter())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub id: String,
    pub title: String,
    pub description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stability: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audience: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_days: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_date: Option<String>,
}

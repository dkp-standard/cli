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

#[cfg(test)]
mod tests {
    use super::*;

    fn constraint(id: &str) -> Constraint {
        Constraint {
            id: id.to_string(),
            title: "Title".to_string(),
            description: "Description".to_string(),
            tags: vec![],
            source_ref: None,
            stability: None,
            audience: vec![],
            ttl_days: None,
            review_date: None,
        }
    }

    #[test]
    fn constraints_file_round_trips() {
        let file = ConstraintsFile {
            edge_cases: vec![constraint("edge-1")],
            anti_patterns: vec![constraint("anti-1")],
            hard_limits: vec![constraint("limit-1")],
        };
        let json = serde_json::to_string(&file).unwrap();
        let back: ConstraintsFile = serde_json::from_str(&json).unwrap();
        assert_eq!(back.edge_cases[0].id, "edge-1");
        assert_eq!(back.anti_patterns[0].id, "anti-1");
        assert_eq!(back.hard_limits[0].id, "limit-1");
    }

    #[test]
    fn all_constraints_chains_all_three_categories() {
        let file = ConstraintsFile {
            edge_cases: vec![constraint("edge-1")],
            anti_patterns: vec![constraint("anti-1")],
            hard_limits: vec![constraint("limit-1")],
        };
        let ids: Vec<&str> = file.all_constraints().map(|c| c.id.as_str()).collect();
        assert_eq!(ids, vec!["edge-1", "anti-1", "limit-1"]);
    }
}

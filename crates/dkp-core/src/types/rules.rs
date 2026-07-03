use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesFile {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub title: String,
    pub description: String,
    pub polarity: RulePolarity,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audience: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_days: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stability: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RulePolarity {
    Affirmative,
    Prohibitive,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules_file_round_trips() {
        let file = RulesFile {
            rules: vec![Rule {
                id: "r1".to_string(),
                title: "Rule Title".to_string(),
                description: "Do this".to_string(),
                polarity: RulePolarity::Affirmative,
                tags: vec![],
                source_ref: None,
                confidence: Some(0.9),
                audience: vec![],
                ttl_days: None,
                review_date: None,
                stability: None,
            }],
        };
        let json = serde_json::to_string(&file).unwrap();
        let back: RulesFile = serde_json::from_str(&json).unwrap();
        assert_eq!(back.rules[0].id, "r1");
        assert!(matches!(back.rules[0].polarity, RulePolarity::Affirmative));
    }

    #[test]
    fn rule_polarity_snake_case_serde() {
        assert_eq!(
            serde_json::to_string(&RulePolarity::Prohibitive).unwrap(),
            "\"prohibitive\""
        );
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlossaryFile {
    pub terms: Vec<GlossaryTerm>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlossaryTerm {
    pub id: String,
    pub term: String,
    pub definition: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audience: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stability: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_days: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skos_broader: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skos_narrower: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_org_type: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glossary_file_round_trips() {
        let file = GlossaryFile {
            terms: vec![GlossaryTerm {
                id: "t1".to_string(),
                term: "Term".to_string(),
                definition: "A definition".to_string(),
                aliases: vec!["alias".to_string()],
                related: vec![],
                tags: vec![],
                source_ref: Some("generated".to_string()),
                audience: vec![],
                stability: None,
                ttl_days: None,
                review_date: None,
                skos_broader: None,
                skos_narrower: None,
                schema_org_type: None,
            }],
        };
        let json = serde_json::to_string(&file).unwrap();
        let back: GlossaryFile = serde_json::from_str(&json).unwrap();
        assert_eq!(back.terms[0].id, "t1");
        assert_eq!(back.terms[0].aliases, vec!["alias".to_string()]);
    }
}

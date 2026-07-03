use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologyFile {
    pub entity_types: Vec<EntityType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityType {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub attributes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relationships: Vec<Relationship>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_org_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audience: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub name: String,
    pub target_type: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub cardinality: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ontology_file_round_trips() {
        let file = OntologyFile {
            entity_types: vec![EntityType {
                id: "e1".to_string(),
                name: "Entity".to_string(),
                description: "An entity".to_string(),
                attributes: vec!["attr1".to_string()],
                relationships: vec![Relationship {
                    name: "related_to".to_string(),
                    target_type: "e2".to_string(),
                    cardinality: "many".to_string(),
                    description: None,
                }],
                schema_org_type: None,
                audience: vec![],
            }],
        };
        let json = serde_json::to_string(&file).unwrap();
        let back: OntologyFile = serde_json::from_str(&json).unwrap();
        assert_eq!(back.entity_types[0].id, "e1");
        assert_eq!(back.entity_types[0].relationships[0].target_type, "e2");
    }
}

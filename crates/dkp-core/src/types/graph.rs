use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    pub nodes: Vec<KgNode>,
    pub edges: Vec<KgEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KgNode {
    pub id: String,
    pub node_type: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KgEdge {
    pub source: String,
    pub relation: KgRelation,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum KgRelation {
    Requires,
    Contradicts,
    Elaborates,
    Supersedes,
    PartOf,
    DependsOn,
    SeeAlso,
    MeasuredBy,
    DefinedBy,
    Specializes,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn knowledge_graph_round_trips() {
        let graph = KnowledgeGraph {
            nodes: vec![KgNode {
                id: "n1".to_string(),
                node_type: "concept".to_string(),
                label: "Node One".to_string(),
                description: None,
            }],
            edges: vec![KgEdge {
                source: "n1".to_string(),
                relation: KgRelation::SeeAlso,
                target: "n1".to_string(),
                weight: Some(0.5),
                description: None,
            }],
        };
        let json = serde_json::to_string(&graph).unwrap();
        let back: KnowledgeGraph = serde_json::from_str(&json).unwrap();
        assert_eq!(back.nodes[0].id, "n1");
        assert!(matches!(back.edges[0].relation, KgRelation::SeeAlso));
    }

    #[test]
    fn kg_relation_kebab_case_serde() {
        assert_eq!(
            serde_json::to_string(&KgRelation::PartOf).unwrap(),
            "\"part-of\""
        );
        assert_eq!(
            serde_json::to_string(&KgRelation::DependsOn).unwrap(),
            "\"depends-on\""
        );
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalChunk {
    pub id: String,
    pub title: String,
    pub chunk_text: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub source_ref: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_priority: Option<RetrievalPriority>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub asset_refs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_days: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stability: Option<Stability>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audience: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RetrievalPriority {
    Critical,
    High,
    Normal,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Stability {
    Stable,
    Volatile,
    Experimental,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_chunk_round_trips() {
        let chunk = RetrievalChunk {
            id: "c1".to_string(),
            title: "Title".to_string(),
            chunk_text: "Body text".to_string(),
            tags: vec![],
            source_ref: "generated".to_string(),
            confidence: None,
            summary: None,
            embedding_model: None,
            token_count: None,
            retrieval_priority: None,
            asset_refs: vec![],
            ttl_days: None,
            review_date: None,
            stability: None,
            audience: vec![],
        };
        let json = serde_json::to_string(&chunk).unwrap();
        let back: RetrievalChunk = serde_json::from_str(&json).unwrap();
        assert_eq!(back.id, chunk.id);
        assert_eq!(back.chunk_text, chunk.chunk_text);
    }

    #[test]
    fn retrieval_priority_lowercase_serde() {
        let json = serde_json::to_string(&RetrievalPriority::Critical).unwrap();
        assert_eq!(json, "\"critical\"");
        let back: RetrievalPriority = serde_json::from_str("\"high\"").unwrap();
        assert!(matches!(back, RetrievalPriority::High));
    }

    #[test]
    fn stability_lowercase_serde() {
        let json = serde_json::to_string(&Stability::Experimental).unwrap();
        assert_eq!(json, "\"experimental\"");
    }
}

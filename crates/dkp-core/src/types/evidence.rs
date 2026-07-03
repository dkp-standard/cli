use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceRecord {
    pub id: String,
    pub title: String,
    pub url: String,
    pub retrieved_date: String,
    pub license: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[cfg(test)]
mod source_record_tests {
    use super::*;

    #[test]
    fn source_record_round_trips() {
        let s = SourceRecord {
            id: "src-1".to_string(),
            title: "Some Source".to_string(),
            url: "https://example.com".to_string(),
            retrieved_date: "2026-01-01".to_string(),
            license: "CC-BY".to_string(),
            author: None,
            publisher: None,
            notes: None,
        };
        let json = serde_json::to_string(&s).unwrap();
        let back: SourceRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(back.id, "src-1");
        assert_eq!(back.url, "https://example.com");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RightsRecord {
    pub source_id: String,
    pub rights_holder: String,
    pub license_type: String,
    pub granted_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commercial_use: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[cfg(test)]
mod rights_record_tests {
    use super::*;

    #[test]
    fn rights_record_round_trips() {
        let r = RightsRecord {
            source_id: "src-1".to_string(),
            rights_holder: "Author".to_string(),
            license_type: "CC-BY".to_string(),
            granted_date: "2026-01-01".to_string(),
            expiry_date: None,
            commercial_use: Some(true),
            attribution_required: Some(true),
            notes: None,
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: RightsRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(back.source_id, "src-1");
        assert_eq!(back.commercial_use, Some(true));
    }
}

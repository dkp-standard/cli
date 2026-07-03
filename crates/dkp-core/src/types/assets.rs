use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetsFile {
    pub assets: Vec<Asset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub asset_type: AssetType,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audience: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetType {
    Image,
    Table,
    Audio,
    Video,
    Document,
    Other,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assets_file_round_trips() {
        let file = AssetsFile {
            assets: vec![Asset {
                id: "a1".to_string(),
                asset_type: AssetType::Image,
                path: "images/a1.png".to_string(),
                caption: Some("A picture".to_string()),
                alt_text: None,
                source_ref: None,
                tags: vec![],
                audience: vec![],
            }],
        };
        let json = serde_json::to_string(&file).unwrap();
        let back: AssetsFile = serde_json::from_str(&json).unwrap();
        assert_eq!(back.assets.len(), 1);
        assert_eq!(back.assets[0].id, "a1");
    }

    #[test]
    fn asset_type_snake_case_serde() {
        assert_eq!(
            serde_json::to_string(&AssetType::Document).unwrap(),
            "\"document\""
        );
    }
}

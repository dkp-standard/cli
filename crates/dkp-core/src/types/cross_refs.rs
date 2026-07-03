use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossRefsFile {
    pub cross_refs: Vec<CrossRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossRef {
    pub pack_name: String,
    pub pack_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    pub local_id: String,
    pub remote_id: String,
    pub relation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_refs_file_round_trips() {
        let file = CrossRefsFile {
            cross_refs: vec![CrossRef {
                pack_name: "other-pack".to_string(),
                pack_version: "1.0.0".to_string(),
                registry: None,
                local_id: "local-1".to_string(),
                remote_id: "remote-1".to_string(),
                relation: "see-also".to_string(),
                description: None,
            }],
        };
        let json = serde_json::to_string(&file).unwrap();
        let back: CrossRefsFile = serde_json::from_str(&json).unwrap();
        assert_eq!(back.cross_refs[0].pack_name, "other-pack");
        assert_eq!(back.cross_refs[0].remote_id, "remote-1");
    }
}

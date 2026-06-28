use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::manifest::Manifest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishRequest {
    pub tarball_url: String,
    pub manifest: Manifest,
    /// Raw checksums.json contents
    pub checksums: serde_json::Value,
    /// Base64-encoded Ed25519 detached signature
    pub bundle_sig: String,
    /// "public" or "private"
    pub visibility: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishResponse {
    pub name: String,
    pub version: String,
    pub conformance: String,
    pub published_at: String,
    pub validation_report: ValidationReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub gate_4: GateResult,
    pub gate_7: GateResult,
    pub gate_8: GateResult,
    pub reviewed_badge: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GateResult {
    Pass,
    Fail,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackVersionResponse {
    pub name: String,
    pub version: String,
    pub manifest: Manifest,
    pub checksums: serde_json::Value,
    pub bundle_sig: String,
    pub tarball_url: String,
    pub archive_format: String,
    pub publisher_public_key: String,
    pub published_at: String,
    pub conformance: String,
    pub visibility: String,
    pub yanked: bool,
    pub yank_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionSummary {
    pub version: String,
    pub published_at: String,
    pub conformance: String,
    pub yanked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yank_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionListResponse {
    pub name: String,
    pub versions: Vec<VersionSummary>,
    pub latest: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub name: String,
    pub version: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub domain: String,
    pub tags: Vec<String>,
    pub conformance: String,
    pub published_at: String,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub total: u64,
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockFile {
    pub lockfile_version: u32,
    pub resolved: HashMap<String, LockedPack>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedPack {
    pub version: String,
    pub tarball_url: String,
    pub archive_format: String,
    /// "sha256-<hex>" integrity string
    pub integrity: String,
}

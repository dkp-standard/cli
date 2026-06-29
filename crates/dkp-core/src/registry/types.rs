use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::manifest::Manifest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishRequest {
    pub manifest: Manifest,
    /// Raw checksums.json contents
    pub checksums: serde_json::Value,
    /// Base64-encoded Ed25519 detached signature
    pub bundle_sig: String,
    /// "public" or "private"
    pub visibility: String,
    /// Declared archive size in bytes — used for quota pre-check on the registry side.
    pub size_bytes: i64,
    /// Archive format: "zip", "tar.gz", or "tar.xz". .dkp files are sent as "tar.xz".
    pub archive_format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishResponse {
    pub name: String,
    pub version: String,
    pub conformance: String,
    pub published_at: String,
    pub validation_report: ValidationReport,
    /// Presigned PUT URL — upload the archive directly to this URL.
    pub upload_url: String,
    /// Object key in R2 — pass to the confirm endpoint.
    pub r2_key: String,
    /// ISO-8601 expiry of the presigned URL.
    pub upload_url_expires_at: String,
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
pub struct ConfirmPublishResponse {
    pub name: String,
    pub version: String,
    pub conformance: String,
    pub size_bytes: i64,
    pub validation_report: ValidationReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadUrlResponse {
    pub url: String,
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
    pub archive_format: String,
    /// "sha256-<hex>" integrity string over checksums.json
    pub integrity: String,
}

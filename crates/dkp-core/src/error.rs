use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DkpError {
    // Pack loading
    #[error("pack not found: {0}")]
    PackNotFound(PathBuf),

    #[error("manifest.json missing from pack at {0}")]
    ManifestMissing(PathBuf),

    #[error("manifest.json invalid: {reason}")]
    ManifestInvalid { reason: String },

    #[error("required field '{field}' missing or empty in manifest.json")]
    ManifestFieldMissing { field: &'static str },

    // Asset parsing
    #[error("failed to parse {asset}: {source}")]
    AssetParse {
        asset: String,
        #[source]
        source: serde_json::Error,
    },

    #[error("invalid JSONL in {file} at line {line}: {reason}")]
    JsonlParse {
        file: String,
        line: usize,
        reason: String,
    },

    #[error("schema validation failed for {asset}: {violations}")]
    SchemaValidation { asset: String, violations: String },

    // Search
    #[error("search index error: {0}")]
    SearchIndex(String),

    // Validation gates
    #[error("gate {gate} failed: {reason}")]
    GateFailed { gate: u8, reason: String },

    // OKF
    #[error("OKF frontmatter error in {file}: {reason}")]
    OkfFrontmatter { file: String, reason: String },

    #[error("broken OKF link in {file}: target '{target}' not found")]
    OkfBrokenLink { file: String, target: String },

    // Evidence
    #[error("source_ref '{source_ref}' in {asset} does not resolve to sources.csv")]
    UnresolvedSourceRef { source_ref: String, asset: String },

    // Archive
    #[error("archive error: {0}")]
    Archive(String),

    // Config
    #[error("config error: {0}")]
    Config(String),

    // Procedures
    #[error("procedure '{id}' not found in pack '{pack}'")]
    ProcedureNotFound { id: String, pack: String },

    #[error("procedure '{id}' has no executable (.wasm absent, no entry_point declared)")]
    ProcedureNoExecutable { id: String },

    #[error("procedure '{id}' exceeded {limit_ms}ms wall-clock timeout")]
    ProcedureTimeout { id: String, limit_ms: u64 },

    #[error("procedure '{id}' produced invalid JSON output: {reason}")]
    ProcedureInvalidOutput { id: String, reason: String },

    #[error("procedure '{id}' trapped during execution: {message}")]
    ProcedureTrap { id: String, message: String },

    #[error("cannot satisfy procedure_capabilities constraint '{constraint}' for '{id}'")]
    ProcedureConstraintUnsatisfied { id: String, constraint: String },

    #[error("procedure schema '{id}' is not valid JSON Schema: {message}")]
    ProcedureSchemaInvalid { id: String, message: String },

    #[error("procedure '{id}' is a non-WASM executable and cannot be run from an unsigned bundle (pass --allow-unsigned to override)")]
    ProcedureUnsignedSubprocess { id: String },

    // Registry
    #[error("registry error: {0}")]
    Registry(String),

    // Pass-through
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type DkpResult<T> = Result<T, DkpError>;

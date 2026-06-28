use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Mirrors {procedure-id}.schema.json (Appendix B §B.23).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureSchema {
    pub id: String,
    pub title: String,
    pub description: String,
    /// JSON Schema object describing input
    pub input: serde_json::Value,
    /// JSON Schema object describing output
    pub output: serde_json::Value,
    /// Present only for non-WASM procedures (spec §9.12)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<EntryPoint>,
}

/// Alternative executable declaration for non-WASM procedures.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryPoint {
    /// Exact filename of the alternative executable (e.g. "macro_calculator.py")
    pub filename: String,
    /// Full execution command string (e.g. "python3 macro_calculator.py")
    pub command: String,
}

/// Fully-resolved view of one procedure in a pack.
#[derive(Debug, Clone)]
pub struct ProcedureDef {
    pub id: String,
    pub schema: ProcedureSchema,
    /// Present when {id}.wasm exists
    pub wasm_path: Option<PathBuf>,
    /// Present when schema.entry_point is set (non-WASM path)
    pub entry_point: Option<EntryPoint>,
    /// Path to {id}.md documentation
    pub doc_path: PathBuf,
    /// Path to {id}.schema.json
    pub schema_path: PathBuf,
}

impl ProcedureDef {
    pub fn is_runnable(&self) -> bool {
        self.wasm_path.is_some() || self.entry_point.is_some()
    }
}

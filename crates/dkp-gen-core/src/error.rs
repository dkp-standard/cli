use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("HTTP {status}: {body}")]
    Http { status: u16, body: String },

    #[error("LLM refused to produce output: {0}")]
    LlmRefusal(String),

    #[error("could not extract JSON from LLM response: {0}")]
    JsonExtraction(String),

    #[error("could not extract JSONL from LLM response: {0}")]
    JsonlExtraction(String),

    #[error("dkp-core error: {0}")]
    DkpCore(#[from] dkp_core::error::DkpError),

    #[error("packaging error: {0}")]
    Packaging(String),

    #[error("asset skipped: {0}")]
    AssetSkipped(String),
}

pub type GenResult<T> = Result<T, GenError>;

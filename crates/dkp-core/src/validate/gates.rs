use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gate {
    /// Machine usability — schemas, required files, graph edge resolution
    Gate4 = 4,
    /// Evaluation — eval_set.jsonl present and achieves min_eval_delta
    Gate7 = 7,
    /// OKF conformance — frontmatter, link resolution, bundle signature
    Gate8 = 8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateStatus {
    Pass,
    Fail,
    /// Gate was not run (e.g., optional asset not present)
    Skipped,
    /// Gate not applicable (e.g., OKF layer absent for gate 8)
    NotApplicable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateResult {
    pub gate: u8,
    pub status: GateStatus,
    pub checks: Vec<CheckResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub description: String,
    pub status: GateStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl CheckResult {
    pub fn pass(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            status: GateStatus::Pass,
            detail: None,
        }
    }

    pub fn fail(description: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            status: GateStatus::Fail,
            detail: Some(detail.into()),
        }
    }

    pub fn skip(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            status: GateStatus::Skipped,
            detail: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationReport {
    pub pack_name: String,
    pub pack_version: String,
    pub gates: Vec<GateResult>,
    pub overall: GateStatus,
    pub conformance: ConformanceLevel,
    pub reviewed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConformanceLevel {
    /// Gates 4 and 8 passed
    DkpConformant,
    /// Gates 4, 7, and 8 passed; review_notes attests human editorial review
    DkpReviewed,
    /// Not conformant
    NonConformant,
}

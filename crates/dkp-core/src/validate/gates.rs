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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_result_pass_sets_pass_status_and_no_detail() {
        let c = CheckResult::pass("thing checked");
        assert_eq!(c.status, GateStatus::Pass);
        assert_eq!(c.description, "thing checked");
        assert!(c.detail.is_none());
    }

    #[test]
    fn check_result_fail_sets_fail_status_and_detail() {
        let c = CheckResult::fail("thing checked", "why it failed");
        assert_eq!(c.status, GateStatus::Fail);
        assert_eq!(c.detail.as_deref(), Some("why it failed"));
    }

    #[test]
    fn check_result_skip_sets_skipped_status_and_no_detail() {
        let c = CheckResult::skip("thing checked");
        assert_eq!(c.status, GateStatus::Skipped);
        assert!(c.detail.is_none());
    }

    #[test]
    fn gate_status_serde_round_trip() {
        for status in [
            GateStatus::Pass,
            GateStatus::Fail,
            GateStatus::Skipped,
            GateStatus::NotApplicable,
        ] {
            let json = serde_json::to_string(&status).unwrap();
            let back: GateStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(status, back);
        }
    }

    #[test]
    fn conformance_level_serde_round_trip() {
        for level in [
            ConformanceLevel::DkpConformant,
            ConformanceLevel::DkpReviewed,
            ConformanceLevel::NonConformant,
        ] {
            let json = serde_json::to_string(&level).unwrap();
            let back: ConformanceLevel = serde_json::from_str(&json).unwrap();
            assert_eq!(level, back);
        }
    }

    #[test]
    fn gate_result_serde_round_trip_omits_none_message() {
        let result = GateResult {
            gate: 4,
            status: GateStatus::Pass,
            checks: vec![CheckResult::pass("ok")],
            message: None,
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(!json.contains("\"message\""));
        let back: GateResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.gate, 4);
        assert_eq!(back.status, GateStatus::Pass);
        assert_eq!(back.checks.len(), 1);
    }
}

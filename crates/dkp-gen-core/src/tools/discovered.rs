use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::error::GenResult;

/// One tool-discovered URL, staged for later human review and promotion
/// into `evidence/sources.csv` via `dkp rights add-source --from-discovered`.
/// Never written directly to `sources.csv` — see `.docs/GEN_AGENTS.md` §3.5.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredSource {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// "web_fetch" | "web_search"
    pub via: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// RFC3339 timestamp.
    pub retrieved_at: String,
    /// "generate" | "fix"
    pub command: String,
}

impl DiscoveredSource {
    pub fn now(
        url: String,
        title: Option<String>,
        via: impl Into<String>,
        query: Option<String>,
        command: impl Into<String>,
    ) -> Self {
        Self {
            url,
            title,
            via: via.into(),
            query,
            retrieved_at: rfc3339_now(),
            command: command.into(),
        }
    }
}

/// Append-only staging log at `build/sources_discovered.jsonl`.
///
/// Uses append-mode writes rather than the pipeline's usual atomic
/// tmp-then-rename helper, since this file is appended to incrementally
/// during a single tool-use loop rather than replaced wholesale.
pub struct DiscoveredLog {
    path: PathBuf,
    // Guards both the dedupe set and the underlying file handle so
    // concurrent tool calls append one full line at a time.
    seen: Mutex<HashSet<String>>,
}

impl DiscoveredLog {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            seen: Mutex::new(HashSet::new()),
        }
    }

    pub fn append(&self, entry: &DiscoveredSource) -> GenResult<()> {
        let mut seen = self.seen.lock().unwrap_or_else(|e| e.into_inner());
        if !seen.insert(entry.url.clone()) {
            return Ok(());
        }
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut line = serde_json::to_string(entry)?;
        line.push('\n');
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        file.write_all(line.as_bytes())?;
        Ok(())
    }
}

fn rfc3339_now() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format_rfc3339(secs)
}

/// Minimal UTC RFC3339 (seconds precision) formatter — avoids adding a
/// chrono/time dependency for a single timestamp field.
fn format_rfc3339(unix_secs: u64) -> String {
    const DAYS_PER_400Y: i64 = 146_097;
    let days_since_epoch = (unix_secs / 86_400) as i64;
    let secs_of_day = unix_secs % 86_400;
    let (hour, min, sec) = (
        secs_of_day / 3600,
        (secs_of_day / 60) % 60,
        secs_of_day % 60,
    );

    // Civil-from-days algorithm (Howard Hinnant), epoch = 1970-01-01.
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - DAYS_PER_400Y + 1 } / DAYS_PER_400Y;
    let doe = (z - era * DAYS_PER_400Y) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };

    format!("{y:04}-{m:02}-{d:02}T{hour:02}:{min:02}:{sec:02}Z")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn append_writes_jsonl_line() {
        let tmp = TempDir::new().unwrap();
        let log = DiscoveredLog::new(tmp.path().join("build/sources_discovered.jsonl"));
        log.append(&DiscoveredSource::now(
            "https://example.com".to_string(),
            Some("Example".to_string()),
            "web_fetch",
            None,
            "generate",
        ))
        .unwrap();
        let content =
            std::fs::read_to_string(tmp.path().join("build/sources_discovered.jsonl")).unwrap();
        assert_eq!(content.lines().count(), 1);
        let parsed: DiscoveredSource =
            serde_json::from_str(content.lines().next().unwrap()).unwrap();
        assert_eq!(parsed.url, "https://example.com");
        assert_eq!(parsed.via, "web_fetch");
    }

    #[test]
    fn append_dedupes_repeated_urls() {
        let tmp = TempDir::new().unwrap();
        let log = DiscoveredLog::new(tmp.path().join("build/sources_discovered.jsonl"));
        for _ in 0..3 {
            log.append(&DiscoveredSource::now(
                "https://example.com".to_string(),
                None,
                "web_fetch",
                None,
                "generate",
            ))
            .unwrap();
        }
        let content =
            std::fs::read_to_string(tmp.path().join("build/sources_discovered.jsonl")).unwrap();
        assert_eq!(content.lines().count(), 1);
    }

    #[test]
    fn rfc3339_formats_known_epoch_seconds() {
        // 2026-01-01T00:00:00Z
        assert_eq!(format_rfc3339(1_767_225_600), "2026-01-01T00:00:00Z");
    }
}

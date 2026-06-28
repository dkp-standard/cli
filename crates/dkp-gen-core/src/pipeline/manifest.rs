use serde_json::Value;

use crate::error::GenResult;
use crate::pipeline::context::PipelineContext;
use crate::prompt::{extract_json, templates};

/// Fill in audience/intended_use/known_limitations via LLM and refresh update_date.
pub async fn update_meta(ctx: &PipelineContext) -> GenResult<()> {
    let manifest_path = ctx.pack_dir.join("manifest.json");
    if !manifest_path.exists() {
        return Ok(());
    }

    let content = std::fs::read_to_string(&manifest_path)?;
    let mut manifest: Value = serde_json::from_str(&content)?;

    let needs_fill = ["audience", "intended_use", "known_limitations"]
        .iter()
        .any(|f| {
            manifest[f]
                .as_str()
                .map(|s| s.trim().is_empty() || s.starts_with("TODO"))
                .unwrap_or(true)
        });

    if needs_fill {
        let (sys, user) = templates::prompt_manifest_meta(&ctx.domain, &ctx.pack_name);
        let raw = ctx.generate("manifest_meta", &sys, &user).await?;
        if let Ok(meta) = extract_json(&raw) {
            for field in ["audience", "intended_use", "known_limitations"] {
                if let Some(v) = meta[field].as_str() {
                    if !v.trim().is_empty() {
                        manifest[field] = Value::String(v.to_string());
                    }
                }
            }
        }
    }

    // Refresh update_date
    manifest["update_date"] = Value::String(today_iso8601());
    ctx.write_json(&manifest_path, &manifest)
}

fn today_iso8601() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let days = secs / 86400;
    let z = days as i64 + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}

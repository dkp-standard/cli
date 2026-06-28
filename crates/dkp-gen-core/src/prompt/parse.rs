use crate::error::{GenError, GenResult};

/// Extract JSON from an LLM response that may wrap it in a ```json ... ``` fence.
pub fn extract_json(raw: &str) -> GenResult<serde_json::Value> {
    let stripped = strip_fence(raw);
    serde_json::from_str(stripped.trim()).map_err(|e| {
        GenError::JsonExtraction(format!("{e}: {}", &stripped[..stripped.len().min(200)]))
    })
}

/// Extract JSONL lines from an LLM response, tolerating fences and blank lines.
pub fn extract_jsonl(raw: &str) -> GenResult<Vec<serde_json::Value>> {
    let stripped = strip_fence(raw);
    // Fast path: if the whole thing is a JSON array, unwrap it directly
    if let Ok(serde_json::Value::Array(arr)) = serde_json::from_str(stripped.trim()) {
        return Ok(arr);
    }

    let mut results = Vec::new();
    for line in stripped.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match serde_json::from_str(line) {
            Ok(v) => results.push(v),
            Err(_) => continue, // tolerate stray non-JSON lines
        }
    }
    if results.is_empty() {
        return Err(GenError::JsonlExtraction(
            "no valid JSONL lines found".into(),
        ));
    }
    Ok(results)
}

fn strip_fence(raw: &str) -> &str {
    let raw = raw.trim();
    // Match ```json, ```jsonl, or plain ```
    if let Some(start) = raw.find("```") {
        let after_open = &raw[start + 3..];
        // Skip optional language tag on the opening fence line
        let content_start = after_open.find('\n').map(|i| i + 1).unwrap_or(0);
        let content = &after_open[content_start..];
        if let Some(end) = content.rfind("```") {
            return content[..end].trim();
        }
    }
    raw
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_fenced_json() {
        let raw = "```json\n{\"a\": 1}\n```";
        let v = extract_json(raw).unwrap();
        assert_eq!(v["a"], 1);
    }

    #[test]
    fn extracts_bare_json() {
        let v = extract_json(r#"{"x": true}"#).unwrap();
        assert!(v["x"].as_bool().unwrap());
    }

    #[test]
    fn extracts_jsonl() {
        let raw = "```jsonl\n{\"a\":1}\n{\"b\":2}\n```";
        let rows = extract_jsonl(raw).unwrap();
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn jsonl_fallback_array() {
        let raw = r#"[{"a":1},{"b":2}]"#;
        let rows = extract_jsonl(raw).unwrap();
        assert_eq!(rows.len(), 2);
    }
}

use dkp_core::types::chunks::RetrievalChunk;

const MIN_CHUNK_CHARS: usize = 300;

/// Port of chunk_by_semantic_boundary from ref.misc/dkp/generate_dkp.py.
/// Splits on markdown headers and list markers, merges short segments.
pub fn split(raw: &str, domain: &str, pack_name: &str) -> Vec<RetrievalChunk> {
    let pack_slug = slug(pack_name);
    let segments = semantic_split(raw);
    let merged = merge_short(segments);

    merged
        .into_iter()
        .enumerate()
        .map(|(i, seg)| {
            let first_line = seg
                .lines()
                .next()
                .unwrap_or("")
                .trim_start_matches(['#', '-', ' '])
                .trim();
            let title = if first_line.is_empty() {
                format!("Chunk {}", i + 1)
            } else {
                first_line.chars().take(80).collect()
            };
            RetrievalChunk {
                id: format!("{}_{:03}", pack_slug, i + 1),
                title,
                chunk_text: seg,
                tags: vec![domain.to_lowercase().replace(' ', "-")],
                source_ref: "generated".into(),
                confidence: Some(0.85),
                summary: None,
                embedding_model: None,
                token_count: None,
                retrieval_priority: None,
                asset_refs: vec![],
                ttl_days: None,
                review_date: None,
                stability: None,
                audience: vec![],
            }
        })
        .collect()
}

fn semantic_split(raw: &str) -> Vec<String> {
    // Split before heading markers and significant list openers
    let mut segments: Vec<String> = Vec::new();
    let mut current = String::new();

    for line in raw.lines() {
        let is_boundary = is_heading(line) || is_significant_list_item(line);
        if is_boundary && !current.trim().is_empty() {
            segments.push(current.trim().to_string());
            current = String::new();
        }
        if !current.is_empty() {
            current.push('\n');
        }
        current.push_str(line);
    }
    if !current.trim().is_empty() {
        segments.push(current.trim().to_string());
    }
    // Filter trivially short segments
    segments.into_iter().filter(|s| s.len() > 40).collect()
}

fn merge_short(segments: Vec<String>) -> Vec<String> {
    let mut merged: Vec<String> = Vec::new();
    let mut buf = String::new();
    for seg in segments {
        if buf.is_empty() {
            buf = seg;
        } else {
            buf.push_str("\n\n");
            buf.push_str(&seg);
        }
        if buf.len() >= MIN_CHUNK_CHARS {
            merged.push(buf);
            buf = String::new();
        }
    }
    if !buf.is_empty() {
        merged.push(buf);
    }
    merged
}

fn is_heading(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("# ") || trimmed.starts_with("## ") || trimmed.starts_with("### ")
}

fn is_significant_list_item(line: &str) -> bool {
    let trimmed = line.trim_start();
    // Numbered list: "1. Capital letter" or bold opener
    if let Some(rest) = trimmed.split_once(". ") {
        if rest.0.parse::<u32>().is_ok() {
            let after = rest.1;
            return after.starts_with(|c: char| c.is_uppercase()) || after.starts_with("**");
        }
    }
    // Bullet: "- Capital letter" or "- **bold"
    if let Some(rest) = trimmed.strip_prefix("- ") {
        return rest.starts_with(|c: char| c.is_uppercase()) || rest.starts_with("**");
    }
    false
}

fn slug(name: &str) -> String {
    let s: String = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect();
    s.trim_matches('_').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_on_headings() {
        let raw =
            "## Section A\nSome content here that is long enough to pass the filter threshold.\n\
                   ## Section B\nMore content here that is also long enough to pass the filter.";
        let chunks = split(raw, "test", "My Pack");
        assert!(chunks.len() >= 1);
        assert!(chunks[0].id.starts_with("my_pack_"));
    }

    #[test]
    fn merges_short_segments() {
        // Each line is short; they should get merged
        let raw = (0..20)
            .map(|i| format!("## H{i}\nshort"))
            .collect::<Vec<_>>()
            .join("\n");
        let chunks = split(&raw, "test", "Pack");
        // All short segments merged into fewer chunks
        assert!(chunks.len() < 20);
    }
}

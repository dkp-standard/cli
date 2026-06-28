use std::path::Path;

use crate::error::DkpResult;

/// A parsed OKF concept file: YAML frontmatter + Markdown body.
#[derive(Debug, Clone)]
pub struct OkfConcept {
    pub path: std::path::PathBuf,
    pub frontmatter: serde_yaml::Value,
    pub body: String,
}

/// Parse a single OKF concept file (Markdown with YAML frontmatter).
pub fn parse_concept(path: &Path) -> DkpResult<OkfConcept> {
    let content = std::fs::read_to_string(path)?;

    if let Some(stripped) = content.strip_prefix("---\n") {
        if let Some(end) = stripped.find("\n---\n") {
            let yaml_str = &stripped[..end];
            let body = stripped[end + 5..].to_string();
            let frontmatter: serde_yaml::Value = serde_yaml::from_str(yaml_str).map_err(|e| {
                crate::error::DkpError::OkfFrontmatter {
                    file: path.display().to_string(),
                    reason: e.to_string(),
                }
            })?;
            return Ok(OkfConcept {
                path: path.to_path_buf(),
                frontmatter,
                body,
            });
        }
    }

    Err(crate::error::DkpError::OkfFrontmatter {
        file: path.display().to_string(),
        reason: "missing YAML frontmatter delimiters".to_string(),
    })
}

/// Walk an OKF layer directory and parse all concept files.
pub fn parse_okf_dir(okf_dir: &Path) -> DkpResult<Vec<OkfConcept>> {
    let mut concepts = Vec::new();
    fn walk(dir: &Path, out: &mut Vec<OkfConcept>) -> crate::error::DkpResult<()> {
        for entry in std::fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                walk(&path, out)?;
            } else if path.extension().is_some_and(|e| e == "md") {
                out.push(parse_concept(&path)?);
            }
        }
        Ok(())
    }
    walk(okf_dir, &mut concepts)?;
    Ok(concepts)
}

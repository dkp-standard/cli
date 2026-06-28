pub mod schema;

#[cfg(feature = "procedures")]
pub mod executor;

pub mod scaffold;

pub use schema::{EntryPoint, ProcedureDef, ProcedureSchema};

use std::path::Path;

use crate::{error::DkpError, DkpResult, Pack};

/// Enumerate all procedures defined in machine/procedures/.
///
/// A procedure is any file with a `.schema.json` suffix. The `.wasm` and `.md`
/// companions are optional (absence is reported by validate_all, not here).
pub fn list(pack: &Pack) -> DkpResult<Vec<ProcedureDef>> {
    let dir = pack.procedures_dir();
    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut defs = Vec::new();

    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();

        // Only look at .schema.json files
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        let Some(id) = name.strip_suffix(".schema.json") else {
            continue;
        };
        let id = id.to_string();

        let schema_path = path.clone();
        let bytes = std::fs::read(&schema_path)?;
        let schema: ProcedureSchema =
            serde_json::from_slice(&bytes).map_err(|e| DkpError::AssetParse {
                asset: format!("procedures/{name}"),
                source: e,
            })?;

        let wasm_path = dir.join(format!("{id}.wasm"));
        let doc_path = dir.join(format!("{id}.md"));

        let entry_point = schema.entry_point.clone();

        defs.push(ProcedureDef {
            id,
            wasm_path: if wasm_path.exists() {
                Some(wasm_path)
            } else {
                None
            },
            entry_point,
            doc_path,
            schema_path,
            schema,
        });
    }

    defs.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(defs)
}

/// Validate all procedures in machine/procedures/ for Gate 4.
///
/// Returns a list of human-readable error strings (empty = all pass).
pub fn validate_all(pack: &Pack) -> DkpResult<Vec<String>> {
    let dir = pack.procedures_dir();
    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut errors = Vec::new();

    // procedure_capabilities must be declared when procedures/ is non-empty
    if pack.manifest.procedure_capabilities.is_none() {
        errors.push(
            "manifest.json missing 'procedure_capabilities' (required when machine/procedures/ is non-empty)".to_string(),
        );
    }

    let defs = list(pack)?;

    if defs.is_empty() {
        // Directory exists but has no .schema.json files
        return Ok(errors);
    }

    for def in &defs {
        let id = &def.id;

        // .schema.json must be valid JSON Schema (check for "type" or "$schema" key)
        if let Err(e) = validate_schema_file(&def.schema_path) {
            errors.push(format!("procedures/{id}.schema.json: {e}"));
        }

        // .md must exist and be non-empty
        if !def.doc_path.exists() {
            errors.push(format!(
                "procedures/{id}.md: missing (required alongside .schema.json)"
            ));
        } else if std::fs::metadata(&def.doc_path)
            .map(|m| m.len() == 0)
            .unwrap_or(false)
        {
            errors.push(format!("procedures/{id}.md: empty"));
        }

        // If .wasm absent, entry_point must be declared in schema
        if def.wasm_path.is_none() && def.entry_point.is_none() {
            errors.push(format!(
                "procedures/{id}: no .wasm binary and no entry_point in .schema.json; \
                 procedure cannot be executed (see spec §9.12)"
            ));
        }

        // If entry_point declared, the referenced filename must exist
        if let Some(ref ep) = def.entry_point {
            let alt_path = dir.join(&ep.filename);
            if !alt_path.exists() {
                errors.push(format!(
                    "procedures/{id}.schema.json entry_point.filename '{}' not found in machine/procedures/",
                    ep.filename
                ));
            }
        }
    }

    Ok(errors)
}

fn validate_schema_file(path: &Path) -> Result<(), String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let value: serde_json::Value =
        serde_json::from_slice(&bytes).map_err(|e| format!("invalid JSON: {e}"))?;

    // Must be an object with at minimum 'input' and 'output' keys
    let obj = value.as_object().ok_or("schema must be a JSON object")?;
    if !obj.contains_key("input") {
        return Err("missing required 'input' key".to_string());
    }
    if !obj.contains_key("output") {
        return Err("missing required 'output' key".to_string());
    }

    // input and output must each be JSON Schema objects
    for key in ["input", "output"] {
        if obj[key].as_object().is_none() {
            return Err(format!("'{key}' must be a JSON object (JSON Schema)"));
        }
    }

    Ok(())
}

/// Return the number of procedures that have a runnable WASM binary.
pub fn count_with_wasm(pack: &Pack) -> usize {
    list(pack)
        .unwrap_or_default()
        .iter()
        .filter(|d| d.wasm_path.is_some())
        .count()
}

/// Return the total number of procedures (schema files) in the pack.
pub fn count_total(pack: &Pack) -> usize {
    list(pack).unwrap_or_default().len()
}

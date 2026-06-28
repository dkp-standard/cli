use std::path::Path;

use crate::DkpResult;

/// Generate the three required procedure files plus a Rust WASI project template.
///
/// Creates:
///   machine/procedures/{id}.schema.json
///   machine/procedures/{id}.md
///   machine/procedures/src/Cargo.toml        (workspace root, created once)
///   machine/procedures/src/{id}/Cargo.toml
///   machine/procedures/src/{id}/src/main.rs
///   machine/procedures/src/.cargo/config.toml  (workspace root, created once)
///   machine/procedures/src/build.sh             (workspace root, created once)
pub fn scaffold(
    pack_root: &Path,
    id: &str,
    title: &str,
    description: &str,
) -> DkpResult<Vec<std::path::PathBuf>> {
    let mut created = Vec::new();

    // ── machine/procedures/ files ────────────────────────────────────────────

    let proc_dir = pack_root.join("machine").join("procedures");
    std::fs::create_dir_all(&proc_dir)?;

    // {id}.schema.json
    let schema_path = proc_dir.join(format!("{id}.schema.json"));
    let schema_json = serde_json::json!({
        "id": id,
        "title": title,
        "description": description,
        "input": {
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {},
            "additionalProperties": true
        },
        "output": {
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "result": {}
            }
        }
    });
    std::fs::write(
        &schema_path,
        serde_json::to_string_pretty(&schema_json).unwrap(),
    )?;
    created.push(schema_path);

    // {id}.md
    let doc_path = proc_dir.join(format!("{id}.md"));
    std::fs::write(
        &doc_path,
        format!(
            "# {title}\n\n\
             {description}\n\n\
             ## Input\n\n\
             Pass a JSON object on stdin. See `{id}.schema.json` for the full schema.\n\n\
             ```json\n{{}}\n```\n\n\
             ## Output\n\n\
             Returns a JSON object on stdout. See `{id}.schema.json` for the full schema.\n\n\
             ```json\n{{\"result\": null}}\n```\n\n\
             ## Build\n\n\
             ```bash\n\
             cd machine/procedures/src\n\
             ./build.sh\n\
             ```\n"
        ),
    )?;
    created.push(doc_path);

    // ── machine/procedures/src/ Rust workspace ──────────────────────────────

    let src_dir = proc_dir.join("src");
    let crate_dir = src_dir.join(id);
    std::fs::create_dir_all(crate_dir.join("src"))?;
    std::fs::create_dir_all(src_dir.join(".cargo"))?;

    // Workspace Cargo.toml — created once, updated each time to add new members
    let workspace_toml_path = src_dir.join("Cargo.toml");
    if workspace_toml_path.exists() {
        let existing = std::fs::read_to_string(&workspace_toml_path)?;
        if !existing.contains(&format!("\"{id}\"")) {
            // Insert before the closing `]` of the members array
            let updated = if let Some(pos) = existing.rfind(']') {
                format!(
                    "{},\n    \"{id}\",\n{}",
                    existing[..pos].trim_end_matches([' ', '\n', ',']),
                    &existing[pos..]
                )
            } else {
                existing + &format!("    \"{id}\",\n]\n")
            };
            std::fs::write(&workspace_toml_path, updated)?;
        }
    } else {
        std::fs::write(
            &workspace_toml_path,
            format!("[workspace]\nmembers = [\n    \"{id}\",\n]\nresolver = \"2\"\n"),
        )?;
    }
    created.push(workspace_toml_path);

    // {id}/Cargo.toml
    let cargo_toml_path = crate_dir.join("Cargo.toml");
    std::fs::write(
        &cargo_toml_path,
        format!(
            "[package]\n\
             name = \"{id}\"\n\
             version = \"0.1.0\"\n\
             edition = \"2021\"\n\n\
             [[bin]]\n\
             name = \"{id}\"\n\
             path = \"src/main.rs\"\n\n\
             [dependencies]\n\
             serde_json = \"1\"\n"
        ),
    )?;
    created.push(cargo_toml_path);

    // {id}/src/main.rs
    let main_rs_path = crate_dir.join("src").join("main.rs");
    std::fs::write(
        &main_rs_path,
        "use std::io::{self, Read, Write};\n\n\
         fn main() {\n\
             let mut input = String::new();\n\
             io::stdin().read_to_string(&mut input).unwrap();\n\
             let _input: serde_json::Value = serde_json::from_str(&input)\n\
                 .unwrap_or(serde_json::Value::Object(Default::default()));\n\n\
             // TODO: implement procedure logic here\n\
             let output = serde_json::json!({ \"result\": null });\n\n\
             io::stdout()\n\
                 .write_all(output.to_string().as_bytes())\n\
                 .unwrap();\n\
         }\n",
    )?;
    created.push(main_rs_path);

    // .cargo/config.toml at workspace root — sets target for all crates in the workspace
    let cargo_config_path = src_dir.join(".cargo").join("config.toml");
    if !cargo_config_path.exists() {
        std::fs::write(&cargo_config_path, "[build]\ntarget = \"wasm32-wasip1\"\n")?;
        created.push(cargo_config_path);
    }

    // build.sh at the workspace root — builds all procedures and copies .wasm files up
    let build_sh_path = src_dir.join("build.sh");
    let build_sh_exists = build_sh_path.exists();
    if !build_sh_exists {
        std::fs::write(
            &build_sh_path,
            "#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR=\"$(cd \"$(dirname \"${BASH_SOURCE[0]}\")\" && pwd)\"
cd \"$SCRIPT_DIR\"
cargo build --release --target wasm32-wasip1
for wasm in target/wasm32-wasip1/release/*.wasm; do
    name=\"$(basename \"$wasm\")\"
    cp \"$wasm\" \"../$name\"
    echo \"Built ../$name\"
done
",
        )?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&build_sh_path)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&build_sh_path, perms)?;
        }

        created.push(build_sh_path);
    }

    Ok(created)
}

/// Generate procedure files for a Python or JavaScript (Node) script.
///
/// Creates:
///   machine/procedures/{id}.schema.json  (with entry_point)
///   machine/procedures/{id}.md
///   machine/procedures/{id}.py  (or .js)
pub fn scaffold_script(
    pack_root: &Path,
    id: &str,
    title: &str,
    description: &str,
    lang: &str,
) -> DkpResult<Vec<std::path::PathBuf>> {
    let mut created = Vec::new();

    let proc_dir = pack_root.join("machine").join("procedures");
    std::fs::create_dir_all(&proc_dir)?;

    let (ext, command, script_template) = match lang {
        "javascript" => (
            "js",
            format!("node {id}.js"),
            "const chunks = [];\nprocess.stdin.on('data', d => chunks.push(d));\nprocess.stdin.on('end', () => {\n    const input = JSON.parse(Buffer.concat(chunks).toString() || '{}');\n    // TODO: implement procedure logic here\n    const output = { result: null };\n    process.stdout.write(JSON.stringify(output));\n});\n".to_string(),
        ),
        _ => (
            "py",
            format!("python3 {id}.py"),
            "import sys\nimport json\n\ndef main():\n    raw = sys.stdin.read()\n    input_data = json.loads(raw) if raw.strip() else {}\n    # TODO: implement procedure logic here\n    output = {\"result\": None}\n    sys.stdout.write(json.dumps(output))\n\nif __name__ == \"__main__\":\n    main()\n".to_string(),
        ),
    };

    // {id}.schema.json with entry_point
    let schema_path = proc_dir.join(format!("{id}.schema.json"));
    let schema_json = serde_json::json!({
        "id": id,
        "title": title,
        "description": description,
        "input": {
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {},
            "additionalProperties": true
        },
        "output": {
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "result": {}
            }
        },
        "entry_point": {
            "filename": format!("{id}.{ext}"),
            "command": command
        }
    });
    std::fs::write(
        &schema_path,
        serde_json::to_string_pretty(&schema_json).unwrap(),
    )?;
    created.push(schema_path);

    // {id}.md
    let doc_path = proc_dir.join(format!("{id}.md"));
    std::fs::write(
        &doc_path,
        format!(
            "# {title}\n\n\
             {description}\n\n\
             ## Input\n\n\
             Pass a JSON object on stdin. See `{id}.schema.json` for the full schema.\n\n\
             ```json\n{{}}\n```\n\n\
             ## Output\n\n\
             Returns a JSON object on stdout. See `{id}.schema.json` for the full schema.\n\n\
             ```json\n{{\"result\": null}}\n```\n\n\
             ## Run\n\n\
             ```bash\n\
             dkp run <pack> {id} --allow-unsigned\n\
             ```\n"
        ),
    )?;
    created.push(doc_path);

    // Script file
    let script_path = proc_dir.join(format!("{id}.{ext}"));
    if !script_path.exists() {
        std::fs::write(&script_path, &script_template)?;
    }
    created.push(script_path);

    Ok(created)
}

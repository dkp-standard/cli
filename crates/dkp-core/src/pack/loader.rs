use std::path::{Path, PathBuf};

use crate::{
    error::{DkpError, DkpResult},
    types::{
        chunks::RetrievalChunk, constraints::ConstraintsFile, decision_trees::DecisionTreesFile,
        eval::EvalCase, glossary::GlossaryFile, graph::KnowledgeGraph, manifest::Manifest,
        ontology::OntologyFile, rules::RulesFile,
    },
};

/// Lightweight handle to an open DKP pack directory.
///
/// `Pack::open()` reads only `manifest.json`. All other assets are loaded
/// lazily by the commands that need them.
pub struct Pack {
    pub root: PathBuf,
    pub manifest: Manifest,
}

impl Pack {
    /// Open a pack directory or `.zip` archive. Reads and validates manifest.json.
    pub fn open(path: impl AsRef<Path>) -> DkpResult<Self> {
        let root = path.as_ref().to_path_buf();

        if !root.exists() {
            return Err(DkpError::PackNotFound(root));
        }

        let manifest_path = root.join("manifest.json");
        if !manifest_path.exists() {
            return Err(DkpError::ManifestMissing(root));
        }

        let bytes = std::fs::read(&manifest_path)?;
        let manifest: Manifest =
            serde_json::from_slice(&bytes).map_err(|e| DkpError::ManifestInvalid {
                reason: e.to_string(),
            })?;

        validate_required_fields(&manifest)?;

        Ok(Pack { root, manifest })
    }

    // ── Directory helpers ────────────────────────────────────────────────────

    pub fn machine_dir(&self) -> PathBuf {
        self.root.join("machine")
    }
    pub fn okf_dir(&self) -> PathBuf {
        self.root.join("okf")
    }
    pub fn human_dir(&self) -> PathBuf {
        self.root.join("human")
    }
    pub fn evidence_dir(&self) -> PathBuf {
        self.root.join("evidence")
    }
    pub fn skills_dir(&self) -> PathBuf {
        self.root.join("skills")
    }
    pub fn l10n_dir(&self) -> PathBuf {
        self.root.join("l10n")
    }
    pub fn build_dir(&self) -> PathBuf {
        self.root.join("build")
    }
    pub fn procedures_dir(&self) -> PathBuf {
        self.machine_dir().join("procedures")
    }
    pub fn wasm_procedures_dir(&self) -> PathBuf {
        self.machine_dir().join("procedures").join("src")
    }

    pub fn machine_file(&self, name: &str) -> PathBuf {
        self.machine_dir().join(name)
    }
    pub fn evidence_file(&self, name: &str) -> PathBuf {
        self.evidence_dir().join(name)
    }

    // ── Presence checks ──────────────────────────────────────────────────────

    pub fn has_okf(&self) -> bool {
        self.okf_dir().exists()
    }
    pub fn has_procedures(&self) -> bool {
        self.procedures_dir().exists()
    }
    pub fn has_bundle_sig(&self) -> bool {
        self.root.join("bundle.sig").exists()
    }
    pub fn has_eval_set(&self) -> bool {
        self.machine_file("eval_set.jsonl").exists()
    }
    pub fn has_knowledge_graph(&self) -> bool {
        self.machine_file("knowledge_graph.json").exists()
    }
    pub fn has_mcp_manifest(&self) -> bool {
        self.machine_file("mcp_manifest.json").exists()
    }
    pub fn has_skills(&self) -> bool {
        self.skills_dir().exists()
    }
    pub fn has_l10n(&self) -> bool {
        self.l10n_dir().exists()
    }
    pub fn has_cross_refs(&self) -> bool {
        self.machine_file("cross_refs.json").exists()
    }
    pub fn has_assets(&self) -> bool {
        self.machine_file("assets.json").exists()
    }
    pub fn has_checksums(&self) -> bool {
        self.root.join("checksums.json").exists()
    }

    pub fn mcp_enabled(&self) -> bool {
        self.manifest.mcp.is_some()
    }

    // ── Asset loaders ────────────────────────────────────────────────────────

    pub fn load_glossary(&self) -> DkpResult<Option<GlossaryFile>> {
        load_json_optional(self, "glossary.json")
    }

    pub fn load_rules(&self) -> DkpResult<Option<RulesFile>> {
        load_json_optional(self, "rules.json")
    }

    pub fn load_ontology(&self) -> DkpResult<Option<OntologyFile>> {
        load_json_optional(self, "ontology.json")
    }

    pub fn load_constraints(&self) -> DkpResult<Option<ConstraintsFile>> {
        load_json_optional(self, "constraints.json")
    }

    pub fn load_decision_trees(&self) -> DkpResult<Option<DecisionTreesFile>> {
        load_json_optional(self, "decision_trees.json")
    }

    pub fn load_graph(&self) -> DkpResult<Option<KnowledgeGraph>> {
        load_json_optional(self, "knowledge_graph.json")
    }

    pub fn load_chunks(&self) -> DkpResult<Vec<RetrievalChunk>> {
        load_jsonl(self, "retrieval_chunks.jsonl")
    }

    pub fn load_eval_set(&self) -> DkpResult<Vec<EvalCase>> {
        load_jsonl(self, "eval_set.jsonl")
    }

    pub fn load_system_prompt(&self) -> DkpResult<Option<String>> {
        let path = self.machine_file("system_prompt.md");
        if !path.exists() {
            return Ok(None);
        }
        Ok(Some(std::fs::read_to_string(path)?))
    }
}

// ── Private helpers ──────────────────────────────────────────────────────────

fn validate_required_fields(m: &Manifest) -> DkpResult<()> {
    macro_rules! require {
        ($field:expr, $name:literal) => {
            if $field.trim().is_empty() {
                return Err(DkpError::ManifestFieldMissing { field: $name });
            }
        };
    }
    require!(m.spec, "spec");
    require!(m.name, "name");
    require!(m.version, "version");
    require!(m.domain, "domain");
    require!(m.audience, "audience");
    require!(m.intended_use, "intended_use");
    require!(m.known_limitations, "known_limitations");
    require!(m.update_date, "update_date");
    Ok(())
}

fn load_json_optional<T: serde::de::DeserializeOwned>(
    pack: &Pack,
    filename: &str,
) -> DkpResult<Option<T>> {
    let path = pack.machine_file(filename);
    if !path.exists() {
        return Ok(None);
    }
    let bytes = std::fs::read(&path)?;
    let value = serde_json::from_slice(&bytes).map_err(|e| DkpError::AssetParse {
        asset: filename.to_string(),
        source: e,
    })?;
    Ok(Some(value))
}

fn load_jsonl<T: serde::de::DeserializeOwned>(pack: &Pack, filename: &str) -> DkpResult<Vec<T>> {
    let path = pack.machine_file(filename);
    if !path.exists() {
        return Ok(vec![]);
    }

    let content = std::fs::read_to_string(&path)?;
    let mut items = Vec::new();
    let mut errors = Vec::new();

    for (i, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match serde_json::from_str::<T>(line) {
            Ok(item) => items.push(item),
            Err(e) => errors.push(DkpError::JsonlParse {
                file: filename.to_string(),
                line: i + 1,
                reason: e.to_string(),
            }),
        }
    }

    if !errors.is_empty() {
        return Err(errors.remove(0));
    }

    Ok(items)
}

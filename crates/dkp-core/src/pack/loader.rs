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
#[derive(Debug)]
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

    if let Err(e) = crate::domain::derive_and_validate_domain_slug(&m.domain) {
        return Err(DkpError::ManifestDomainInvalid {
            domain: m.domain.clone(),
            reason: e.to_string(),
        });
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn minimal_manifest_json() -> &'static str {
        r#"{
            "spec": "1.0.0",
            "name": "test-pack",
            "version": "1.0.0",
            "domain": "testing",
            "audience": "internal",
            "intended_use": "unit tests",
            "known_limitations": "none",
            "update_date": "2026-01-01"
        }"#
    }

    fn write_manifest(dir: &std::path::Path, contents: &str) {
        std::fs::write(dir.join("manifest.json"), contents).unwrap();
    }

    #[test]
    fn open_nonexistent_dir_errors() {
        let tmp = TempDir::new().unwrap();
        let missing = tmp.path().join("does-not-exist");
        let err = Pack::open(&missing).unwrap_err();
        assert!(matches!(err, DkpError::PackNotFound(_)));
    }

    #[test]
    fn open_missing_manifest_errors() {
        let tmp = TempDir::new().unwrap();
        let err = Pack::open(tmp.path()).unwrap_err();
        assert!(matches!(err, DkpError::ManifestMissing(_)));
    }

    #[test]
    fn open_invalid_manifest_json_errors() {
        let tmp = TempDir::new().unwrap();
        write_manifest(tmp.path(), "{ not valid json");
        let err = Pack::open(tmp.path()).unwrap_err();
        assert!(matches!(err, DkpError::ManifestInvalid { .. }));
    }

    #[test]
    fn open_manifest_missing_required_field_errors() {
        let tmp = TempDir::new().unwrap();
        write_manifest(
            tmp.path(),
            r#"{
                "spec": "1.0.0",
                "name": "",
                "version": "1.0.0",
                "domain": "testing",
                "audience": "internal",
                "intended_use": "unit tests",
                "known_limitations": "none",
                "update_date": "2026-01-01"
            }"#,
        );
        let err = Pack::open(tmp.path()).unwrap_err();
        assert!(matches!(
            err,
            DkpError::ManifestFieldMissing { field: "name" }
        ));
    }

    #[test]
    fn open_manifest_invalid_domain_errors() {
        // Reserved-word domains (e.g. "admin") are registry moderation
        // policy, not a structural error, so `Pack::open` (local/offline)
        // must accept them. Use a domain that slugifies to empty (too
        // short) to exercise the structural-validity error path instead.
        let tmp = TempDir::new().unwrap();
        write_manifest(
            tmp.path(),
            r#"{
                "spec": "1.0.0",
                "name": "test-pack",
                "version": "1.0.0",
                "domain": "!!!",
                "audience": "internal",
                "intended_use": "unit tests",
                "known_limitations": "none",
                "update_date": "2026-01-01"
            }"#,
        );
        let err = Pack::open(tmp.path()).unwrap_err();
        assert!(matches!(err, DkpError::ManifestDomainInvalid { .. }));
    }

    #[test]
    fn open_manifest_reserved_word_domain_succeeds_locally() {
        let tmp = TempDir::new().unwrap();
        write_manifest(
            tmp.path(),
            r#"{
                "spec": "1.0.0",
                "name": "test-pack",
                "version": "1.0.0",
                "domain": "Admin",
                "audience": "internal",
                "intended_use": "unit tests",
                "known_limitations": "none",
                "update_date": "2026-01-01"
            }"#,
        );
        Pack::open(tmp.path()).expect("reserved-word domains are allowed for local builds");
    }

    #[test]
    fn open_valid_minimal_manifest_succeeds() {
        let tmp = TempDir::new().unwrap();
        write_manifest(tmp.path(), minimal_manifest_json());
        let pack = Pack::open(tmp.path()).unwrap();
        assert_eq!(pack.manifest.name, "test-pack");
        assert_eq!(pack.manifest.domain, "testing");
        assert_eq!(pack.root, tmp.path());
    }

    #[test]
    fn has_helpers_reflect_filesystem_state() {
        let tmp = TempDir::new().unwrap();
        write_manifest(tmp.path(), minimal_manifest_json());
        let pack = Pack::open(tmp.path()).unwrap();

        assert!(!pack.has_okf());
        assert!(!pack.has_procedures());
        assert!(!pack.has_bundle_sig());
        assert!(!pack.has_eval_set());
        assert!(!pack.has_knowledge_graph());
        assert!(!pack.has_mcp_manifest());
        assert!(!pack.has_skills());
        assert!(!pack.has_l10n());
        assert!(!pack.has_cross_refs());
        assert!(!pack.has_assets());
        assert!(!pack.has_checksums());
        assert!(!pack.mcp_enabled());

        std::fs::create_dir_all(pack.okf_dir()).unwrap();
        std::fs::create_dir_all(pack.procedures_dir()).unwrap();
        std::fs::write(pack.root.join("bundle.sig"), "sig").unwrap();
        std::fs::write(pack.machine_file("eval_set.jsonl"), "").unwrap();
        std::fs::write(pack.machine_file("knowledge_graph.json"), "{}").unwrap();
        std::fs::write(pack.machine_file("mcp_manifest.json"), "{}").unwrap();
        std::fs::create_dir_all(pack.skills_dir()).unwrap();
        std::fs::create_dir_all(pack.l10n_dir()).unwrap();
        std::fs::write(pack.machine_file("cross_refs.json"), "{}").unwrap();
        std::fs::write(pack.machine_file("assets.json"), "{}").unwrap();
        std::fs::write(pack.root.join("checksums.json"), "{}").unwrap();

        assert!(pack.has_okf());
        assert!(pack.has_procedures());
        assert!(pack.has_bundle_sig());
        assert!(pack.has_eval_set());
        assert!(pack.has_knowledge_graph());
        assert!(pack.has_mcp_manifest());
        assert!(pack.has_skills());
        assert!(pack.has_l10n());
        assert!(pack.has_cross_refs());
        assert!(pack.has_assets());
        assert!(pack.has_checksums());
    }

    #[test]
    fn load_json_optional_wrappers_absent_return_none() {
        let tmp = TempDir::new().unwrap();
        write_manifest(tmp.path(), minimal_manifest_json());
        let pack = Pack::open(tmp.path()).unwrap();

        assert!(pack.load_glossary().unwrap().is_none());
        assert!(pack.load_rules().unwrap().is_none());
        assert!(pack.load_ontology().unwrap().is_none());
        assert!(pack.load_constraints().unwrap().is_none());
        assert!(pack.load_decision_trees().unwrap().is_none());
        assert!(pack.load_graph().unwrap().is_none());
        assert!(pack.load_system_prompt().unwrap().is_none());
    }

    #[test]
    fn load_glossary_valid_returns_some() {
        let tmp = TempDir::new().unwrap();
        write_manifest(tmp.path(), minimal_manifest_json());
        let pack = Pack::open(tmp.path()).unwrap();
        std::fs::create_dir_all(pack.machine_dir()).unwrap();
        std::fs::write(
            pack.machine_file("glossary.json"),
            r#"{"terms": [{"id": "t1", "term": "Term", "definition": "def"}]}"#,
        )
        .unwrap();

        let glossary = pack.load_glossary().unwrap().unwrap();
        assert_eq!(glossary.terms.len(), 1);
        assert_eq!(glossary.terms[0].id, "t1");
    }

    #[test]
    fn load_glossary_invalid_json_errors() {
        let tmp = TempDir::new().unwrap();
        write_manifest(tmp.path(), minimal_manifest_json());
        let pack = Pack::open(tmp.path()).unwrap();
        std::fs::create_dir_all(pack.machine_dir()).unwrap();
        std::fs::write(pack.machine_file("glossary.json"), "{ bad json").unwrap();

        let err = pack.load_glossary().unwrap_err();
        assert!(matches!(err, DkpError::AssetParse { .. }));
    }

    #[test]
    fn load_chunks_absent_returns_empty_vec() {
        let tmp = TempDir::new().unwrap();
        write_manifest(tmp.path(), minimal_manifest_json());
        let pack = Pack::open(tmp.path()).unwrap();
        assert!(pack.load_chunks().unwrap().is_empty());
    }

    #[test]
    fn load_chunks_valid_jsonl() {
        let tmp = TempDir::new().unwrap();
        write_manifest(tmp.path(), minimal_manifest_json());
        let pack = Pack::open(tmp.path()).unwrap();
        std::fs::create_dir_all(pack.machine_dir()).unwrap();
        let jsonl = concat!(
            r#"{"id":"c1","title":"T1","chunk_text":"body one","source_ref":"generated"}"#,
            "\n",
            r#"{"id":"c2","title":"T2","chunk_text":"body two","source_ref":"generated"}"#,
            "\n",
        );
        std::fs::write(pack.machine_file("retrieval_chunks.jsonl"), jsonl).unwrap();

        let chunks = pack.load_chunks().unwrap();
        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0].id, "c1");
        assert_eq!(chunks[1].id, "c2");
    }

    #[test]
    fn load_chunks_invalid_line_errors() {
        let tmp = TempDir::new().unwrap();
        write_manifest(tmp.path(), minimal_manifest_json());
        let pack = Pack::open(tmp.path()).unwrap();
        std::fs::create_dir_all(pack.machine_dir()).unwrap();
        std::fs::write(pack.machine_file("retrieval_chunks.jsonl"), "not json\n").unwrap();

        let err = pack.load_chunks().unwrap_err();
        assert!(matches!(err, DkpError::JsonlParse { .. }));
    }

    #[test]
    fn load_system_prompt_present() {
        let tmp = TempDir::new().unwrap();
        write_manifest(tmp.path(), minimal_manifest_json());
        let pack = Pack::open(tmp.path()).unwrap();
        std::fs::create_dir_all(pack.machine_dir()).unwrap();
        std::fs::write(pack.machine_file("system_prompt.md"), "Be helpful.").unwrap();

        assert_eq!(
            pack.load_system_prompt().unwrap(),
            Some("Be helpful.".to_string())
        );
    }
}

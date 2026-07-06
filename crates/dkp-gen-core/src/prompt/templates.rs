/// Returns (system, user) prompt pair for each asset type.
/// Prompt text ported from ref.misc/dkp/generate_dkp.py.
pub fn base_system(domain: &str, pack_name: &str) -> String {
    format!(
        "You are a domain expert creating a Domain Knowledge Pack for the '{domain}' domain. \
         Pack name: '{pack_name}'. \
         Produce structured, high-signal content. Be specific, practical, and avoid generic advice."
    )
}

/// Appended to `base_system` when tool use is available, so grounded steps
/// actually use `web_fetch`/`web_search` rather than relying solely on
/// training-data recall.
pub fn grounding_preamble() -> &'static str {
    " Before writing, use the web_search and web_fetch tools to find and read \
      authoritative, current sources on this domain. Ground your output in what \
      you find rather than relying solely on general knowledge."
}

pub fn prompt_system_prompt(domain: &str, pack_name: &str) -> (String, String) {
    let system = base_system(domain, pack_name);
    let user = format!(
        "Write a concise LLM system prompt for an agent specialized in '{domain}'. \
         It should ground the agent in domain terminology, constraints, and decision patterns. \
         Output only the system prompt text, no surrounding explanation."
    );
    (system, user)
}

pub fn prompt_rules(domain: &str, pack_name: &str) -> (String, String) {
    let system = base_system(domain, pack_name);
    let user = format!(
        "Write the core operational rules for the '{domain}' domain as a JSON object with key \"rules\", \
         a list of objects each having: id (str), title (str), description (str), \
         polarity (\"affirmative\" | \"prohibitive\"), stability (\"stable\" | \"volatile\" | \"experimental\"), \
         source_ref (str, use \"generated\"). \
         Include must-do rules, must-avoid rules, common failure modes, and key invariants. \
         Be specific and actionable. Output ONLY valid JSON, no explanation."
    );
    (system, user)
}

pub fn prompt_ontology(domain: &str, pack_name: &str) -> (String, String) {
    let system = base_system(domain, pack_name);
    let user = format!(
        "Create a domain ontology for '{domain}' as a JSON object with key \"entity_types\", \
         a list of objects each having: id (str, kebab-case), name (str), description (str), \
         attributes (list of str), relationships (list of objects with name, target_type, cardinality). \
         Every entity MUST have at least one relationship to another entity in the ontology. \
         cardinality must be one of: \"one-to-one\", \"one-to-many\", \"many-to-many\". \
         Example relationship: {{\"name\": \"has-rounds\", \"target_type\": \"funding-round\", \"cardinality\": \"one-to-many\"}}. \
         Output ONLY valid JSON, no explanation."
    );
    (system, user)
}

pub fn prompt_glossary(domain: &str, pack_name: &str) -> (String, String) {
    let system = base_system(domain, pack_name);
    let user = format!(
        "Create a domain glossary for '{domain}' as a JSON object with key \"terms\", \
         a list of objects each having: id (str), term (str), definition (str), \
         stability (\"stable\" | \"volatile\" | \"experimental\"), source_ref (str, use \"generated\"). \
         Include at least 20 terms covering core concepts. \
         Output ONLY valid JSON, no explanation."
    );
    (system, user)
}

pub fn prompt_constraints(domain: &str, pack_name: &str) -> (String, String) {
    let system = base_system(domain, pack_name);
    let user = format!(
        "Create domain constraints for '{domain}' as a JSON object with keys: \
         \"edge_cases\" (list: id, title, description), \
         \"anti_patterns\" (list: id, title, description), \
         \"hard_limits\" (list: id, title, description). \
         Output ONLY valid JSON, no explanation."
    );
    (system, user)
}

pub fn prompt_decision_trees(domain: &str, pack_name: &str) -> (String, String) {
    let system = base_system(domain, pack_name);
    let user = format!(
        "Create 2-3 decision trees for common '{domain}' decisions as a JSON object with key \"trees\", \
         a list of objects each having: id (str), title (str), description (str), root (recursive node: \
         {{\"question\": str, \"branches\": [{{\"condition\": str, \"next\": node}}]}} or {{\"answer\": str}}). \
         Output ONLY valid JSON, no explanation."
    );
    (system, user)
}

pub fn prompt_chunks_raw(
    domain: &str,
    pack_name: &str,
    context_bundle: &str,
    discovered_source_count: usize,
) -> (String, String) {
    let system = base_system(domain, pack_name);
    let coverage_guidance = "Cover between 20 and approximately 60 distinct topics/facts as \
         separate ## or ### sections, scaling with how broad the domain naturally is — a narrow \
         domain should land near 20, a broad one can go higher, but don't pad a narrow domain \
         with filler or repetition just to reach the upper end.";
    let source_guidance = if discovered_source_count > 0 {
        format!(
            " You have already gathered {discovered_source_count} source(s) via web_search/web_fetch \
             during earlier generation steps in this pipeline; aim for roughly 1-3 chunks per \
             distinct source in addition to the general topic coverage above, and feel free to \
             fetch or search for more sources here if it would improve coverage."
        )
    } else {
        String::new()
    };
    let user = format!(
        "Write comprehensive domain knowledge for '{domain}' structured as discrete, self-contained facts, \
         rules, procedures, and patterns. Use markdown headers (##, ###) and numbered lists to separate topics. \
         {coverage_guidance}{source_guidance} \
         Context summary: {context_bundle}. \
         Each unit should be independently useful when retrieved by an agent."
    );
    (system, user)
}

pub fn prompt_eval_set(domain: &str, pack_name: &str, corpus_excerpt: &str) -> (String, String) {
    let system = base_system(domain, pack_name);
    let user = format!(
        "Create 15 evaluation entries for the '{domain}' domain knowledge pack. \
         Each entry is a JSON object on its own line (JSONL format) with keys: \
         \"query\" (a realistic question an agent might face), \
         \"expected_dimensions\" (list of answer aspects that should be addressed), \
         \"critical_must_include\" (list of 2-4 plain English concept words or phrases — \
         choose terms that ARE present in the corpus excerpt below), \
         \"scoring_rubric\" (one sentence describing what a good answer looks like). \
         Output ONLY the JSONL lines, no surrounding text.\n\n\
         Corpus excerpt to ground your must_include terms:\n{corpus_excerpt}"
    );
    (system, user)
}

pub fn prompt_handbook(domain: &str, pack_name: &str) -> (String, String) {
    let system = base_system(domain, pack_name);
    let user = format!(
        "Write a practical '{domain}' handbook for the '{pack_name}' Domain Knowledge Pack. \
         Include: introduction, key concepts, practical examples with walkthroughs, \
         common pitfalls, implementation checklists, and troubleshooting guidance. \
         Format as a well-structured markdown document with clear headers."
    );
    (system, user)
}

pub fn prompt_quickstart(domain: &str, pack_name: &str) -> (String, String) {
    let system = base_system(domain, pack_name);
    let user = format!(
        "Write a quick-start guide for '{pack_name}' covering the '{domain}' domain. \
         Target a reader who needs to get productive in under 30 minutes. \
         Include: prerequisites, first steps, a minimal worked example, and common gotchas. \
         Format as markdown."
    );
    (system, user)
}

pub fn prompt_faq(domain: &str, pack_name: &str) -> (String, String) {
    let system = base_system(domain, pack_name);
    let user = format!(
        "Write an FAQ document for the '{domain}' domain covering 10-15 common questions. \
         Each question should have a direct, specific answer. \
         Format as markdown with ## headers for each Q&A pair."
    );
    (system, user)
}

pub fn prompt_examples(domain: &str, pack_name: &str) -> (String, String) {
    let system = base_system(domain, pack_name);
    let user = format!(
        "Write 3-5 worked examples for the '{domain}' domain. \
         Each example should show a realistic scenario, the decision process, and the outcome. \
         Format as markdown with clear problem/solution structure."
    );
    (system, user)
}

pub fn prompt_manifest_meta(domain: &str, pack_name: &str) -> (String, String) {
    let system = base_system(domain, pack_name);
    let user = format!(
        "For the '{domain}' domain knowledge pack '{pack_name}', write concise values for these manifest fields. \
         Respond with ONLY a JSON object containing these exact keys: \
         \"title\" (a short, human-readable display name for the pack, distinct from the machine-readable pack name), \
         \"audience\" (1-2 sentences describing the target user), \
         \"intended_use\" (1-2 sentences on how the pack should be used), \
         \"known_limitations\" (1-2 sentences on what the pack does not cover or guarantee). \
         Output ONLY the JSON object."
    );
    (system, user)
}

pub fn prompt_readme_contents(domain: &str, pack_name: &str, pack_summary: &str) -> (String, String) {
    let system = base_system(domain, pack_name);
    let user = format!(
        "Write the \"Contents\" section of a README for the '{domain}' domain knowledge pack \
         '{pack_name}'. Describe, in 2-4 short paragraphs or a bullet list, what this pack \
         actually covers — the key topics, rule categories, and terminology a reader should \
         expect to find, based on the summary below. Be specific to this pack's real content, \
         not generic DKP boilerplate.\n\n\
         Pack contents summary:\n{pack_summary}\n\n\
         Output ONLY the section body in markdown (no \"## Contents\" heading, no surrounding \
         explanation)."
    );
    (system, user)
}

pub fn prompt_eval_answer(
    domain: &str,
    pack_name: &str,
    query: &str,
    context: &str,
) -> (String, String) {
    let system = if context.is_empty() {
        format!(
            "You are a knowledgeable assistant in the '{domain}' domain. \
             Answer questions accurately based on your general knowledge."
        )
    } else {
        format!(
            "You are a knowledgeable assistant in the '{domain}' domain for the '{pack_name}' knowledge pack. \
             Use the following domain knowledge to answer questions:\n\n{context}"
        )
    };
    let user = format!("Answer the following question concisely and accurately:\n\n{query}");
    (system, user)
}

pub fn prompt_eval_score(
    query: &str,
    answer: &str,
    rubric: &str,
    must_include: &[String],
) -> (String, String) {
    let system = "You are an objective evaluator scoring how well an answer meets a scoring rubric. \
                  Respond with ONLY a JSON object with keys: \"score\" (float 0.0-1.0), \"pass\" (bool) and \"reason\" (one sentence).".to_string();
    let must_list = must_include
        .iter()
        .map(|s| format!("- {s}"))
        .collect::<Vec<_>>()
        .join("\n");
    let user = format!(
        "Query: {query}\n\n\
         Answer: {answer}\n\n\
         Scoring rubric: {rubric}\n\n\
         The answer MUST include references to ALL of these concepts:\n{must_list}\n\n\
         Score the answer from 0.0 (completely fails the rubric) to 1.0 (fully meets the rubric). \
         Respond with ONLY JSON: {{\"score\": 0.0-1.0, \"pass\": true/false, \"reason\": \"one sentence\"}}"
    );
    (system, user)
}

pub fn prompt_consistency_check(
    domain: &str,
    pack_name: &str,
    machine_bundle: &str,
) -> (String, String) {
    let system = "You are a domain QA reviewer checking a Domain Knowledge Pack's machine layer \
                  for internal contradictions. Respond with ONLY a JSON object with keys: \
                  \"consistent\" (bool), \"issues\" (list of objects with \"assets\" (list of str), \
                  \"description\" (str), \"severity\" (\"error\" | \"warning\"))."
        .to_string();
    let user = format!(
        "Review the following machine-layer assets for the '{domain}' domain pack '{pack_name}' \
         and flag any internal contradictions between rules, constraints, glossary, and retrieval chunks \
         (e.g. a rule that a constraint's hard_limits contradicts, or a chunk that uses a glossary term \
         inconsistently with its definition). If there are no contradictions, return \
         {{\"consistent\": true, \"issues\": []}}.\n\n\
         Machine layer bundle:\n{machine_bundle}\n\n\
         Output ONLY valid JSON, no explanation."
    );
    (system, user)
}

pub fn prompt_citation_check(
    domain: &str,
    claims_excerpt: &str,
    sources_text: &str,
) -> (String, String) {
    let system = "You are a fact-checker verifying that a Domain Knowledge Pack's claims are \
                  supported by its cited sources. Respond with ONLY a JSON object with key \
                  \"unsupported_claims\", a list of objects with \"asset\" (str, which file the \
                  claim is from), \"claim\" (str, the specific claim text), \"reason\" (str, why \
                  it isn't supported by the sources)."
        .to_string();
    let user = format!(
        "For the '{domain}' domain pack, check whether the claims below are supported by the \
         provided sources. Flag specific, checkable factual claims (e.g. version numbers, dates, \
         legal requirements) that are NOT backed by any source text. Do not flag general domain \
         knowledge or structural content. If everything is supported, return \
         {{\"unsupported_claims\": []}}.\n\n\
         Claims to check:\n{claims_excerpt}\n\n\
         Sources:\n{sources_text}\n\n\
         Output ONLY valid JSON, no explanation."
    );
    (system, user)
}

pub fn prompt_fix_chunks(
    domain: &str,
    pack_name: &str,
    failure_summary: &str,
    corpus: &str,
) -> (String, String) {
    let system = base_system(domain, pack_name);
    let user = format!(
        "The eval set for the '{domain}' domain pack '{pack_name}' has failures. \
         Failure summary:\n{failure_summary}\n\n\
         Rewrite and expand the domain knowledge content to address these gaps. \
         Use markdown headers (##, ###) and numbered lists to structure the content. \
         Existing corpus for context:\n{corpus}\n\n\
         Output comprehensive domain knowledge that covers the missing topics. \
         Each section should be independently useful when retrieved by an agent."
    );
    (system, user)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_chunks_raw_always_includes_min_max_guidance() {
        let (_, user) = prompt_chunks_raw("testing", "Test Pack", "{}", 0);
        assert!(user.contains("between 20 and approximately 60"));
        assert!(!user.contains("You have already gathered"));
    }

    #[test]
    fn prompt_chunks_raw_adds_source_scaling_guidance_when_sources_discovered() {
        let (_, user) = prompt_chunks_raw("testing", "Test Pack", "{}", 7);
        assert!(user.contains("You have already gathered 7 source(s)"));
        assert!(user.contains("1-3 chunks per"));
    }
}

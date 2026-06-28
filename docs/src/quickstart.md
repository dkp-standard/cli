# Quickstart

This guide gets you from zero to a validated, injectable DKP pack in under ten minutes.

## Prerequisites

- `dkp` CLI installed and on your `$PATH` — run `dkp --version` to confirm
- For AI-generation commands (`dkp new`, `dkp generate`): an API key for a supported LLM provider

---

## 1. Scaffold a new pack

```sh
dkp init "Kubernetes Networking" --domain "Kubernetes"
```

This creates a directory `kubernetes-networking/` with all required layers pre-populated with placeholder content:

```
kubernetes-networking/
  manifest.json
  README.md
  machine/
    system_prompt.md
    glossary.json
    rules.json
    constraints.json
    decision_trees.json
    retrieval_chunks.jsonl
    ontology.json
  evidence/
    sources.csv
    rights_log.csv
    review_notes.md
  okf/
    index.md
```

Add `--extras` to also scaffold the optional assets: `eval_set.jsonl`, `knowledge_graph.json`, `human/handbook.md`, and `CHANGELOG.md`.

---

## 2. Fill in the machine layer

Open `machine/` and replace the placeholder stubs with real domain content:

| File | What to put in it |
|---|---|
| `system_prompt.md` | A grounding prompt for AI agents working in this domain |
| `glossary.json` | Canonical terms and definitions |
| `rules.json` | Must-do and must-avoid rules for agents |
| `constraints.json` | Edge cases, anti-patterns, and hard limits |
| `decision_trees.json` | Branching decision procedures |
| `retrieval_chunks.jsonl` | Self-contained facts for RAG retrieval |
| `ontology.json` | Domain entity types and their attributes |

All schemas are defined in [Appendix B of the DKP Specification](https://github.com/archmag0s/dkp/blob/main/docs/SPEC.md).

**Tip:** Use `dkp new` (see below) to have an LLM generate this content for you.

---

## 3. Validate the pack

```sh
dkp validate kubernetes-networking/
```

This runs all automated quality gates — schema validity, graph integrity, required-field presence. Fix any errors before proceeding.

Use `--strict` to treat warnings as errors:

```sh
dkp validate kubernetes-networking/ --strict
```

---

## 4. Inspect what you built

```sh
# Summary: name, version, asset counts, conformance status
dkp info kubernetes-networking/

# Full-text search across machine assets
dkp search kubernetes-networking/ "pod networking"

# Retrieve all glossary terms
dkp get kubernetes-networking/ term

# Retrieve the top 5 most relevant chunks for a query
dkp chunk kubernetes-networking/ "CNI plugin selection" --top 5
```

**Prefer a visual interface?** `dkp tui` opens a keyboard-driven full-screen browser; `dkp webui` serves the same content as a local web app:

```sh
dkp tui kubernetes-networking/
dkp webui kubernetes-networking/
```

---

## 5. Inject into an LLM context

```sh
# Print a ready-to-inject context block (default: system-prompt scope)
dkp inject kubernetes-networking/

# Include all machine assets
dkp inject kubernetes-networking/ --scope full

# Estimate token cost first
dkp inject kubernetes-networking/ --count-tokens
```

The output is Markdown by default. Pipe it directly into any LLM CLI or copy it into your system prompt.

---

## 6. Test with a grounded prompt

```sh
# Interactive REPL — ask questions against your pack
dkp prompt kubernetes-networking/ --api-key $OPENAI_API_KEY

# One-shot question
dkp prompt kubernetes-networking/ "What CNI plugins are supported?" --api-key $OPENAI_API_KEY
```

Any OpenAI-compatible provider works via `--base-url`. [OpenRouter](https://openrouter.ai) is a convenient option if you want access to multiple model families (Claude, Gemini, Llama, etc.) under a single API key:

```sh
dkp prompt kubernetes-networking/ "What CNI plugins are supported?" \
  --base-url https://openrouter.ai/api/v1 \
  --api-key $OPENROUTER_API_KEY \
  --model meta-llama/llama-3.3-70b-instruct
```

---

## 7. Build and sign for distribution

```sh
# Pre-release checklist
dkp release-check kubernetes-networking/

# Package into a versioned archive
dkp build kubernetes-networking/ --out dist/

# Generate a signing keypair (one-time setup)
dkp keygen --out ~/.dkp/

# Sign the archive
dkp sign dist/kubernetes-networking-0.1.0.zip
```

---

## Shortcut: LLM-generate the whole pack in one command

If you have an API key, skip steps 1–2 and let the CLI scaffold and generate everything:

```sh
dkp new "Kubernetes Networking" --domain "Kubernetes" \
  --api-key $OPENAI_API_KEY \
  --model gpt-4o

# Then validate what was generated
dkp validate kubernetes-networking/
```

Any OpenAI-compatible provider works — pass `--base-url` to point at a different endpoint:

```sh
# Using OpenRouter (access to Claude, Gemini, Llama, and more under one key)
dkp new "Kubernetes Networking" --domain "Kubernetes" \
  --base-url https://openrouter.ai/api/v1 \
  --api-key $OPENROUTER_API_KEY \
  --model anthropic/claude-sonnet-4-5
```

`dkp new` runs `init`, calls the LLM to populate all machine assets, runs `validate`, and optionally packages the result.

---

## What's next?

- Read the [full CLI reference](reference/cli-reference.md) for every flag and subcommand
- See [Man Pages](reference/man-pages.md) and [Shell Completions](reference/shell-completions.md) for shell integration
- Read the [DKP Specification](https://github.com/archmag0s/dkp/blob/main/docs/SPEC.md) for the full bundle schema and quality standard
- Use `dkp tui` or `dkp webui` to browse your pack visually
- Use `dkp prompt` for an interactive REPL to test the pack against any OpenAI-compatible model

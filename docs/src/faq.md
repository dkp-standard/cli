# FAQ

## Concepts

### What is a DKP bundle?

A DKP bundle is a directory (or `.zip` archive) that packages curated domain knowledge in a structured, validated format. It has six layers:

- **`machine/`** — structured JSON/JSONL assets that are the source of truth: glossary, rules, constraints, decision trees, retrieval chunks, and ontology
- **`okf/`** — an OKF-conformant Markdown bundle generated from the machine layer, readable by any OKF-compatible agent framework
- **`human/`** — a handbook, quickstart, and FAQ for human readers
- **`evidence/`** — provenance: source citations, rights log, and editorial review notes
- **`skills/`** — optional SKILL.md-compatible procedural knowledge
- **`l10n/`** — optional localized content for non-base locales

A single `manifest.json` at the root ties it together.

### How is DKP different from just giving an LLM a folder of documents?

Three things: structure, validation, and evaluation. Raw documents have no schema, so a processor has to guess what's a rule vs. a definition vs. a constraint. DKP makes those distinctions explicit and machine-readable. Validation (`dkp validate`) enforces them automatically before you deploy. And the optional eval set (`machine/eval_set.jsonl`) lets you measure whether the pack actually improves model answers compared to a baseline — with `dkp eval`.

### How is DKP related to OKF?

DKP is a strict superset of [Open Knowledge Format (OKF)](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md). Every conformant DKP bundle contains a fully conformant OKF bundle in its `okf/` layer. Any OKF-compatible tool can read that layer without modification. DKP adds the type taxonomy, the machine layer, quality gates, provenance, and eval methodology that OKF deliberately leaves open.

### What is the `machine/` layer and why is it the source of truth?

The `machine/` layer contains the authoritative structured JSON/JSONL assets. The `okf/` layer is *generated* from the machine layer by `dkp okf export`. This direction matters: machine assets are tightly typed and schema-validated; OKF Markdown files are human-friendly but lossy. If you edit the OKF layer directly and then re-export, your changes will be overwritten. Always edit the machine layer.

### What are the 8 quality gates?

Gates 1–3 and 5–6 are automated and enforced by `dkp validate`:

1. **Schema validity** — all machine assets pass their JSON schemas
2. **Graph integrity** — all `knowledge_graph.json` edges resolve to known concept IDs
3. **OKF conformance** — the `okf/` layer is a valid OKF bundle
5. **Cross-reference integrity** — all `cross_refs.json` local IDs resolve within the bundle
6. **Provenance completeness** — all sources have rights records

Gates 4, 7, and 8 require human or LLM-assisted review and are attested in `evidence/review_notes.md`:

4. **Relevance** — content is accurate and on-domain
7. **Utility** — the eval set shows measurable improvement over baseline
8. **Human usability** — the `human/` layer is complete and readable

A pack that passes gates 1–3, 5–6 is **DKP-Conformant**. A pack that additionally passes gates 4, 7–8 is **DKP-Reviewed**.

---

## Using the CLI

### How do I install `dkp`?

Install from [crates.io](https://crates.io/crates/dkp):

```sh
cargo install dkp
```

This installs the full feature set (TUI, MCP server, web UI, and WASM procedures) by default. To install a leaner binary without optional features:

```sh
cargo install dkp --no-default-features
cargo install dkp --no-default-features --features procedures   # core + WASM only
```

Distribution packages (Homebrew, apt, etc.) will be added once the first stable release is cut.

### `dkp validate` passes but `dkp validate --strict` fails. What's the difference?

`--strict` promotes warnings to errors. Warnings are issued for RECOMMENDED fields that are absent (like `knowledge_graph.json`, `eval_set.jsonl`, or the `human/` layer). A pack can be structurally valid — all REQUIRED assets present and schema-valid — while still missing recommended content. `--strict` is appropriate for CI pipelines that enforce the full quality bar before publishing.

### How do I add a feature-gated command like `dkp serve` or `dkp tui`?

These commands are included by default. If you installed with `--no-default-features` and need to add them back individually:

```sh
cargo install dkp --no-default-features --features mcp        # enables dkp serve
cargo install dkp --no-default-features --features tui        # enables dkp tui
cargo install dkp --no-default-features --features webui      # enables dkp webui
cargo install dkp --no-default-features --features procedures # enables dkp run
```

### What does `dkp tui` do?

`dkp tui` opens a full-screen terminal user interface for browsing and inspecting packs without typing individual commands. From the TUI you can navigate a pack's layers, read asset details, run searches, and view validation results — all keyboard-driven. It requires the `tui` feature (included by default).

### What does `dkp webui` do?

`dkp webui` starts a local web server and opens a browser-based UI for exploring a pack. It's useful when you want a richer visual layout than the TUI provides — expandable asset trees, hyperlinked cross-references, and a search bar. It requires the `webui` feature (included by default). The server binds to `127.0.0.1` and exits when you close the browser tab or press Ctrl-C.

```sh
dkp tui acme-widgets/           # open TUI for a pack
dkp webui acme-widgets/         # open web UI (default port 8735)
dkp webui acme-widgets/ --port 9000
```

### What output formats does `--output` support?

Every command accepts `--output <FORMAT>`:

- `plain` (default) — human-readable text
- `table` — aligned table layout using comfy-table
- `json` — machine-readable JSON, suitable for piping into `jq`

### What does `dkp prompt` do?

`dkp prompt` opens an interactive REPL that injects your pack into an LLM context and lets you ask questions against it. It's the quickest way to verify that the pack actually improves model answers — run it after authoring or after `dkp fix` to confirm the changes helped.

```sh
# Interactive REPL
dkp prompt acme-widgets/ --api-key $OPENAI_API_KEY

# One-shot question (no REPL)
dkp prompt acme-widgets/ "What are the power LED error codes?" --api-key $OPENAI_API_KEY

# Any OpenAI-compatible provider via --base-url
dkp prompt acme-widgets/ \
  --base-url https://openrouter.ai/api/v1 \
  --api-key $OPENROUTER_API_KEY \
  --model anthropic/claude-sonnet-4-5
```

For automated quality measurement (baseline vs. grounded delta), use `dkp eval` instead — it runs the `eval_set.jsonl` and produces a scored report.

### How do I filter results to a specific audience?

Pass `--audience <ID>` as a global flag. Assets in the machine layer can declare an `audience` field; the CLI filters retrieval and injection output to only include assets that match (or have no audience restriction).

---

## Pack Authoring

### What's the minimum viable pack?

`manifest.json` plus all seven REQUIRED machine assets:

```
my-pack/
  manifest.json
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
```

`dkp init` creates all of these with placeholder content. `dkp validate` will confirm the structure is correct.

### Can I have a pack with no retrieval chunks?

Technically yes — `retrieval_chunks.jsonl` is REQUIRED to exist but can be empty. However, a pack with no chunks won't be useful for RAG retrieval or injection via `dkp inject --scope chunks`. You should include at least a handful of meaningful chunks for any real use case.

### What goes in `system_prompt.md` vs. the rest of the machine layer?

`system_prompt.md` is the high-level grounding instruction for an agent operating in this domain — the "who you are and what rules apply" preamble. The rest of the machine layer provides the structured facts. `dkp inject --scope system-prompt` produces just that file. `dkp inject --scope full` appends the full machine layer after it.

### How should I version my pack?

Follow Semantic Versioning (`MAJOR.MINOR.PATCH`):

- **PATCH** — corrected facts, typo fixes, added sources; no schema changes
- **MINOR** — new terms, rules, or chunks added; existing content unchanged
- **MAJOR** — breaking changes: removed or renamed concepts, schema changes, significant scope changes

Consumers pinning `^1.2.0` in `manifest.json` `compatibility` dependencies will receive MINOR and PATCH updates automatically.

### What's the difference between a `DecisionProcedure` and an executable procedure?

A `DecisionProcedure` is a static, human-readable JSON decision tree in `okf/procedures/` — branching logic expressed as data. An executable procedure is a WASM binary in `machine/procedures/` that can be run directly with `dkp run`. Use decision procedures for logic that needs to be readable and auditable; use executable procedures for computation-heavy logic that requires actual code execution.

---

## Registry and Distribution

### How do I publish a pack?

```sh
# 1. Build and sign
dkp build my-pack/ --out dist/
dkp keygen  # one-time setup
dkp sign dist/my-pack-1.0.0.zip

# 2. Authenticate
dkp registry login --email you@example.com

# 3. Upload the archive to your storage (S3, GitHub Releases, etc.)
#    and get the public HTTPS URL, then:
dkp publish --url https://your-storage.example.com/my-pack-1.0.0.zip
```

### How do I install a pack from the registry?

```sh
dkp install @namespace/my-pack
dkp install @namespace/my-pack@1.2.0   # pin a specific version
```

Installed packs go to `~/.dkp/packs/` by default. Pass `--dest <DIR>` to install elsewhere.

### Can I publish a private pack?

Yes. Pass `--private` to `dkp publish`. Use `dkp registry pack grant --to email@example.com <name>` to grant access to specific accounts. Access tokens are verified by the registry on install.

---

## MCP Integration

### How do I serve a pack as an MCP server?

```sh
# Serve over stdio (default — for local MCP clients)
dkp serve my-pack/

# Serve over HTTP
dkp serve my-pack/ --transport http --port 8734
```

The MCP server exposes up to six tools depending on the pack's contents:

- `inject` — return a formatted context block from the pack for injection into an LLM prompt (scope: `system-prompt`, `full`, `minimal`, or `chunks`; format: `markdown`, `xml`, or `json`)
- `search` — full-text search across all pack assets (chunks, terms, rules, constraints)
- `chunk` — retrieve a specific retrieval chunk by ID
- `get` — fetch assets by type (`term`, `rule`, `chunk`, `constraint`, `entity`, `eval`, `graph`, `cross-ref`, `system-prompt`), optionally filtered by ID or title
- `list_procedures` — list available WASM procedures (only present if the pack has procedures)
- `run_procedure` — execute a WASM procedure by ID (only present if the pack has procedures)

Pass `--readonly` to expose only MCP resources (no tools). The server also exposes pack assets as MCP resources under the `dkp://<slug>/` URI scheme — system prompt, retrieval chunks, glossary terms, rules, and constraints are all individually addressable.

### What goes in `machine/mcp_manifest.json`?

Advisory configuration for MCP processors — preferred transport, auth scheme, tool whitelist, and rate limits. Processors SHOULD respect these hints but are not required to. Generate or regenerate it with:

```sh
dkp mcp-manifest my-pack/
```

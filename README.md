# Domain Knowledge Pack (DKP) Standard

[https://dkp-standard.com](https://dkp-standard.com)

[SPEC](https://github.com/dkp-standard/docs/blob/main/SPEC.md)

**DKP is an open standard for packaging curated domain knowledge so AI agents actually use it well.**

Most knowledge fed to AI agents is unstructured — raw documents, loosely formatted notes, or ad-hoc context dumps. DKP changes that. It gives producers a clear, validated bundle format and gives processors (agents, RAG pipelines, LLM apps) something they can reliably load, search, and trust.

DKP builds on [Open Knowledge Format (OKF)](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md) — every DKP bundle contains a fully conformant OKF bundle in its `okf/` layer — and adds the structure that OKF deliberately leaves open: a type taxonomy, a machine-readable layer, quality gates, provenance tracking, and an optional skill and localization layer.

---

## Who is DKP for?

**Knowledge engineers and domain experts** who want to package what they know — terminology, rules, constraints, decision logic — into a format that AI tools can use without hallucinating or making up facts.

**Developers building AI agents, RAG pipelines, or LLM applications** who want a reliable, validated input format instead of raw files. DKP bundles come with structured retrieval chunks, a knowledge graph, eval sets, and optional MCP tool integration out of the box.

**Teams deploying AI at scale** who need provenance, rights tracking, audience scoping, localization, and supply-chain integrity for the knowledge they put in front of a model.

---

## What's in a DKP bundle?

A bundle is a directory (or `.zip`) with six layers:

| Layer | What it contains |
|---|---|
| `machine/` | Structured JSON/JSONL assets: glossary, rules, constraints, decision trees, retrieval chunks, knowledge graph, eval set. Source of truth. |
| `okf/` | OKF-native Markdown concept files generated from the machine layer. Compatible with any OKF-supporting agent framework. |
| `human/` | A handbook, quickstart, cheatsheet, and FAQ written for human readers — no AI tooling required. |
| `evidence/` | Source citations, rights log, and editorial review notes. |
| `skills/` | Procedural SKILL.md-compatible skills bundled alongside the declarative knowledge. |
| `l10n/` | Translated or locale-adapted content for non-base locales. |

A single `manifest.json` at the root declares the pack's identity, audience, compatibility, access control, retrieval hints, and optional MCP surface.

---

## Quality built in

DKP defines an **8-gate quality standard** with three conformance tiers:

- **DKP-Conformant** — structural gates (4 and 8) enforced automatically by `dkp validate`: schema validity, graph integrity, OKF conformance.
- **DKP-Evaluated** — a DKP-Conformant bundle that additionally includes a populated `eval_set.jsonl` and passes Gate 7, demonstrating measurable retrieval improvement above the `min_eval_delta` threshold.
- **DKP-Reviewed** — a DKP-Evaluated bundle that additionally satisfies the editorial gates (1–3, 5–6), attested by a dated, named sign-off in `evidence/review_notes.md`.

An optional eval set (`machine/eval_set.jsonl`) lets you measure how much the pack actually improves model answers compared to a baseline — and `dkp eval` runs it.

---

## MCP integration

A conformant processor can serve any bundle's resources and tools (`inject`, `search`, `chunk`, `get`, and optionally `list_procedures`/`run_procedure`) over MCP without custom integration work. The optional `mcp` field in the manifest provides advisory configuration — preferred transport, auth scheme, tool whitelist — that processors SHOULD respect.

---

## CLI

The `dkp` CLI is the primary tool for authoring, inspecting, and deploying packs. Install it and run `dkp --help` for a full command listing.

### Global flags

These flags apply to every command:

| Flag | Description |
|---|---|
| `--output <FORMAT>` | Output format: `plain` (default), `table`, `json` |
| `-q, --quiet` | Suppress informational output; print only results |
| `-v, --verbose` | Print debug info (schema paths, provider calls, etc.) |
| `--audience <ID>` | Filter content to assets tagged for a specific audience profile |

### Authoring a pack from scratch

```sh
# Scaffold a new pack directory with all required files
dkp init my-pack/

# Or: scaffold and LLM-generate a complete pack in one command
dkp new "ACME widget troubleshooting" --domain support --dir acme-widgets/
```

### Generating and iterating content

```sh
# Run (or re-run) LLM generation on an existing pack
dkp generate acme-widgets/

# Run the eval set through the LLM
dkp eval acme-widgets/

# Failure-aware chunk regeneration using eval results
dkp fix acme-widgets/

# Generate evidence drafts for manual review gates
dkp review acme-widgets/
```

### Inspecting a pack

```sh
# Print a summary: name, version, asset counts, compliance status
dkp info acme-widgets/

# List all packs under a root directory
dkp list ~/packs/

# Retrieve a specific asset or all assets of a type
dkp get acme-widgets/ term
dkp get acme-widgets/ rule "widget power"

# Print a ready-to-inject LLM context block
dkp inject acme-widgets/
dkp inject acme-widgets/ --scope full --format xml
dkp inject acme-widgets/ --scope chunks --max-tokens 4000

# Inspect and validate the knowledge graph
dkp graph acme-widgets/ stats
dkp graph acme-widgets/ validate
dkp graph acme-widgets/ list

# Inspect and validate cross_refs.json
dkp cross-refs acme-widgets/ list
dkp cross-refs acme-widgets/ validate
```

### Searching

```sh
# Full-text BM25 search across machine assets
dkp search acme-widgets/ "widget not turning on"

# Retrieve the top-N most relevant retrieval chunks for a query
dkp chunk acme-widgets/ "power LED behavior" --top 5

# Search the registry instead of a local pack
dkp search --registry "widget troubleshooting"
```

### Validating and testing

```sh
# Run schema and compliance checks; exits non-zero on failure
dkp validate acme-widgets/
dkp validate acme-widgets/ --gate 4    # structural only
dkp validate acme-widgets/ --gate 7    # eval only
dkp validate acme-widgets/ --strict    # treat warnings as errors

# Run eval set against baseline and grounded prompts; print delta report
dkp eval acme-widgets/

# Interactive grounded prompt REPL — see "Interactive interfaces" above for full usage
dkp prompt acme-widgets/ --api-key $OPENAI_API_KEY

# Compare two pack versions and report what changed
dkp diff acme-widgets-v1/ acme-widgets-v2/
```

### Skills and localization

```sh
# Manage and validate the skills/ layer
dkp skills acme-widgets/ list
dkp skills acme-widgets/ validate
dkp skills acme-widgets/ show <name>

# Manage and validate the l10n/ localization layer
dkp l10n acme-widgets/ list
dkp l10n acme-widgets/ validate
dkp l10n acme-widgets/ export fr-FR --out ./acme-widgets-fr/
```

### Procedures

```sh
# List and validate executable procedures
dkp procedures acme-widgets/ list
dkp procedures acme-widgets/ validate

# Scaffold a new procedure
dkp procedures acme-widgets/ new macro-calc

# Invoke a procedure by ID
dkp run acme-widgets/ macro-calc --input '{"dose": 70}'
```

### Interactive interfaces

```sh
# Open a full-screen terminal UI for browsing a pack
dkp tui acme-widgets/

# Start a local web UI in your browser
dkp webui acme-widgets/
dkp webui acme-widgets/ --port 9000

# Interactive REPL: ask questions against a pack using any OpenAI-compatible model
dkp prompt acme-widgets/ --api-key $OPENAI_API_KEY
dkp prompt acme-widgets/ "What are the power LED error codes?" --api-key $OPENAI_API_KEY
dkp prompt acme-widgets/ \
  --base-url https://openrouter.ai/api/v1 \
  --api-key $OPENROUTER_API_KEY \
  --model anthropic/claude-sonnet-4-5
```

`dkp tui` is a keyboard-driven browser for pack layers, asset details, search, and validation results. `dkp webui` serves the same content as a local web app with expandable trees and hyperlinked cross-references. Both require their respective features (`tui`, `webui`), which are included by default.

`dkp prompt` injects the pack into an LLM context and lets you query it interactively — the fastest way to verify a pack improves model answers during authoring. For automated scoring, use `dkp eval`.

### Evidence and provenance

```sh
# Summary of sources and rights coverage
dkp rights acme-widgets/ status

# Flag entries with missing or expired fields
dkp rights acme-widgets/ check

# Add a source record interactively
dkp rights acme-widgets/ add-source

# Formatted rights compliance report
dkp rights acme-widgets/ report
```

### Exporting

```sh
# Convert machine assets to another format
dkp export acme-widgets/ okf --out ./okf-bundle/
dkp export acme-widgets/ langchain --out ./langchain-docs/
dkp export acme-widgets/ llamaindex --out ./llama-docs/
dkp export acme-widgets/ openai-files --out ./openai-docs/
dkp export acme-widgets/ markdown --out ./md-docs/
dkp export acme-widgets/ csv --out ./csv-export/
dkp export acme-widgets/ anki --out ./anki-export/

# OKF-specific operations (export, validate, stats, links, browse)
dkp okf acme-widgets/ validate
dkp okf acme-widgets/ stats
```

### Building and releasing

```sh
# Pre-release compliance checklist (runs all gates, checks human fields)
dkp release-check acme-widgets/

# Package the pack into a versioned archive with checksums.json
dkp build acme-widgets/ --out dist/

# Generate or regenerate machine/mcp_manifest.json
dkp mcp-manifest acme-widgets/

# Start the pack as an MCP server
dkp serve acme-widgets/
```

### Signing

```sh
# Generate an Ed25519 keypair for signing packs
dkp keygen --out keys/

# Sign a built archive with a private key
dkp sign dist/acme-widgets-1.0.0.zip --key keys/signing.key
```

### Registry

```sh
# Account management
dkp registry login --email you@example.com
dkp registry logout
dkp registry token rotate
dkp registry keys add --key keys/signing.pub

# Per-pack management
dkp registry pack versions acme/widgets
dkp registry pack set-visibility acme/widgets public
dkp registry pack grant acme/widgets --to colleague@example.com
dkp registry pack access acme/widgets

# Install a pack from the registry
dkp install acme/widgets@1.0.0

# Remove an installed pack
dkp uninstall acme/widgets

# Update installed packs to satisfy lock-file constraints
dkp update

# Publish a built and signed pack
dkp publish dist/acme-widgets-1.0.0.zip

# Mark a published version as yanked
dkp yank acme/widgets@1.0.0
```

---

## Read the specification

The full DKP Specification is in [SPEC.md](SPEC.md). It covers:

- Bundle structure and all layer schemas (§7–§14)
- The manifest and all its fields (§8)
- The 8-gate quality standard (§5)
- MCP surface declaration and tool schemas (§15)
- Normative JSON Schemas for every machine-layer asset (Appendix B)
- A complete worked example bundle (Appendix A)

---

**License:** Apache 2.0

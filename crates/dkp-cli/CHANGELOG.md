# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.1](https://github.com/dkp-standard/cli/compare/dkp-v0.4.0...dkp-v0.4.1) - 2026-07-06

### Added

- bump Rust edition to 2024, optimize release webui build workflow

### Other

- *(dkp)* cargo fmt
- *(dkp-cli)* flatten nested conditionals using let-chains
- *(dkp)* cargo fmt
- upgrade workspace to Rust edition 2024

## [0.4.0](https://github.com/dkp-standard/cli/compare/dkp-v0.3.4...dkp-v0.4.0) - 2026-07-06

### Added

- *(dkp)* add font assets, licenses, and update Cargo.lock
- add tool-use execution framework with web search and fetch

### Other

- Merge pull request #10 from dkp-standard/feat/dkp-gen-core-agent
- *(dkp-cli, dkp-gen-core)* wrap long lines for improved readability

## [0.3.4](https://github.com/dkp-standard/cli/compare/dkp-v0.3.3...dkp-v0.3.4) - 2026-07-05

### Fixed

- *(dkp-cli)* correct dkp binary asset path in package manifest

### Other

- Merge branch 'main' of github.com:dkp-standard/cli
- improve CI workflows, add DCO config, update README links

## [0.3.3](https://github.com/dkp-standard/cli/compare/dkp-v0.3.2...dkp-v0.3.3) - 2026-07-05

### Other

- Merge pull request #6 from dkp-standard/release-plz-2026-07-05T06-12-31Z
- update CLI examples and spec references across all READMEs

## [0.3.2](https://github.com/dkp-standard/cli/compare/dkp-v0.3.1...dkp-v0.3.2) - 2026-07-05

### Other

- add contributing guide, dual licenses, and update author field

## [0.3.1](https://github.com/dkp-standard/cli/compare/dkp-v0.3.0...dkp-v0.3.1) - 2026-07-05

### Fixed

- release fixes, update READMEs to include human agents

### Other

- Merge branch 'main' of github.com:dkp-standard/cli

## [0.3.0](https://github.com/dkp-standard/cli/compare/dkp-v0.2.0...dkp-v0.3.0) - 2026-07-04

### Added

- *(cli)* add publisher key pinning for registry installs
- [**breaking**] bump manifest spec to 1.0.0, surface publish warnings
- update eval workflow, gate 7 validation, add search pagination
- add domain slugification and normalize list domain filtering/sorting
- add package deprecation support
- automate registry email verification, fix dkp info label alignment
- *(registry)* [**breaking**] migrate auth to email magic links and add eval summary fields
- add support for tar.xz and .dkp archive formats
- *(dkp-cli)* remove --registry flag, use presigned uploads for publish
- *(cli)* add init --title flag, refactor search argument parsing
- *(cli)* implement registry uninstall and add registry search support

### Fixed

- *(registry)* correct signing key endpoint, add download/readme fields

### Other

- optimize rust caching, unify feature flags, fix code formatting
- *(dkp-gen-core)* add unit tests for human pipeline generation
- *(dkp-cli)* simplify DKP filename version check with is_some_and
- Merge branch 'main' of github.com:dkp-standard/cli

## [0.2.0](https://github.com/dkp-standard/cli/compare/dkp-v0.1.2...dkp-v0.2.0) - 2026-06-28

### Added

- *(init)* support scoped pack names, add manifest metadata fields
- improve scoped package slugs, add hex key support, enhance publish

### Other

- *(cli)* improve readability of build and publish command code
- [**breaking**] rename --dest to --out, update archive hashing and signature

## [0.1.2](https://github.com/dkp-standard/cli/compare/dkp-v0.1.1...dkp-v0.1.2) - 2026-06-28

### Added

- add sync-readme alias and crate readmes for crates.io publishing

### Other

- Merge branch 'main' of github.com:dkp-standard/cli

## [0.1.1](https://github.com/dkp-standard/cli/compare/dkp-v0.1.0...dkp-v0.1.1) - 2026-06-28

### Fixed

- lint errors
- lint errors

## [0.1.0](https://github.com/dkp-standard/cli-wip/releases/tag/dkp-v0.1.0) - 2026-06-27

### Added

- add `dkp registry yank` command to yank published pack versions
- *(serve)* implement HTTP SSE transport support for dkp serve
- *(cli)* add generic asset get, refactor inject with budget truncation
- add cargo docs alias, doc generation deps, and project FAQ
- *(webui/graph)* add node selection and detail popover to GraphPanel
- add knowledge graph visualization, update deps and gitignore
- *(dkp-cli)* add interactive REPL commands for procedure management
- *(dkp-cli)* add configurable pack context injection to prompt command
- implement cross-refs CLI commands and rights expiry utilities
- *(dkp-cli)* implement MCP manifest generation and TUI panel
- add multi-language procedure scaffolding and unsigned dev support
- *(serve)* add feature-gated procedure tools to MCP server
- *(dkp)* add Gate 4 procedure validation for DKP packs
- *(dkp-cli)* implement eval, review, and fix command logic
- *(gen)* add LLM pack generation workflow and dkp-gen-core crate
- *(dkp-cli)* implement pack archiving with checksums and search index
- *(dkp-cli)* implement chunk search and add OKF export subcommands
- *(dkp-cli)* add init subcommand and table render for info

### Fixed

- *(dkp-cli)* correct serve handler indentation, format eval pct calc
- *(cli)* unify API key env var, secure output, fix build/eval issues

### Other

- *(dkp-cli)* improve command code quality and prevent overflow
- adopt idiomatic is_none_or and div_ceil std methods
- reformat long function calls and literals for readability
- *(github)* add GitHub Actions CI, release and build workflows
- *(dkp-core)* simplify constraint/eval types, update dependencies
- initial commit

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.3](https://github.com/dkp-standard/cli/compare/dkp-core-v0.3.2...dkp-core-v0.3.3) - 2026-07-05

### Other

- update CLI examples and spec references across all READMEs

## [0.3.1](https://github.com/dkp-standard/cli/compare/dkp-core-v0.3.0...dkp-core-v0.3.1) - 2026-07-05

### Fixed

- release fixes, update READMEs to include human agents

### Other

- Merge branch 'main' of github.com:dkp-standard/cli

## [0.3.0](https://github.com/dkp-standard/cli/compare/dkp-core-v0.2.0...dkp-core-v0.3.0) - 2026-07-04

### Added

- *(cli)* add publisher key pinning for registry installs
- [**breaking**] bump manifest spec to 1.0.0, surface publish warnings
- *(core)* split domain slug validation for local and registry use
- update eval workflow, gate 7 validation, add search pagination
- add domain slugification and normalize list domain filtering/sorting
- add package deprecation support
- *(registry)* [**breaking**] migrate auth to email magic links and add eval summary fields
- *(dkp-core)* enforce WASM-only procedure validation
- add support for tar.xz and .dkp archive formats
- *(dkp-cli)* remove --registry flag, use presigned uploads for publish

### Fixed

- *(registry)* correct signing key endpoint, add download/readme fields

### Other

- *(dkp-gen-core)* add unit tests for human pipeline generation

## [0.2.0](https://github.com/dkp-standard/cli/compare/dkp-core-v0.1.2...dkp-core-v0.2.0) - 2026-06-28

### Other

- [**breaking**] rename --dest to --out, update archive hashing and signature

## [0.1.2](https://github.com/dkp-standard/cli/compare/dkp-core-v0.1.1...dkp-core-v0.1.2) - 2026-06-28

### Added

- add sync-readme alias and crate readmes for crates.io publishing

## [0.1.0](https://github.com/dkp-standard/cli-wip/releases/tag/dkp-core-v0.1.0) - 2026-06-27

### Added

- add `dkp registry yank` command to yank published pack versions
- implement cross-refs CLI commands and rights expiry utilities
- *(dkp-cli)* implement MCP manifest generation and TUI panel
- add multi-language procedure scaffolding and unsigned dev support
- *(dkp)* add Gate 4 procedure validation for DKP packs
- *(dkp-cli)* implement eval, review, and fix command logic
- *(dkp-cli)* implement chunk search and add OKF export subcommands

### Other

- build fixes
- reformat long function calls and literals for readability
- *(dkp-core)* simplify constraint/eval types, update dependencies
- initial commit

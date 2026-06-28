
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'dkp' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'dkp'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'dkp' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Scaffold a new DKP pack directory with all required files')
            [CompletionResult]::new('info', 'info', [CompletionResultType]::ParameterValue, 'Print a summary of a pack (name, version, asset counts, compliance)')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all packs under a root directory')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'Run schema and compliance checks; exit non-zero on failure')
            [CompletionResult]::new('search', 'search', [CompletionResultType]::ParameterValue, 'Full-text BM25 search across machine assets (or registry with --registry)')
            [CompletionResult]::new('get', 'get', [CompletionResultType]::ParameterValue, 'Retrieve a specific asset or all assets of a type')
            [CompletionResult]::new('inject', 'inject', [CompletionResultType]::ParameterValue, 'Print a ready-to-inject LLM context block')
            [CompletionResult]::new('export', 'export', [CompletionResultType]::ParameterValue, 'Convert machine assets to another format (okf, langchain, llamaindex, …)')
            [CompletionResult]::new('okf', 'okf', [CompletionResultType]::ParameterValue, 'OKF-specific operations (export, validate, stats, links, browse)')
            [CompletionResult]::new('chunk', 'chunk', [CompletionResultType]::ParameterValue, 'Retrieve the top-N most relevant retrieval chunks for a query')
            [CompletionResult]::new('eval', 'eval', [CompletionResultType]::ParameterValue, 'Run eval set against baseline and grounded prompts; print delta report')
            [CompletionResult]::new('prompt', 'prompt', [CompletionResultType]::ParameterValue, 'Interactive grounded prompt REPL for testing a pack')
            [CompletionResult]::new('diff', 'diff', [CompletionResultType]::ParameterValue, 'Compare two pack versions and report what changed')
            [CompletionResult]::new('build', 'build', [CompletionResultType]::ParameterValue, 'Package a pack into a versioned archive with checksums.json')
            [CompletionResult]::new('release-check', 'release-check', [CompletionResultType]::ParameterValue, 'Pre-release compliance checklist (runs all gates, checks human fields)')
            [CompletionResult]::new('rights', 'rights', [CompletionResultType]::ParameterValue, 'Source and rights log operations')
            [CompletionResult]::new('mcp-manifest', 'mcp-manifest', [CompletionResultType]::ParameterValue, 'Generate or regenerate machine/mcp_manifest.json')
            [CompletionResult]::new('serve', 'serve', [CompletionResultType]::ParameterValue, 'Start the pack as an MCP server (requires --features mcp)')
            [CompletionResult]::new('tui', 'tui', [CompletionResultType]::ParameterValue, 'Interactive TUI browser (requires --features tui)')
            [CompletionResult]::new('webui', 'webui', [CompletionResultType]::ParameterValue, 'Browse a pack in a local web UI (requires --features webui)')
            [CompletionResult]::new('run', 'run', [CompletionResultType]::ParameterValue, 'Invoke a WASM/WASI procedure from machine/procedures/')
            [CompletionResult]::new('procedures', 'procedures', [CompletionResultType]::ParameterValue, 'List, validate, and scaffold executable procedures')
            [CompletionResult]::new('graph', 'graph', [CompletionResultType]::ParameterValue, 'Inspect and validate knowledge_graph.json')
            [CompletionResult]::new('cross-refs', 'cross-refs', [CompletionResultType]::ParameterValue, 'Inspect and validate cross_refs.json')
            [CompletionResult]::new('skills', 'skills', [CompletionResultType]::ParameterValue, 'Manage and validate the skills/ layer')
            [CompletionResult]::new('l10n', 'l10n', [CompletionResultType]::ParameterValue, 'Manage and validate the l10n/ localization layer')
            [CompletionResult]::new('new', 'new', [CompletionResultType]::ParameterValue, 'Scaffold + LLM-generate a complete pack in one command')
            [CompletionResult]::new('generate', 'generate', [CompletionResultType]::ParameterValue, 'Run (or re-run) LLM generation on an existing pack')
            [CompletionResult]::new('fix', 'fix', [CompletionResultType]::ParameterValue, 'Failure-aware chunk regeneration using eval results')
            [CompletionResult]::new('review', 'review', [CompletionResultType]::ParameterValue, 'Generate evidence drafts for manual review gates')
            [CompletionResult]::new('keygen', 'keygen', [CompletionResultType]::ParameterValue, 'Generate an Ed25519 keypair for signing packs')
            [CompletionResult]::new('sign', 'sign', [CompletionResultType]::ParameterValue, 'Sign a built archive with an Ed25519 private key')
            [CompletionResult]::new('install', 'install', [CompletionResultType]::ParameterValue, 'Install a pack from the registry')
            [CompletionResult]::new('uninstall', 'uninstall', [CompletionResultType]::ParameterValue, 'Remove an installed pack')
            [CompletionResult]::new('update', 'update', [CompletionResultType]::ParameterValue, 'Re-resolve and update installed packs to satisfy lock-file constraints')
            [CompletionResult]::new('publish', 'publish', [CompletionResultType]::ParameterValue, 'Publish a built and signed pack to the registry')
            [CompletionResult]::new('yank', 'yank', [CompletionResultType]::ParameterValue, 'Mark a published version as yanked')
            [CompletionResult]::new('registry', 'registry', [CompletionResultType]::ParameterValue, 'Registry account and pack management (login, logout, keys, access)')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;init' {
            [CompletionResult]::new('--domain', '--domain', [CompletionResultType]::ParameterName, 'Top-level domain category (e.g. "Health", "Finance")')
            [CompletionResult]::new('--out', '--out', [CompletionResultType]::ParameterName, 'Output directory (default: ./<name-slug>/)')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('--extras', '--extras', [CompletionResultType]::ParameterName, 'Also scaffold optional recommended assets: eval_set.jsonl, knowledge_graph.json, human/handbook.md, README.md, CHANGELOG.md')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'Overwrite if directory already exists')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;info' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('--check', '--check', [CompletionResultType]::ParameterName, 'Exit non-zero if any required machine asset is missing')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;list' {
            [CompletionResult]::new('--domain', '--domain', [CompletionResultType]::ParameterName, 'Filter by domain name')
            [CompletionResult]::new('--tier', '--tier', [CompletionResultType]::ParameterName, 'Filter by tier tag (e.g. starter, pro)')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;validate' {
            [CompletionResult]::new('--gate', '--gate', [CompletionResultType]::ParameterName, 'Run only a specific quality gate (4, 7, or 8)')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('--strict', '--strict', [CompletionResultType]::ParameterName, 'Treat warnings as errors')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;search' {
            [CompletionResult]::new('--type', '--type', [CompletionResultType]::ParameterName, 'Narrow results to a specific asset type')
            [CompletionResult]::new('--limit', '--limit', [CompletionResultType]::ParameterName, 'Maximum number of results')
            [CompletionResult]::new('--domain', '--domain', [CompletionResultType]::ParameterName, 'domain')
            [CompletionResult]::new('--conformance', '--conformance', [CompletionResultType]::ParameterName, 'conformance')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('--registry', '--registry', [CompletionResultType]::ParameterName, 'Search the registry index instead of a local pack')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;get' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('--by-id', '--by-id', [CompletionResultType]::ParameterName, 'Fetch by ID field instead of title search')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;inject' {
            [CompletionResult]::new('--scope', '--scope', [CompletionResultType]::ParameterName, 'Content scope: system-prompt | full | minimal | chunks')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Wrapping format: markdown | xml | json')
            [CompletionResult]::new('--max-tokens', '--max-tokens', [CompletionResultType]::ParameterName, 'Truncate to fit within a token budget, dropping lowest-confidence chunks first')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('--count-tokens', '--count-tokens', [CompletionResultType]::ParameterName, 'Print estimated token count before the block')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;export' {
            [CompletionResult]::new('--out', '--out', [CompletionResultType]::ParameterName, 'Output directory')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;okf' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('export', 'export', [CompletionResultType]::ParameterValue, 'Generate okf/ from machine/')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'Check OKF conformance (Gate 8)')
            [CompletionResult]::new('stats', 'stats', [CompletionResultType]::ParameterValue, 'Print concept count by type')
            [CompletionResult]::new('links', 'links', [CompletionResultType]::ParameterValue, 'Check cross-link integrity')
            [CompletionResult]::new('browse', 'browse', [CompletionResultType]::ParameterValue, 'Interactive terminal browser')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;okf;export' {
            [CompletionResult]::new('--out', '--out', [CompletionResultType]::ParameterName, 'out')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;okf;validate' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;okf;stats' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;okf;links' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;okf;browse' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;okf;help' {
            [CompletionResult]::new('export', 'export', [CompletionResultType]::ParameterValue, 'Generate okf/ from machine/')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'Check OKF conformance (Gate 8)')
            [CompletionResult]::new('stats', 'stats', [CompletionResultType]::ParameterValue, 'Print concept count by type')
            [CompletionResult]::new('links', 'links', [CompletionResultType]::ParameterValue, 'Check cross-link integrity')
            [CompletionResult]::new('browse', 'browse', [CompletionResultType]::ParameterValue, 'Interactive terminal browser')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;okf;help;export' {
            break
        }
        'dkp;okf;help;validate' {
            break
        }
        'dkp;okf;help;stats' {
            break
        }
        'dkp;okf;help;links' {
            break
        }
        'dkp;okf;help;browse' {
            break
        }
        'dkp;okf;help;help' {
            break
        }
        'dkp;chunk' {
            [CompletionResult]::new('--top', '--top', [CompletionResultType]::ParameterName, 'Number of chunks to return')
            [CompletionResult]::new('--min-confidence', '--min-confidence', [CompletionResultType]::ParameterName, 'Filter by minimum confidence score')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;eval' {
            [CompletionResult]::new('--provider', '--provider', [CompletionResultType]::ParameterName, 'provider')
            [CompletionResult]::new('--model', '--model', [CompletionResultType]::ParameterName, 'model')
            [CompletionResult]::new('--base-url', '--base-url', [CompletionResultType]::ParameterName, 'base-url')
            [CompletionResult]::new('--api-key', '--api-key', [CompletionResultType]::ParameterName, 'api-key')
            [CompletionResult]::new('--pairs', '--pairs', [CompletionResultType]::ParameterName, 'Run only first N eval pairs (default: all)')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('--baseline-only', '--baseline-only', [CompletionResultType]::ParameterName, 'Score without DKP context (baseline only)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;prompt' {
            [CompletionResult]::new('--provider', '--provider', [CompletionResultType]::ParameterName, 'provider')
            [CompletionResult]::new('--model', '--model', [CompletionResultType]::ParameterName, 'model')
            [CompletionResult]::new('--api-key', '--api-key', [CompletionResultType]::ParameterName, 'api-key')
            [CompletionResult]::new('--max-tokens', '--max-tokens', [CompletionResultType]::ParameterName, 'Token budget for injected pack context')
            [CompletionResult]::new('--scope', '--scope', [CompletionResultType]::ParameterName, 'Context scope: system-prompt | full | chunks | minimal')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;diff' {
            [CompletionResult]::new('--type', '--type', [CompletionResultType]::ParameterName, 'Diff only one asset type (term, chunk, rule, constraint)')
            [CompletionResult]::new('--threshold', '--threshold', [CompletionResultType]::ParameterName, 'Content-drift percentage threshold for "modified" classification')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;build' {
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Archive format: zip | tar.gz')
            [CompletionResult]::new('--out', '--out', [CompletionResultType]::ParameterName, 'Output directory (default: build/)')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('--no-human', '--no-human', [CompletionResultType]::ParameterName, 'Exclude human/ assets (machine-only distribution)')
            [CompletionResult]::new('--gen-mcp-manifest', '--gen-mcp-manifest', [CompletionResultType]::ParameterName, 'Regenerate machine/mcp_manifest.json before packaging')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;release-check' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('--strict', '--strict', [CompletionResultType]::ParameterName, 'Fail on warnings')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;rights' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('status', 'status', [CompletionResultType]::ParameterValue, 'Summary of sources and rights coverage')
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'Flag entries with missing fields or expired rights')
            [CompletionResult]::new('add-source', 'add-source', [CompletionResultType]::ParameterValue, 'Interactive prompt to add a source entry')
            [CompletionResult]::new('report', 'report', [CompletionResultType]::ParameterValue, 'Formatted compliance report for human review')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;rights;status' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;rights;check' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;rights;add-source' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;rights;report' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;rights;help' {
            [CompletionResult]::new('status', 'status', [CompletionResultType]::ParameterValue, 'Summary of sources and rights coverage')
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'Flag entries with missing fields or expired rights')
            [CompletionResult]::new('add-source', 'add-source', [CompletionResultType]::ParameterValue, 'Interactive prompt to add a source entry')
            [CompletionResult]::new('report', 'report', [CompletionResultType]::ParameterValue, 'Formatted compliance report for human review')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;rights;help;status' {
            break
        }
        'dkp;rights;help;check' {
            break
        }
        'dkp;rights;help;add-source' {
            break
        }
        'dkp;rights;help;report' {
            break
        }
        'dkp;rights;help;help' {
            break
        }
        'dkp;mcp-manifest' {
            [CompletionResult]::new('--out', '--out', [CompletionResultType]::ParameterName, 'Write to a custom path instead of machine/mcp_manifest.json')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('--dry-run', '--dry-run', [CompletionResultType]::ParameterName, 'Print what would be written without writing')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;serve' {
            [CompletionResult]::new('--transport', '--transport', [CompletionResultType]::ParameterName, 'Transport mechanism: stdio | http')
            [CompletionResult]::new('--port', '--port', [CompletionResultType]::ParameterName, 'HTTP server port (when --transport http)')
            [CompletionResult]::new('--auth-token', '--auth-token', [CompletionResultType]::ParameterName, 'Static bearer token for authentication')
            [CompletionResult]::new('--log-level', '--log-level', [CompletionResultType]::ParameterName, 'Server log verbosity: debug | info | warn')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('--readonly', '--readonly', [CompletionResultType]::ParameterName, 'Expose resources only; do not expose any tools')
            [CompletionResult]::new('--allow-unsigned', '--allow-unsigned', [CompletionResultType]::ParameterName, 'Allow running non-WASM procedures from unsigned bundles (dev/testing only)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;tui' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;webui' {
            [CompletionResult]::new('--port', '--port', [CompletionResultType]::ParameterName, 'Port to listen on (0 = OS-assigned random port)')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('--no-open', '--no-open', [CompletionResultType]::ParameterName, 'Do not automatically open the browser')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;run' {
            [CompletionResult]::new('--input', '--input', [CompletionResultType]::ParameterName, 'JSON object to pass as stdin to the procedure (default: {})')
            [CompletionResult]::new('--timeout-ms', '--timeout-ms', [CompletionResultType]::ParameterName, 'Override the wall-clock timeout in milliseconds')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('--allow-unsigned', '--allow-unsigned', [CompletionResultType]::ParameterName, 'Allow running non-WASM procedures from unsigned bundles (dev/testing only)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;procedures' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all procedures defined in machine/procedures/')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'Validate procedure file completeness and schema correctness')
            [CompletionResult]::new('new', 'new', [CompletionResultType]::ParameterValue, 'Scaffold a new Rust WASI procedure project')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;procedures;list' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;procedures;validate' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;procedures;new' {
            [CompletionResult]::new('--title', '--title', [CompletionResultType]::ParameterName, 'Human-readable title for the procedure')
            [CompletionResult]::new('--description', '--description', [CompletionResultType]::ParameterName, 'One-sentence description of what the procedure does')
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Procedure language/runtime: wasm | python | javascript')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;procedures;help' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all procedures defined in machine/procedures/')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'Validate procedure file completeness and schema correctness')
            [CompletionResult]::new('new', 'new', [CompletionResultType]::ParameterValue, 'Scaffold a new Rust WASI procedure project')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;procedures;help;list' {
            break
        }
        'dkp;procedures;help;validate' {
            break
        }
        'dkp;procedures;help;new' {
            break
        }
        'dkp;procedures;help;help' {
            break
        }
        'dkp;graph' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('stats', 'stats', [CompletionResultType]::ParameterValue, 'Print node and edge counts by type')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'Check all edges resolve to known concept IDs (Gate 4)')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all nodes with their type and id')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;graph;stats' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;graph;validate' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;graph;list' {
            [CompletionResult]::new('--type', '--type', [CompletionResultType]::ParameterName, 'type')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;graph;help' {
            [CompletionResult]::new('stats', 'stats', [CompletionResultType]::ParameterValue, 'Print node and edge counts by type')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'Check all edges resolve to known concept IDs (Gate 4)')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all nodes with their type and id')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;graph;help;stats' {
            break
        }
        'dkp;graph;help;validate' {
            break
        }
        'dkp;graph;help;list' {
            break
        }
        'dkp;graph;help;help' {
            break
        }
        'dkp;cross-refs' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all declared pack dependencies')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'Check local_id values resolve to concepts in this bundle (Gate 4)')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;cross-refs;list' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;cross-refs;validate' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;cross-refs;help' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all declared pack dependencies')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'Check local_id values resolve to concepts in this bundle (Gate 4)')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;cross-refs;help;list' {
            break
        }
        'dkp;cross-refs;help;validate' {
            break
        }
        'dkp;cross-refs;help;help' {
            break
        }
        'dkp;skills' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all skills in skills/')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'Check SKILL.md format conformance')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Print a specific skill''s SKILL.md')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;skills;list' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;skills;validate' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;skills;show' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;skills;help' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all skills in skills/')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'Check SKILL.md format conformance')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Print a specific skill''s SKILL.md')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;skills;help;list' {
            break
        }
        'dkp;skills;help;validate' {
            break
        }
        'dkp;skills;help;show' {
            break
        }
        'dkp;skills;help;help' {
            break
        }
        'dkp;l10n' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List available locales')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'Check locale content doesn''t contradict the base pack')
            [CompletionResult]::new('export', 'export', [CompletionResultType]::ParameterValue, 'Export a locale-specific bundle')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;l10n;list' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;l10n;validate' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;l10n;export' {
            [CompletionResult]::new('--out', '--out', [CompletionResultType]::ParameterName, 'out')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;l10n;help' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List available locales')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'Check locale content doesn''t contradict the base pack')
            [CompletionResult]::new('export', 'export', [CompletionResultType]::ParameterValue, 'Export a locale-specific bundle')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;l10n;help;list' {
            break
        }
        'dkp;l10n;help;validate' {
            break
        }
        'dkp;l10n;help;export' {
            break
        }
        'dkp;l10n;help;help' {
            break
        }
        'dkp;new' {
            [CompletionResult]::new('--domain', '--domain', [CompletionResultType]::ParameterName, 'Domain category (e.g. "Kubernetes", "Clinical Nutrition")')
            [CompletionResult]::new('--dir', '--dir', [CompletionResultType]::ParameterName, 'Output directory (default: ./<name-slug>/)')
            [CompletionResult]::new('--api-key', '--api-key', [CompletionResultType]::ParameterName, 'api-key')
            [CompletionResult]::new('--base-url', '--base-url', [CompletionResultType]::ParameterName, 'base-url')
            [CompletionResult]::new('--model', '--model', [CompletionResultType]::ParameterName, 'model')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('--overwrite', '--overwrite', [CompletionResultType]::ParameterName, 'Overwrite already-generated assets')
            [CompletionResult]::new('--skip-validate', '--skip-validate', [CompletionResultType]::ParameterName, 'Skip validation step')
            [CompletionResult]::new('--skip-package', '--skip-package', [CompletionResultType]::ParameterName, 'Skip packaging step')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;generate' {
            [CompletionResult]::new('--api-key', '--api-key', [CompletionResultType]::ParameterName, 'api-key')
            [CompletionResult]::new('--base-url', '--base-url', [CompletionResultType]::ParameterName, 'base-url')
            [CompletionResult]::new('--model', '--model', [CompletionResultType]::ParameterName, 'model')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('--overwrite', '--overwrite', [CompletionResultType]::ParameterName, 'Overwrite existing assets')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;fix' {
            [CompletionResult]::new('--api-key', '--api-key', [CompletionResultType]::ParameterName, 'api-key')
            [CompletionResult]::new('--base-url', '--base-url', [CompletionResultType]::ParameterName, 'base-url')
            [CompletionResult]::new('--model', '--model', [CompletionResultType]::ParameterName, 'model')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;review' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;keygen' {
            [CompletionResult]::new('--out', '--out', [CompletionResultType]::ParameterName, 'Write keys to a custom directory (default: ~/.dkp/)')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'Overwrite existing keys without prompting')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;sign' {
            [CompletionResult]::new('--key', '--key', [CompletionResultType]::ParameterName, 'Path to Ed25519 private key (default: ~/.dkp/private.key)')
            [CompletionResult]::new('--out', '--out', [CompletionResultType]::ParameterName, 'Write signature to a custom path (default: <archive-dir>/bundle.sig)')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;install' {
            [CompletionResult]::new('--out', '--out', [CompletionResultType]::ParameterName, 'Install to a custom directory')
            [CompletionResult]::new('--registry', '--registry', [CompletionResultType]::ParameterName, 'Override registry URL')
            [CompletionResult]::new('--token', '--token', [CompletionResultType]::ParameterName, 'Registry API token')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'Install to global store (~/.dkp/packs/)')
            [CompletionResult]::new('--global', '--global', [CompletionResultType]::ParameterName, 'Install to global store (~/.dkp/packs/)')
            [CompletionResult]::new('--no-verify', '--no-verify', [CompletionResultType]::ParameterName, 'Skip signature verification (NOT RECOMMENDED)')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;uninstall' {
            [CompletionResult]::new('--out', '--out', [CompletionResultType]::ParameterName, 'Remove from a custom directory')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'Remove from global store')
            [CompletionResult]::new('--global', '--global', [CompletionResultType]::ParameterName, 'Remove from global store')
            [CompletionResult]::new('--all-versions', '--all-versions', [CompletionResultType]::ParameterName, 'Remove all installed versions')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;update' {
            [CompletionResult]::new('--registry', '--registry', [CompletionResultType]::ParameterName, 'Override registry URL')
            [CompletionResult]::new('--token', '--token', [CompletionResultType]::ParameterName, 'Registry API token')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;publish' {
            [CompletionResult]::new('--url', '--url', [CompletionResultType]::ParameterName, 'HTTPS URL to the hosted archive (publisher-controlled storage)')
            [CompletionResult]::new('--build-dir', '--build-dir', [CompletionResultType]::ParameterName, 'Directory containing checksums.json and bundle.sig (default: <pack>/build/)')
            [CompletionResult]::new('--registry', '--registry', [CompletionResultType]::ParameterName, 'Override registry URL')
            [CompletionResult]::new('--token', '--token', [CompletionResultType]::ParameterName, 'Registry API token')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('--private', '--private', [CompletionResultType]::ParameterName, 'Set pack visibility to private')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;yank' {
            [CompletionResult]::new('--reason', '--reason', [CompletionResultType]::ParameterName, 'Reason shown to consumers who attempt to install this version')
            [CompletionResult]::new('--registry', '--registry', [CompletionResultType]::ParameterName, 'Override registry URL')
            [CompletionResult]::new('--token', '--token', [CompletionResultType]::ParameterName, 'Registry API token')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;registry' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('register', 'register', [CompletionResultType]::ParameterValue, 'Create a new publisher account and save the API key to ~/.dkp/credentials')
            [CompletionResult]::new('login', 'login', [CompletionResultType]::ParameterValue, 'Authenticate with an existing account and save API key to ~/.dkp/credentials')
            [CompletionResult]::new('logout', 'logout', [CompletionResultType]::ParameterValue, 'Remove saved credentials')
            [CompletionResult]::new('token', 'token', [CompletionResultType]::ParameterValue, 'Rotate your API key')
            [CompletionResult]::new('keys', 'keys', [CompletionResultType]::ParameterValue, 'Manage Ed25519 public keys registered with the registry')
            [CompletionResult]::new('pack', 'pack', [CompletionResultType]::ParameterValue, 'Pack-level management subcommands')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;registry;register' {
            [CompletionResult]::new('--email', '--email', [CompletionResultType]::ParameterName, 'email')
            [CompletionResult]::new('--registry', '--registry', [CompletionResultType]::ParameterName, 'registry')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;registry;login' {
            [CompletionResult]::new('--email', '--email', [CompletionResultType]::ParameterName, 'email')
            [CompletionResult]::new('--registry', '--registry', [CompletionResultType]::ParameterName, 'registry')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;registry;logout' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;registry;token' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('rotate', 'rotate', [CompletionResultType]::ParameterValue, 'rotate')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;registry;token;rotate' {
            [CompletionResult]::new('--registry', '--registry', [CompletionResultType]::ParameterName, 'registry')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;registry;token;help' {
            [CompletionResult]::new('rotate', 'rotate', [CompletionResultType]::ParameterValue, 'rotate')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;registry;token;help;rotate' {
            break
        }
        'dkp;registry;token;help;help' {
            break
        }
        'dkp;registry;keys' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'add')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;registry;keys;add' {
            [CompletionResult]::new('--key', '--key', [CompletionResultType]::ParameterName, 'key')
            [CompletionResult]::new('--registry', '--registry', [CompletionResultType]::ParameterName, 'registry')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;registry;keys;help' {
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'add')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;registry;keys;help;add' {
            break
        }
        'dkp;registry;keys;help;help' {
            break
        }
        'dkp;registry;pack' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('versions', 'versions', [CompletionResultType]::ParameterValue, 'List all published versions of a pack')
            [CompletionResult]::new('set-visibility', 'set-visibility', [CompletionResultType]::ParameterValue, 'Set pack visibility (public or private)')
            [CompletionResult]::new('grant', 'grant', [CompletionResultType]::ParameterValue, 'Grant access to a private pack')
            [CompletionResult]::new('revoke', 'revoke', [CompletionResultType]::ParameterValue, 'Revoke access to a private pack')
            [CompletionResult]::new('access', 'access', [CompletionResultType]::ParameterValue, 'List accounts with access to a private pack')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;registry;pack;versions' {
            [CompletionResult]::new('--registry', '--registry', [CompletionResultType]::ParameterName, 'registry')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;registry;pack;set-visibility' {
            [CompletionResult]::new('--registry', '--registry', [CompletionResultType]::ParameterName, 'registry')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;registry;pack;grant' {
            [CompletionResult]::new('--to', '--to', [CompletionResultType]::ParameterName, 'to')
            [CompletionResult]::new('--registry', '--registry', [CompletionResultType]::ParameterName, 'registry')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;registry;pack;revoke' {
            [CompletionResult]::new('--from', '--from', [CompletionResultType]::ParameterName, 'from')
            [CompletionResult]::new('--registry', '--registry', [CompletionResultType]::ParameterName, 'registry')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;registry;pack;access' {
            [CompletionResult]::new('--registry', '--registry', [CompletionResultType]::ParameterName, 'registry')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--audience', '--audience', [CompletionResultType]::ParameterName, 'Filter content to assets tagged for a specific audience profile')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress informational output; print only results')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Print debug info (schema paths, provider calls, etc.)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'dkp;registry;pack;help' {
            [CompletionResult]::new('versions', 'versions', [CompletionResultType]::ParameterValue, 'List all published versions of a pack')
            [CompletionResult]::new('set-visibility', 'set-visibility', [CompletionResultType]::ParameterValue, 'Set pack visibility (public or private)')
            [CompletionResult]::new('grant', 'grant', [CompletionResultType]::ParameterValue, 'Grant access to a private pack')
            [CompletionResult]::new('revoke', 'revoke', [CompletionResultType]::ParameterValue, 'Revoke access to a private pack')
            [CompletionResult]::new('access', 'access', [CompletionResultType]::ParameterValue, 'List accounts with access to a private pack')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;registry;pack;help;versions' {
            break
        }
        'dkp;registry;pack;help;set-visibility' {
            break
        }
        'dkp;registry;pack;help;grant' {
            break
        }
        'dkp;registry;pack;help;revoke' {
            break
        }
        'dkp;registry;pack;help;access' {
            break
        }
        'dkp;registry;pack;help;help' {
            break
        }
        'dkp;registry;help' {
            [CompletionResult]::new('register', 'register', [CompletionResultType]::ParameterValue, 'Create a new publisher account and save the API key to ~/.dkp/credentials')
            [CompletionResult]::new('login', 'login', [CompletionResultType]::ParameterValue, 'Authenticate with an existing account and save API key to ~/.dkp/credentials')
            [CompletionResult]::new('logout', 'logout', [CompletionResultType]::ParameterValue, 'Remove saved credentials')
            [CompletionResult]::new('token', 'token', [CompletionResultType]::ParameterValue, 'Rotate your API key')
            [CompletionResult]::new('keys', 'keys', [CompletionResultType]::ParameterValue, 'Manage Ed25519 public keys registered with the registry')
            [CompletionResult]::new('pack', 'pack', [CompletionResultType]::ParameterValue, 'Pack-level management subcommands')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;registry;help;register' {
            break
        }
        'dkp;registry;help;login' {
            break
        }
        'dkp;registry;help;logout' {
            break
        }
        'dkp;registry;help;token' {
            [CompletionResult]::new('rotate', 'rotate', [CompletionResultType]::ParameterValue, 'rotate')
            break
        }
        'dkp;registry;help;token;rotate' {
            break
        }
        'dkp;registry;help;keys' {
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'add')
            break
        }
        'dkp;registry;help;keys;add' {
            break
        }
        'dkp;registry;help;pack' {
            [CompletionResult]::new('versions', 'versions', [CompletionResultType]::ParameterValue, 'List all published versions of a pack')
            [CompletionResult]::new('set-visibility', 'set-visibility', [CompletionResultType]::ParameterValue, 'Set pack visibility (public or private)')
            [CompletionResult]::new('grant', 'grant', [CompletionResultType]::ParameterValue, 'Grant access to a private pack')
            [CompletionResult]::new('revoke', 'revoke', [CompletionResultType]::ParameterValue, 'Revoke access to a private pack')
            [CompletionResult]::new('access', 'access', [CompletionResultType]::ParameterValue, 'List accounts with access to a private pack')
            break
        }
        'dkp;registry;help;pack;versions' {
            break
        }
        'dkp;registry;help;pack;set-visibility' {
            break
        }
        'dkp;registry;help;pack;grant' {
            break
        }
        'dkp;registry;help;pack;revoke' {
            break
        }
        'dkp;registry;help;pack;access' {
            break
        }
        'dkp;registry;help;help' {
            break
        }
        'dkp;help' {
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Scaffold a new DKP pack directory with all required files')
            [CompletionResult]::new('info', 'info', [CompletionResultType]::ParameterValue, 'Print a summary of a pack (name, version, asset counts, compliance)')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all packs under a root directory')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'Run schema and compliance checks; exit non-zero on failure')
            [CompletionResult]::new('search', 'search', [CompletionResultType]::ParameterValue, 'Full-text BM25 search across machine assets (or registry with --registry)')
            [CompletionResult]::new('get', 'get', [CompletionResultType]::ParameterValue, 'Retrieve a specific asset or all assets of a type')
            [CompletionResult]::new('inject', 'inject', [CompletionResultType]::ParameterValue, 'Print a ready-to-inject LLM context block')
            [CompletionResult]::new('export', 'export', [CompletionResultType]::ParameterValue, 'Convert machine assets to another format (okf, langchain, llamaindex, …)')
            [CompletionResult]::new('okf', 'okf', [CompletionResultType]::ParameterValue, 'OKF-specific operations (export, validate, stats, links, browse)')
            [CompletionResult]::new('chunk', 'chunk', [CompletionResultType]::ParameterValue, 'Retrieve the top-N most relevant retrieval chunks for a query')
            [CompletionResult]::new('eval', 'eval', [CompletionResultType]::ParameterValue, 'Run eval set against baseline and grounded prompts; print delta report')
            [CompletionResult]::new('prompt', 'prompt', [CompletionResultType]::ParameterValue, 'Interactive grounded prompt REPL for testing a pack')
            [CompletionResult]::new('diff', 'diff', [CompletionResultType]::ParameterValue, 'Compare two pack versions and report what changed')
            [CompletionResult]::new('build', 'build', [CompletionResultType]::ParameterValue, 'Package a pack into a versioned archive with checksums.json')
            [CompletionResult]::new('release-check', 'release-check', [CompletionResultType]::ParameterValue, 'Pre-release compliance checklist (runs all gates, checks human fields)')
            [CompletionResult]::new('rights', 'rights', [CompletionResultType]::ParameterValue, 'Source and rights log operations')
            [CompletionResult]::new('mcp-manifest', 'mcp-manifest', [CompletionResultType]::ParameterValue, 'Generate or regenerate machine/mcp_manifest.json')
            [CompletionResult]::new('serve', 'serve', [CompletionResultType]::ParameterValue, 'Start the pack as an MCP server (requires --features mcp)')
            [CompletionResult]::new('tui', 'tui', [CompletionResultType]::ParameterValue, 'Interactive TUI browser (requires --features tui)')
            [CompletionResult]::new('webui', 'webui', [CompletionResultType]::ParameterValue, 'Browse a pack in a local web UI (requires --features webui)')
            [CompletionResult]::new('run', 'run', [CompletionResultType]::ParameterValue, 'Invoke a WASM/WASI procedure from machine/procedures/')
            [CompletionResult]::new('procedures', 'procedures', [CompletionResultType]::ParameterValue, 'List, validate, and scaffold executable procedures')
            [CompletionResult]::new('graph', 'graph', [CompletionResultType]::ParameterValue, 'Inspect and validate knowledge_graph.json')
            [CompletionResult]::new('cross-refs', 'cross-refs', [CompletionResultType]::ParameterValue, 'Inspect and validate cross_refs.json')
            [CompletionResult]::new('skills', 'skills', [CompletionResultType]::ParameterValue, 'Manage and validate the skills/ layer')
            [CompletionResult]::new('l10n', 'l10n', [CompletionResultType]::ParameterValue, 'Manage and validate the l10n/ localization layer')
            [CompletionResult]::new('new', 'new', [CompletionResultType]::ParameterValue, 'Scaffold + LLM-generate a complete pack in one command')
            [CompletionResult]::new('generate', 'generate', [CompletionResultType]::ParameterValue, 'Run (or re-run) LLM generation on an existing pack')
            [CompletionResult]::new('fix', 'fix', [CompletionResultType]::ParameterValue, 'Failure-aware chunk regeneration using eval results')
            [CompletionResult]::new('review', 'review', [CompletionResultType]::ParameterValue, 'Generate evidence drafts for manual review gates')
            [CompletionResult]::new('keygen', 'keygen', [CompletionResultType]::ParameterValue, 'Generate an Ed25519 keypair for signing packs')
            [CompletionResult]::new('sign', 'sign', [CompletionResultType]::ParameterValue, 'Sign a built archive with an Ed25519 private key')
            [CompletionResult]::new('install', 'install', [CompletionResultType]::ParameterValue, 'Install a pack from the registry')
            [CompletionResult]::new('uninstall', 'uninstall', [CompletionResultType]::ParameterValue, 'Remove an installed pack')
            [CompletionResult]::new('update', 'update', [CompletionResultType]::ParameterValue, 'Re-resolve and update installed packs to satisfy lock-file constraints')
            [CompletionResult]::new('publish', 'publish', [CompletionResultType]::ParameterValue, 'Publish a built and signed pack to the registry')
            [CompletionResult]::new('yank', 'yank', [CompletionResultType]::ParameterValue, 'Mark a published version as yanked')
            [CompletionResult]::new('registry', 'registry', [CompletionResultType]::ParameterValue, 'Registry account and pack management (login, logout, keys, access)')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'dkp;help;init' {
            break
        }
        'dkp;help;info' {
            break
        }
        'dkp;help;list' {
            break
        }
        'dkp;help;validate' {
            break
        }
        'dkp;help;search' {
            break
        }
        'dkp;help;get' {
            break
        }
        'dkp;help;inject' {
            break
        }
        'dkp;help;export' {
            break
        }
        'dkp;help;okf' {
            [CompletionResult]::new('export', 'export', [CompletionResultType]::ParameterValue, 'Generate okf/ from machine/')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'Check OKF conformance (Gate 8)')
            [CompletionResult]::new('stats', 'stats', [CompletionResultType]::ParameterValue, 'Print concept count by type')
            [CompletionResult]::new('links', 'links', [CompletionResultType]::ParameterValue, 'Check cross-link integrity')
            [CompletionResult]::new('browse', 'browse', [CompletionResultType]::ParameterValue, 'Interactive terminal browser')
            break
        }
        'dkp;help;okf;export' {
            break
        }
        'dkp;help;okf;validate' {
            break
        }
        'dkp;help;okf;stats' {
            break
        }
        'dkp;help;okf;links' {
            break
        }
        'dkp;help;okf;browse' {
            break
        }
        'dkp;help;chunk' {
            break
        }
        'dkp;help;eval' {
            break
        }
        'dkp;help;prompt' {
            break
        }
        'dkp;help;diff' {
            break
        }
        'dkp;help;build' {
            break
        }
        'dkp;help;release-check' {
            break
        }
        'dkp;help;rights' {
            [CompletionResult]::new('status', 'status', [CompletionResultType]::ParameterValue, 'Summary of sources and rights coverage')
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'Flag entries with missing fields or expired rights')
            [CompletionResult]::new('add-source', 'add-source', [CompletionResultType]::ParameterValue, 'Interactive prompt to add a source entry')
            [CompletionResult]::new('report', 'report', [CompletionResultType]::ParameterValue, 'Formatted compliance report for human review')
            break
        }
        'dkp;help;rights;status' {
            break
        }
        'dkp;help;rights;check' {
            break
        }
        'dkp;help;rights;add-source' {
            break
        }
        'dkp;help;rights;report' {
            break
        }
        'dkp;help;mcp-manifest' {
            break
        }
        'dkp;help;serve' {
            break
        }
        'dkp;help;tui' {
            break
        }
        'dkp;help;webui' {
            break
        }
        'dkp;help;run' {
            break
        }
        'dkp;help;procedures' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all procedures defined in machine/procedures/')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'Validate procedure file completeness and schema correctness')
            [CompletionResult]::new('new', 'new', [CompletionResultType]::ParameterValue, 'Scaffold a new Rust WASI procedure project')
            break
        }
        'dkp;help;procedures;list' {
            break
        }
        'dkp;help;procedures;validate' {
            break
        }
        'dkp;help;procedures;new' {
            break
        }
        'dkp;help;graph' {
            [CompletionResult]::new('stats', 'stats', [CompletionResultType]::ParameterValue, 'Print node and edge counts by type')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'Check all edges resolve to known concept IDs (Gate 4)')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all nodes with their type and id')
            break
        }
        'dkp;help;graph;stats' {
            break
        }
        'dkp;help;graph;validate' {
            break
        }
        'dkp;help;graph;list' {
            break
        }
        'dkp;help;cross-refs' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all declared pack dependencies')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'Check local_id values resolve to concepts in this bundle (Gate 4)')
            break
        }
        'dkp;help;cross-refs;list' {
            break
        }
        'dkp;help;cross-refs;validate' {
            break
        }
        'dkp;help;skills' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all skills in skills/')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'Check SKILL.md format conformance')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Print a specific skill''s SKILL.md')
            break
        }
        'dkp;help;skills;list' {
            break
        }
        'dkp;help;skills;validate' {
            break
        }
        'dkp;help;skills;show' {
            break
        }
        'dkp;help;l10n' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List available locales')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'Check locale content doesn''t contradict the base pack')
            [CompletionResult]::new('export', 'export', [CompletionResultType]::ParameterValue, 'Export a locale-specific bundle')
            break
        }
        'dkp;help;l10n;list' {
            break
        }
        'dkp;help;l10n;validate' {
            break
        }
        'dkp;help;l10n;export' {
            break
        }
        'dkp;help;new' {
            break
        }
        'dkp;help;generate' {
            break
        }
        'dkp;help;fix' {
            break
        }
        'dkp;help;review' {
            break
        }
        'dkp;help;keygen' {
            break
        }
        'dkp;help;sign' {
            break
        }
        'dkp;help;install' {
            break
        }
        'dkp;help;uninstall' {
            break
        }
        'dkp;help;update' {
            break
        }
        'dkp;help;publish' {
            break
        }
        'dkp;help;yank' {
            break
        }
        'dkp;help;registry' {
            [CompletionResult]::new('register', 'register', [CompletionResultType]::ParameterValue, 'Create a new publisher account and save the API key to ~/.dkp/credentials')
            [CompletionResult]::new('login', 'login', [CompletionResultType]::ParameterValue, 'Authenticate with an existing account and save API key to ~/.dkp/credentials')
            [CompletionResult]::new('logout', 'logout', [CompletionResultType]::ParameterValue, 'Remove saved credentials')
            [CompletionResult]::new('token', 'token', [CompletionResultType]::ParameterValue, 'Rotate your API key')
            [CompletionResult]::new('keys', 'keys', [CompletionResultType]::ParameterValue, 'Manage Ed25519 public keys registered with the registry')
            [CompletionResult]::new('pack', 'pack', [CompletionResultType]::ParameterValue, 'Pack-level management subcommands')
            break
        }
        'dkp;help;registry;register' {
            break
        }
        'dkp;help;registry;login' {
            break
        }
        'dkp;help;registry;logout' {
            break
        }
        'dkp;help;registry;token' {
            [CompletionResult]::new('rotate', 'rotate', [CompletionResultType]::ParameterValue, 'rotate')
            break
        }
        'dkp;help;registry;token;rotate' {
            break
        }
        'dkp;help;registry;keys' {
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'add')
            break
        }
        'dkp;help;registry;keys;add' {
            break
        }
        'dkp;help;registry;pack' {
            [CompletionResult]::new('versions', 'versions', [CompletionResultType]::ParameterValue, 'List all published versions of a pack')
            [CompletionResult]::new('set-visibility', 'set-visibility', [CompletionResultType]::ParameterValue, 'Set pack visibility (public or private)')
            [CompletionResult]::new('grant', 'grant', [CompletionResultType]::ParameterValue, 'Grant access to a private pack')
            [CompletionResult]::new('revoke', 'revoke', [CompletionResultType]::ParameterValue, 'Revoke access to a private pack')
            [CompletionResult]::new('access', 'access', [CompletionResultType]::ParameterValue, 'List accounts with access to a private pack')
            break
        }
        'dkp;help;registry;pack;versions' {
            break
        }
        'dkp;help;registry;pack;set-visibility' {
            break
        }
        'dkp;help;registry;pack;grant' {
            break
        }
        'dkp;help;registry;pack;revoke' {
            break
        }
        'dkp;help;registry;pack;access' {
            break
        }
        'dkp;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}

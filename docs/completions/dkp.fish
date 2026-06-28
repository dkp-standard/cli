# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_dkp_global_optspecs
	string join \n output= q/quiet v/verbose audience= h/help V/version
end

function __fish_dkp_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_dkp_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_dkp_using_subcommand
	set -l cmd (__fish_dkp_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c dkp -n "__fish_dkp_needs_command" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_needs_command" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_needs_command" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_needs_command" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_needs_command" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_needs_command" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "init" -d 'Scaffold a new DKP pack directory with all required files'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "info" -d 'Print a summary of a pack (name, version, asset counts, compliance)'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "list" -d 'List all packs under a root directory'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "validate" -d 'Run schema and compliance checks; exit non-zero on failure'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "search" -d 'Full-text BM25 search across machine assets (or registry with --registry)'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "get" -d 'Retrieve a specific asset or all assets of a type'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "inject" -d 'Print a ready-to-inject LLM context block'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "export" -d 'Convert machine assets to another format (okf, langchain, llamaindex, …)'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "okf" -d 'OKF-specific operations (export, validate, stats, links, browse)'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "chunk" -d 'Retrieve the top-N most relevant retrieval chunks for a query'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "eval" -d 'Run eval set against baseline and grounded prompts; print delta report'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "prompt" -d 'Interactive grounded prompt REPL for testing a pack'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "diff" -d 'Compare two pack versions and report what changed'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "build" -d 'Package a pack into a versioned archive with checksums.json'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "release-check" -d 'Pre-release compliance checklist (runs all gates, checks human fields)'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "rights" -d 'Source and rights log operations'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "mcp-manifest" -d 'Generate or regenerate machine/mcp_manifest.json'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "serve" -d 'Start the pack as an MCP server (requires --features mcp)'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "tui" -d 'Interactive TUI browser (requires --features tui)'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "webui" -d 'Browse a pack in a local web UI (requires --features webui)'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "run" -d 'Invoke a WASM/WASI procedure from machine/procedures/'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "procedures" -d 'List, validate, and scaffold executable procedures'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "graph" -d 'Inspect and validate knowledge_graph.json'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "cross-refs" -d 'Inspect and validate cross_refs.json'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "skills" -d 'Manage and validate the skills/ layer'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "l10n" -d 'Manage and validate the l10n/ localization layer'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "new" -d 'Scaffold + LLM-generate a complete pack in one command'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "generate" -d 'Run (or re-run) LLM generation on an existing pack'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "fix" -d 'Failure-aware chunk regeneration using eval results'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "review" -d 'Generate evidence drafts for manual review gates'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "keygen" -d 'Generate an Ed25519 keypair for signing packs'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "sign" -d 'Sign a built archive with an Ed25519 private key'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "install" -d 'Install a pack from the registry'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "uninstall" -d 'Remove an installed pack'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "update" -d 'Re-resolve and update installed packs to satisfy lock-file constraints'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "publish" -d 'Publish a built and signed pack to the registry'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "yank" -d 'Mark a published version as yanked'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "registry" -d 'Registry account and pack management (login, logout, keys, access)'
complete -c dkp -n "__fish_dkp_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dkp -n "__fish_dkp_using_subcommand init" -l domain -d 'Top-level domain category (e.g. "Health", "Finance")' -r
complete -c dkp -n "__fish_dkp_using_subcommand init" -l out -d 'Output directory (default: ./<name-slug>/)' -r -F
complete -c dkp -n "__fish_dkp_using_subcommand init" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand init" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand init" -l extras -d 'Also scaffold optional recommended assets: eval_set.jsonl, knowledge_graph.json, human/handbook.md, README.md, CHANGELOG.md'
complete -c dkp -n "__fish_dkp_using_subcommand init" -l force -d 'Overwrite if directory already exists'
complete -c dkp -n "__fish_dkp_using_subcommand init" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand init" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand init" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand init" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand info" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand info" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand info" -l check -d 'Exit non-zero if any required machine asset is missing'
complete -c dkp -n "__fish_dkp_using_subcommand info" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand info" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand info" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand info" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand list" -l domain -d 'Filter by domain name' -r
complete -c dkp -n "__fish_dkp_using_subcommand list" -l tier -d 'Filter by tier tag (e.g. starter, pro)' -r
complete -c dkp -n "__fish_dkp_using_subcommand list" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand list" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand list" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand list" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand list" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand list" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand validate" -l gate -d 'Run only a specific quality gate (4, 7, or 8)' -r
complete -c dkp -n "__fish_dkp_using_subcommand validate" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand validate" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand validate" -l strict -d 'Treat warnings as errors'
complete -c dkp -n "__fish_dkp_using_subcommand validate" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand validate" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand validate" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand validate" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand search" -l type -d 'Narrow results to a specific asset type' -r
complete -c dkp -n "__fish_dkp_using_subcommand search" -l limit -d 'Maximum number of results' -r
complete -c dkp -n "__fish_dkp_using_subcommand search" -l domain -r
complete -c dkp -n "__fish_dkp_using_subcommand search" -l conformance -r
complete -c dkp -n "__fish_dkp_using_subcommand search" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand search" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand search" -l registry -d 'Search the registry index instead of a local pack'
complete -c dkp -n "__fish_dkp_using_subcommand search" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand search" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand search" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand search" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand get" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand get" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand get" -l by-id -d 'Fetch by ID field instead of title search'
complete -c dkp -n "__fish_dkp_using_subcommand get" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand get" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand get" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand get" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand inject" -l scope -d 'Content scope: system-prompt | full | minimal | chunks' -r
complete -c dkp -n "__fish_dkp_using_subcommand inject" -l format -d 'Wrapping format: markdown | xml | json' -r
complete -c dkp -n "__fish_dkp_using_subcommand inject" -l max-tokens -d 'Truncate to fit within a token budget, dropping lowest-confidence chunks first' -r
complete -c dkp -n "__fish_dkp_using_subcommand inject" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand inject" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand inject" -l count-tokens -d 'Print estimated token count before the block'
complete -c dkp -n "__fish_dkp_using_subcommand inject" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand inject" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand inject" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand inject" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand export" -l out -d 'Output directory' -r -F
complete -c dkp -n "__fish_dkp_using_subcommand export" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand export" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand export" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand export" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand export" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand export" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and not __fish_seen_subcommand_from export validate stats links browse help" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand okf; and not __fish_seen_subcommand_from export validate stats links browse help" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand okf; and not __fish_seen_subcommand_from export validate stats links browse help" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and not __fish_seen_subcommand_from export validate stats links browse help" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and not __fish_seen_subcommand_from export validate stats links browse help" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and not __fish_seen_subcommand_from export validate stats links browse help" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and not __fish_seen_subcommand_from export validate stats links browse help" -a "export" -d 'Generate okf/ from machine/'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and not __fish_seen_subcommand_from export validate stats links browse help" -a "validate" -d 'Check OKF conformance (Gate 8)'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and not __fish_seen_subcommand_from export validate stats links browse help" -a "stats" -d 'Print concept count by type'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and not __fish_seen_subcommand_from export validate stats links browse help" -a "links" -d 'Check cross-link integrity'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and not __fish_seen_subcommand_from export validate stats links browse help" -a "browse" -d 'Interactive terminal browser'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and not __fish_seen_subcommand_from export validate stats links browse help" -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from export" -l out -r -F
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from export" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from export" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from export" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from export" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from export" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from export" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from validate" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from validate" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from validate" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from validate" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from validate" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from validate" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from stats" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from stats" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from stats" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from stats" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from stats" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from stats" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from links" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from links" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from links" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from links" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from links" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from links" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from browse" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from browse" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from browse" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from browse" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from browse" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from browse" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from help" -f -a "export" -d 'Generate okf/ from machine/'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from help" -f -a "validate" -d 'Check OKF conformance (Gate 8)'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from help" -f -a "stats" -d 'Print concept count by type'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from help" -f -a "links" -d 'Check cross-link integrity'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from help" -f -a "browse" -d 'Interactive terminal browser'
complete -c dkp -n "__fish_dkp_using_subcommand okf; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dkp -n "__fish_dkp_using_subcommand chunk" -l top -d 'Number of chunks to return' -r
complete -c dkp -n "__fish_dkp_using_subcommand chunk" -l min-confidence -d 'Filter by minimum confidence score' -r
complete -c dkp -n "__fish_dkp_using_subcommand chunk" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand chunk" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand chunk" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand chunk" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand chunk" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand chunk" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand eval" -l provider -r
complete -c dkp -n "__fish_dkp_using_subcommand eval" -l model -r
complete -c dkp -n "__fish_dkp_using_subcommand eval" -l base-url -r
complete -c dkp -n "__fish_dkp_using_subcommand eval" -l api-key -r
complete -c dkp -n "__fish_dkp_using_subcommand eval" -l pairs -d 'Run only first N eval pairs (default: all)' -r
complete -c dkp -n "__fish_dkp_using_subcommand eval" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand eval" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand eval" -l baseline-only -d 'Score without DKP context (baseline only)'
complete -c dkp -n "__fish_dkp_using_subcommand eval" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand eval" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand eval" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand eval" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand prompt" -l provider -r
complete -c dkp -n "__fish_dkp_using_subcommand prompt" -l model -r
complete -c dkp -n "__fish_dkp_using_subcommand prompt" -l api-key -r
complete -c dkp -n "__fish_dkp_using_subcommand prompt" -l max-tokens -d 'Token budget for injected pack context' -r
complete -c dkp -n "__fish_dkp_using_subcommand prompt" -l scope -d 'Context scope: system-prompt | full | chunks | minimal' -r
complete -c dkp -n "__fish_dkp_using_subcommand prompt" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand prompt" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand prompt" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand prompt" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand prompt" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand prompt" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand diff" -l type -d 'Diff only one asset type (term, chunk, rule, constraint)' -r
complete -c dkp -n "__fish_dkp_using_subcommand diff" -l threshold -d 'Content-drift percentage threshold for "modified" classification' -r
complete -c dkp -n "__fish_dkp_using_subcommand diff" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand diff" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand diff" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand diff" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand diff" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand diff" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand build" -l format -d 'Archive format: zip | tar.gz' -r
complete -c dkp -n "__fish_dkp_using_subcommand build" -l out -d 'Output directory (default: build/)' -r -F
complete -c dkp -n "__fish_dkp_using_subcommand build" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand build" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand build" -l no-human -d 'Exclude human/ assets (machine-only distribution)'
complete -c dkp -n "__fish_dkp_using_subcommand build" -l gen-mcp-manifest -d 'Regenerate machine/mcp_manifest.json before packaging'
complete -c dkp -n "__fish_dkp_using_subcommand build" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand build" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand build" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand build" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand release-check" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand release-check" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand release-check" -l strict -d 'Fail on warnings'
complete -c dkp -n "__fish_dkp_using_subcommand release-check" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand release-check" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand release-check" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand release-check" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and not __fish_seen_subcommand_from status check add-source report help" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand rights; and not __fish_seen_subcommand_from status check add-source report help" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand rights; and not __fish_seen_subcommand_from status check add-source report help" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and not __fish_seen_subcommand_from status check add-source report help" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and not __fish_seen_subcommand_from status check add-source report help" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and not __fish_seen_subcommand_from status check add-source report help" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and not __fish_seen_subcommand_from status check add-source report help" -a "status" -d 'Summary of sources and rights coverage'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and not __fish_seen_subcommand_from status check add-source report help" -a "check" -d 'Flag entries with missing fields or expired rights'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and not __fish_seen_subcommand_from status check add-source report help" -a "add-source" -d 'Interactive prompt to add a source entry'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and not __fish_seen_subcommand_from status check add-source report help" -a "report" -d 'Formatted compliance report for human review'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and not __fish_seen_subcommand_from status check add-source report help" -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from status" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from status" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from status" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from status" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from status" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from status" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from check" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from check" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from check" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from check" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from check" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from check" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from add-source" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from add-source" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from add-source" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from add-source" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from add-source" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from add-source" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from report" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from report" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from report" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from report" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from report" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from report" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from help" -f -a "status" -d 'Summary of sources and rights coverage'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from help" -f -a "check" -d 'Flag entries with missing fields or expired rights'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from help" -f -a "add-source" -d 'Interactive prompt to add a source entry'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from help" -f -a "report" -d 'Formatted compliance report for human review'
complete -c dkp -n "__fish_dkp_using_subcommand rights; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dkp -n "__fish_dkp_using_subcommand mcp-manifest" -l out -d 'Write to a custom path instead of machine/mcp_manifest.json' -r -F
complete -c dkp -n "__fish_dkp_using_subcommand mcp-manifest" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand mcp-manifest" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand mcp-manifest" -l dry-run -d 'Print what would be written without writing'
complete -c dkp -n "__fish_dkp_using_subcommand mcp-manifest" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand mcp-manifest" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand mcp-manifest" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand mcp-manifest" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand serve" -l transport -d 'Transport mechanism: stdio | http' -r
complete -c dkp -n "__fish_dkp_using_subcommand serve" -l port -d 'HTTP server port (when --transport http)' -r
complete -c dkp -n "__fish_dkp_using_subcommand serve" -l auth-token -d 'Static bearer token for authentication' -r
complete -c dkp -n "__fish_dkp_using_subcommand serve" -l log-level -d 'Server log verbosity: debug | info | warn' -r
complete -c dkp -n "__fish_dkp_using_subcommand serve" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand serve" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand serve" -l readonly -d 'Expose resources only; do not expose any tools'
complete -c dkp -n "__fish_dkp_using_subcommand serve" -l allow-unsigned -d 'Allow running non-WASM procedures from unsigned bundles (dev/testing only)'
complete -c dkp -n "__fish_dkp_using_subcommand serve" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand serve" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand serve" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand serve" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand tui" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand tui" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand tui" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand tui" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand tui" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand tui" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand webui" -l port -d 'Port to listen on (0 = OS-assigned random port)' -r
complete -c dkp -n "__fish_dkp_using_subcommand webui" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand webui" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand webui" -l no-open -d 'Do not automatically open the browser'
complete -c dkp -n "__fish_dkp_using_subcommand webui" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand webui" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand webui" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand webui" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand run" -l input -d 'JSON object to pass as stdin to the procedure (default: {})' -r
complete -c dkp -n "__fish_dkp_using_subcommand run" -l timeout-ms -d 'Override the wall-clock timeout in milliseconds' -r
complete -c dkp -n "__fish_dkp_using_subcommand run" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand run" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand run" -l allow-unsigned -d 'Allow running non-WASM procedures from unsigned bundles (dev/testing only)'
complete -c dkp -n "__fish_dkp_using_subcommand run" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand run" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand run" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand run" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and not __fish_seen_subcommand_from list validate new help" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and not __fish_seen_subcommand_from list validate new help" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and not __fish_seen_subcommand_from list validate new help" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and not __fish_seen_subcommand_from list validate new help" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and not __fish_seen_subcommand_from list validate new help" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and not __fish_seen_subcommand_from list validate new help" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and not __fish_seen_subcommand_from list validate new help" -f -a "list" -d 'List all procedures defined in machine/procedures/'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and not __fish_seen_subcommand_from list validate new help" -f -a "validate" -d 'Validate procedure file completeness and schema correctness'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and not __fish_seen_subcommand_from list validate new help" -f -a "new" -d 'Scaffold a new Rust WASI procedure project'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and not __fish_seen_subcommand_from list validate new help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from list" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from list" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from list" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from list" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from list" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from validate" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from validate" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from validate" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from validate" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from validate" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from validate" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from new" -l title -d 'Human-readable title for the procedure' -r
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from new" -l description -d 'One-sentence description of what the procedure does' -r
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from new" -l lang -d 'Procedure language/runtime: wasm | python | javascript' -r
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from new" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from new" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from new" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from new" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from new" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from new" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from help" -f -a "list" -d 'List all procedures defined in machine/procedures/'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from help" -f -a "validate" -d 'Validate procedure file completeness and schema correctness'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from help" -f -a "new" -d 'Scaffold a new Rust WASI procedure project'
complete -c dkp -n "__fish_dkp_using_subcommand procedures; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and not __fish_seen_subcommand_from stats validate list help" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand graph; and not __fish_seen_subcommand_from stats validate list help" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand graph; and not __fish_seen_subcommand_from stats validate list help" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and not __fish_seen_subcommand_from stats validate list help" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and not __fish_seen_subcommand_from stats validate list help" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and not __fish_seen_subcommand_from stats validate list help" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and not __fish_seen_subcommand_from stats validate list help" -a "stats" -d 'Print node and edge counts by type'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and not __fish_seen_subcommand_from stats validate list help" -a "validate" -d 'Check all edges resolve to known concept IDs (Gate 4)'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and not __fish_seen_subcommand_from stats validate list help" -a "list" -d 'List all nodes with their type and id'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and not __fish_seen_subcommand_from stats validate list help" -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from stats" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from stats" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from stats" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from stats" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from stats" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from stats" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from validate" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from validate" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from validate" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from validate" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from validate" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from validate" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from list" -l type -r
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from list" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from list" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from list" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from list" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from list" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from help" -f -a "stats" -d 'Print node and edge counts by type'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from help" -f -a "validate" -d 'Check all edges resolve to known concept IDs (Gate 4)'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from help" -f -a "list" -d 'List all nodes with their type and id'
complete -c dkp -n "__fish_dkp_using_subcommand graph; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and not __fish_seen_subcommand_from list validate help" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and not __fish_seen_subcommand_from list validate help" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and not __fish_seen_subcommand_from list validate help" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and not __fish_seen_subcommand_from list validate help" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and not __fish_seen_subcommand_from list validate help" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and not __fish_seen_subcommand_from list validate help" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and not __fish_seen_subcommand_from list validate help" -a "list" -d 'List all declared pack dependencies'
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and not __fish_seen_subcommand_from list validate help" -a "validate" -d 'Check local_id values resolve to concepts in this bundle (Gate 4)'
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and not __fish_seen_subcommand_from list validate help" -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and __fish_seen_subcommand_from list" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and __fish_seen_subcommand_from list" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and __fish_seen_subcommand_from list" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and __fish_seen_subcommand_from list" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and __fish_seen_subcommand_from list" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and __fish_seen_subcommand_from validate" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and __fish_seen_subcommand_from validate" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and __fish_seen_subcommand_from validate" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and __fish_seen_subcommand_from validate" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and __fish_seen_subcommand_from validate" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and __fish_seen_subcommand_from validate" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and __fish_seen_subcommand_from help" -f -a "list" -d 'List all declared pack dependencies'
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and __fish_seen_subcommand_from help" -f -a "validate" -d 'Check local_id values resolve to concepts in this bundle (Gate 4)'
complete -c dkp -n "__fish_dkp_using_subcommand cross-refs; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and not __fish_seen_subcommand_from list validate show help" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand skills; and not __fish_seen_subcommand_from list validate show help" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand skills; and not __fish_seen_subcommand_from list validate show help" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and not __fish_seen_subcommand_from list validate show help" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and not __fish_seen_subcommand_from list validate show help" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and not __fish_seen_subcommand_from list validate show help" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and not __fish_seen_subcommand_from list validate show help" -a "list" -d 'List all skills in skills/'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and not __fish_seen_subcommand_from list validate show help" -a "validate" -d 'Check SKILL.md format conformance'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and not __fish_seen_subcommand_from list validate show help" -a "show" -d 'Print a specific skill\'s SKILL.md'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and not __fish_seen_subcommand_from list validate show help" -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from list" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from list" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from list" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from list" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from list" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from validate" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from validate" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from validate" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from validate" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from validate" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from validate" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from show" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from show" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from show" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from show" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from show" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from show" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from help" -f -a "list" -d 'List all skills in skills/'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from help" -f -a "validate" -d 'Check SKILL.md format conformance'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from help" -f -a "show" -d 'Print a specific skill\'s SKILL.md'
complete -c dkp -n "__fish_dkp_using_subcommand skills; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and not __fish_seen_subcommand_from list validate export help" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and not __fish_seen_subcommand_from list validate export help" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and not __fish_seen_subcommand_from list validate export help" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and not __fish_seen_subcommand_from list validate export help" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and not __fish_seen_subcommand_from list validate export help" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and not __fish_seen_subcommand_from list validate export help" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and not __fish_seen_subcommand_from list validate export help" -a "list" -d 'List available locales'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and not __fish_seen_subcommand_from list validate export help" -a "validate" -d 'Check locale content doesn\'t contradict the base pack'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and not __fish_seen_subcommand_from list validate export help" -a "export" -d 'Export a locale-specific bundle'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and not __fish_seen_subcommand_from list validate export help" -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from list" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from list" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from list" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from list" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from list" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from validate" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from validate" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from validate" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from validate" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from validate" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from validate" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from export" -l out -r -F
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from export" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from export" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from export" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from export" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from export" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from export" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from help" -f -a "list" -d 'List available locales'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from help" -f -a "validate" -d 'Check locale content doesn\'t contradict the base pack'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from help" -f -a "export" -d 'Export a locale-specific bundle'
complete -c dkp -n "__fish_dkp_using_subcommand l10n; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dkp -n "__fish_dkp_using_subcommand new" -l domain -d 'Domain category (e.g. "Kubernetes", "Clinical Nutrition")' -r
complete -c dkp -n "__fish_dkp_using_subcommand new" -l dir -d 'Output directory (default: ./<name-slug>/)' -r -F
complete -c dkp -n "__fish_dkp_using_subcommand new" -l api-key -r
complete -c dkp -n "__fish_dkp_using_subcommand new" -l base-url -r
complete -c dkp -n "__fish_dkp_using_subcommand new" -l model -r
complete -c dkp -n "__fish_dkp_using_subcommand new" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand new" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand new" -l overwrite -d 'Overwrite already-generated assets'
complete -c dkp -n "__fish_dkp_using_subcommand new" -l skip-validate -d 'Skip validation step'
complete -c dkp -n "__fish_dkp_using_subcommand new" -l skip-package -d 'Skip packaging step'
complete -c dkp -n "__fish_dkp_using_subcommand new" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand new" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand new" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand new" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand generate" -l api-key -r
complete -c dkp -n "__fish_dkp_using_subcommand generate" -l base-url -r
complete -c dkp -n "__fish_dkp_using_subcommand generate" -l model -r
complete -c dkp -n "__fish_dkp_using_subcommand generate" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand generate" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand generate" -l overwrite -d 'Overwrite existing assets'
complete -c dkp -n "__fish_dkp_using_subcommand generate" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand generate" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand generate" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand generate" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand fix" -l api-key -r
complete -c dkp -n "__fish_dkp_using_subcommand fix" -l base-url -r
complete -c dkp -n "__fish_dkp_using_subcommand fix" -l model -r
complete -c dkp -n "__fish_dkp_using_subcommand fix" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand fix" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand fix" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand fix" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand fix" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand fix" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand review" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand review" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand review" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand review" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand review" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand review" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand keygen" -l out -d 'Write keys to a custom directory (default: ~/.dkp/)' -r -F
complete -c dkp -n "__fish_dkp_using_subcommand keygen" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand keygen" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand keygen" -l force -d 'Overwrite existing keys without prompting'
complete -c dkp -n "__fish_dkp_using_subcommand keygen" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand keygen" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand keygen" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand keygen" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand sign" -l key -d 'Path to Ed25519 private key (default: ~/.dkp/private.key)' -r -F
complete -c dkp -n "__fish_dkp_using_subcommand sign" -l out -d 'Write signature to a custom path (default: <archive-dir>/bundle.sig)' -r -F
complete -c dkp -n "__fish_dkp_using_subcommand sign" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand sign" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand sign" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand sign" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand sign" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand sign" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand install" -l out -d 'Install to a custom directory' -r -F
complete -c dkp -n "__fish_dkp_using_subcommand install" -l registry -d 'Override registry URL' -r
complete -c dkp -n "__fish_dkp_using_subcommand install" -l token -d 'Registry API token' -r
complete -c dkp -n "__fish_dkp_using_subcommand install" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand install" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand install" -s g -l global -d 'Install to global store (~/.dkp/packs/)'
complete -c dkp -n "__fish_dkp_using_subcommand install" -l no-verify -d 'Skip signature verification (NOT RECOMMENDED)'
complete -c dkp -n "__fish_dkp_using_subcommand install" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand install" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand install" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand install" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand uninstall" -l out -d 'Remove from a custom directory' -r -F
complete -c dkp -n "__fish_dkp_using_subcommand uninstall" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand uninstall" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand uninstall" -s g -l global -d 'Remove from global store'
complete -c dkp -n "__fish_dkp_using_subcommand uninstall" -l all-versions -d 'Remove all installed versions'
complete -c dkp -n "__fish_dkp_using_subcommand uninstall" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand uninstall" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand uninstall" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand uninstall" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand update" -l registry -d 'Override registry URL' -r
complete -c dkp -n "__fish_dkp_using_subcommand update" -l token -d 'Registry API token' -r
complete -c dkp -n "__fish_dkp_using_subcommand update" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand update" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand update" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand update" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand update" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand update" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand publish" -l url -d 'HTTPS URL to the hosted archive (publisher-controlled storage)' -r
complete -c dkp -n "__fish_dkp_using_subcommand publish" -l build-dir -d 'Directory containing checksums.json and bundle.sig (default: <pack>/build/)' -r -F
complete -c dkp -n "__fish_dkp_using_subcommand publish" -l registry -d 'Override registry URL' -r
complete -c dkp -n "__fish_dkp_using_subcommand publish" -l token -d 'Registry API token' -r
complete -c dkp -n "__fish_dkp_using_subcommand publish" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand publish" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand publish" -l private -d 'Set pack visibility to private'
complete -c dkp -n "__fish_dkp_using_subcommand publish" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand publish" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand publish" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand publish" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand yank" -l reason -d 'Reason shown to consumers who attempt to install this version' -r
complete -c dkp -n "__fish_dkp_using_subcommand yank" -l registry -d 'Override registry URL' -r
complete -c dkp -n "__fish_dkp_using_subcommand yank" -l token -d 'Registry API token' -r
complete -c dkp -n "__fish_dkp_using_subcommand yank" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand yank" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand yank" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand yank" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand yank" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand yank" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and not __fish_seen_subcommand_from register login logout token keys pack help" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand registry; and not __fish_seen_subcommand_from register login logout token keys pack help" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand registry; and not __fish_seen_subcommand_from register login logout token keys pack help" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and not __fish_seen_subcommand_from register login logout token keys pack help" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and not __fish_seen_subcommand_from register login logout token keys pack help" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and not __fish_seen_subcommand_from register login logout token keys pack help" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and not __fish_seen_subcommand_from register login logout token keys pack help" -f -a "register" -d 'Create a new publisher account and save the API key to ~/.dkp/credentials'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and not __fish_seen_subcommand_from register login logout token keys pack help" -f -a "login" -d 'Authenticate with an existing account and save API key to ~/.dkp/credentials'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and not __fish_seen_subcommand_from register login logout token keys pack help" -f -a "logout" -d 'Remove saved credentials'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and not __fish_seen_subcommand_from register login logout token keys pack help" -f -a "token" -d 'Rotate your API key'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and not __fish_seen_subcommand_from register login logout token keys pack help" -f -a "keys" -d 'Manage Ed25519 public keys registered with the registry'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and not __fish_seen_subcommand_from register login logout token keys pack help" -f -a "pack" -d 'Pack-level management subcommands'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and not __fish_seen_subcommand_from register login logout token keys pack help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from register" -l email -r
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from register" -l registry -r
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from register" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from register" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from register" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from register" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from register" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from register" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from login" -l email -r
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from login" -l registry -r
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from login" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from login" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from login" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from login" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from login" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from login" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from logout" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from logout" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from logout" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from logout" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from logout" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from logout" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from token" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from token" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from token" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from token" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from token" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from token" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from token" -f -a "rotate"
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from token" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from keys" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from keys" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from keys" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from keys" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from keys" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from keys" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from keys" -f -a "add"
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from keys" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from pack" -l output -d 'Output format' -r -f -a "plain\t'Human-readable plain text (default)'
table\t'Aligned table using comfy-table'
json\t'Pretty-printed JSON'"
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from pack" -l audience -d 'Filter content to assets tagged for a specific audience profile' -r
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from pack" -s q -l quiet -d 'Suppress informational output; print only results'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from pack" -s v -l verbose -d 'Print debug info (schema paths, provider calls, etc.)'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from pack" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from pack" -s V -l version -d 'Print version'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from pack" -f -a "versions" -d 'List all published versions of a pack'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from pack" -f -a "set-visibility" -d 'Set pack visibility (public or private)'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from pack" -f -a "grant" -d 'Grant access to a private pack'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from pack" -f -a "revoke" -d 'Revoke access to a private pack'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from pack" -f -a "access" -d 'List accounts with access to a private pack'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from pack" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from help" -f -a "register" -d 'Create a new publisher account and save the API key to ~/.dkp/credentials'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from help" -f -a "login" -d 'Authenticate with an existing account and save API key to ~/.dkp/credentials'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from help" -f -a "logout" -d 'Remove saved credentials'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from help" -f -a "token" -d 'Rotate your API key'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from help" -f -a "keys" -d 'Manage Ed25519 public keys registered with the registry'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from help" -f -a "pack" -d 'Pack-level management subcommands'
complete -c dkp -n "__fish_dkp_using_subcommand registry; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "init" -d 'Scaffold a new DKP pack directory with all required files'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "info" -d 'Print a summary of a pack (name, version, asset counts, compliance)'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "list" -d 'List all packs under a root directory'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "validate" -d 'Run schema and compliance checks; exit non-zero on failure'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "search" -d 'Full-text BM25 search across machine assets (or registry with --registry)'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "get" -d 'Retrieve a specific asset or all assets of a type'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "inject" -d 'Print a ready-to-inject LLM context block'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "export" -d 'Convert machine assets to another format (okf, langchain, llamaindex, …)'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "okf" -d 'OKF-specific operations (export, validate, stats, links, browse)'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "chunk" -d 'Retrieve the top-N most relevant retrieval chunks for a query'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "eval" -d 'Run eval set against baseline and grounded prompts; print delta report'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "prompt" -d 'Interactive grounded prompt REPL for testing a pack'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "diff" -d 'Compare two pack versions and report what changed'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "build" -d 'Package a pack into a versioned archive with checksums.json'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "release-check" -d 'Pre-release compliance checklist (runs all gates, checks human fields)'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "rights" -d 'Source and rights log operations'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "mcp-manifest" -d 'Generate or regenerate machine/mcp_manifest.json'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "serve" -d 'Start the pack as an MCP server (requires --features mcp)'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "tui" -d 'Interactive TUI browser (requires --features tui)'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "webui" -d 'Browse a pack in a local web UI (requires --features webui)'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "run" -d 'Invoke a WASM/WASI procedure from machine/procedures/'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "procedures" -d 'List, validate, and scaffold executable procedures'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "graph" -d 'Inspect and validate knowledge_graph.json'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "cross-refs" -d 'Inspect and validate cross_refs.json'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "skills" -d 'Manage and validate the skills/ layer'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "l10n" -d 'Manage and validate the l10n/ localization layer'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "new" -d 'Scaffold + LLM-generate a complete pack in one command'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "generate" -d 'Run (or re-run) LLM generation on an existing pack'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "fix" -d 'Failure-aware chunk regeneration using eval results'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "review" -d 'Generate evidence drafts for manual review gates'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "keygen" -d 'Generate an Ed25519 keypair for signing packs'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "sign" -d 'Sign a built archive with an Ed25519 private key'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "install" -d 'Install a pack from the registry'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "uninstall" -d 'Remove an installed pack'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "update" -d 'Re-resolve and update installed packs to satisfy lock-file constraints'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "publish" -d 'Publish a built and signed pack to the registry'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "yank" -d 'Mark a published version as yanked'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "registry" -d 'Registry account and pack management (login, logout, keys, access)'
complete -c dkp -n "__fish_dkp_using_subcommand help; and not __fish_seen_subcommand_from init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from okf" -f -a "export" -d 'Generate okf/ from machine/'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from okf" -f -a "validate" -d 'Check OKF conformance (Gate 8)'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from okf" -f -a "stats" -d 'Print concept count by type'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from okf" -f -a "links" -d 'Check cross-link integrity'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from okf" -f -a "browse" -d 'Interactive terminal browser'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from rights" -f -a "status" -d 'Summary of sources and rights coverage'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from rights" -f -a "check" -d 'Flag entries with missing fields or expired rights'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from rights" -f -a "add-source" -d 'Interactive prompt to add a source entry'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from rights" -f -a "report" -d 'Formatted compliance report for human review'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from procedures" -f -a "list" -d 'List all procedures defined in machine/procedures/'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from procedures" -f -a "validate" -d 'Validate procedure file completeness and schema correctness'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from procedures" -f -a "new" -d 'Scaffold a new Rust WASI procedure project'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from graph" -f -a "stats" -d 'Print node and edge counts by type'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from graph" -f -a "validate" -d 'Check all edges resolve to known concept IDs (Gate 4)'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from graph" -f -a "list" -d 'List all nodes with their type and id'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from cross-refs" -f -a "list" -d 'List all declared pack dependencies'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from cross-refs" -f -a "validate" -d 'Check local_id values resolve to concepts in this bundle (Gate 4)'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from skills" -f -a "list" -d 'List all skills in skills/'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from skills" -f -a "validate" -d 'Check SKILL.md format conformance'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from skills" -f -a "show" -d 'Print a specific skill\'s SKILL.md'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from l10n" -f -a "list" -d 'List available locales'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from l10n" -f -a "validate" -d 'Check locale content doesn\'t contradict the base pack'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from l10n" -f -a "export" -d 'Export a locale-specific bundle'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from registry" -f -a "register" -d 'Create a new publisher account and save the API key to ~/.dkp/credentials'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from registry" -f -a "login" -d 'Authenticate with an existing account and save API key to ~/.dkp/credentials'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from registry" -f -a "logout" -d 'Remove saved credentials'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from registry" -f -a "token" -d 'Rotate your API key'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from registry" -f -a "keys" -d 'Manage Ed25519 public keys registered with the registry'
complete -c dkp -n "__fish_dkp_using_subcommand help; and __fish_seen_subcommand_from registry" -f -a "pack" -d 'Pack-level management subcommands'

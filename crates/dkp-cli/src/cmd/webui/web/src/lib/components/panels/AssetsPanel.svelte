<script lang="ts">
  import type { PackSummary } from '../../api.ts'

  type Panel = 'assets' | 'glossary' | 'rules' | 'chunks' | 'eval' | 'mcp' | 'graph'

  let { pack, onnavigate }: { pack: PackSummary | null; onnavigate: (p: Panel) => void } = $props()

  const cards: { label: string; panel?: Panel; getValue: (p: PackSummary) => string | number; accent: string }[] = [
    { label: 'Glossary terms', panel: 'glossary', getValue: (p) => p.counts.glossary, accent: 'text-accent-blue' },
    { label: 'Rules', panel: 'rules', getValue: (p) => p.counts.rules, accent: 'text-accent-green' },
    { label: 'Retrieval chunks', panel: 'chunks', getValue: (p) => p.counts.chunks, accent: 'text-accent-purple' },
    { label: 'Eval cases', panel: 'eval', getValue: (p) => p.counts.eval_cases, accent: 'text-accent-yellow' },
    { label: 'Knowledge graph', panel: 'graph', getValue: (p) => p.counts.has_graph ? '✓' : '—', accent: 'text-accent-cyan' },
    { label: 'MCP manifest', panel: 'mcp', getValue: (p) => p.counts.has_mcp_manifest ? '✓' : '—', accent: 'text-accent-cyan' },
    { label: 'Ontology', getValue: (p) => p.counts.has_ontology ? '✓' : '—', accent: 'text-gray-400' },
    { label: 'Constraints', getValue: (p) => p.counts.has_constraints ? '✓' : '—', accent: 'text-gray-400' },
    { label: 'System prompt', getValue: (p) => p.counts.has_system_prompt ? '✓' : '—', accent: 'text-gray-400' },
    { label: 'Cross-refs', getValue: (p) => p.counts.has_cross_refs ? '✓' : '—', accent: 'text-gray-400' },
  ]
</script>

{#if pack}
  <div>
    <h1 class="text-xl font-semibold text-white mb-1">{pack.name}</h1>
    <p class="text-sm text-gray-500 mb-6">
      {pack.domain} · v{pack.version} · updated {pack.update_date}
    </p>
    {#if pack.description}
      <p class="text-sm text-gray-400 mb-8 max-w-2xl">{pack.description}</p>
    {/if}

    <h2 class="text-xs font-semibold uppercase tracking-widest text-gray-500 mb-4">Assets</h2>
    <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-3">
      {#each cards as card}
        <button
          class="bg-surface-1 border border-surface-3 rounded-lg p-4 text-left transition-all
            {card.panel ? 'hover:border-accent-blue/50 hover:bg-surface-2 cursor-pointer' : 'cursor-default'}"
          onclick={() => card.panel && onnavigate(card.panel)}
        >
          <p class="text-xs text-gray-500 mb-1">{card.label}</p>
          <p class="text-2xl font-semibold {card.accent}">{pack ? card.getValue(pack) : '—'}</p>
        </button>
      {/each}
    </div>
  </div>
{:else}
  <div class="grid grid-cols-4 gap-3">
    {#each Array(8) as _}
      <div class="bg-surface-1 border border-surface-3 rounded-lg p-4 h-20 animate-pulse"></div>
    {/each}
  </div>
{/if}

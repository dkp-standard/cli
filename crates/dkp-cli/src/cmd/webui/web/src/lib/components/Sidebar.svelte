<script lang="ts">
  import type { PackSummary } from '../api.ts'

  type Panel = 'assets' | 'glossary' | 'rules' | 'chunks' | 'eval' | 'mcp' | 'graph'

  let { pack, activePanel, onselect }: {
    pack: PackSummary | null
    activePanel: Panel
    onselect: (p: Panel) => void
  } = $props()

  const panels: { id: Panel; label: string; key: string; show: (p: PackSummary) => boolean }[] = [
    { id: 'assets',   label: 'Overview',  key: '1', show: () => true },
    { id: 'glossary', label: 'Glossary',  key: '2', show: (p) => p.counts.glossary > 0 },
    { id: 'rules',    label: 'Rules',     key: '3', show: (p) => p.counts.rules > 0 },
    { id: 'chunks',   label: 'Chunks',    key: '4', show: (p) => p.counts.chunks > 0 },
    { id: 'eval',     label: 'Eval',      key: '5', show: (p) => p.counts.eval_cases > 0 },
    { id: 'mcp',      label: 'MCP',       key: '6', show: (p) => p.counts.has_mcp_manifest },
    { id: 'graph',    label: 'Graph',     key: '7', show: (p) => p.counts.has_graph },
  ]
</script>

<aside class="w-52 flex-shrink-0 bg-surface-1 border-r border-surface-3 flex flex-col overflow-hidden">
  <!-- Pack identity -->
  <div class="p-4 border-b border-surface-3">
    {#if pack}
      <p class="text-xs text-accent-blue font-semibold uppercase tracking-widest mb-1">DKP</p>
      <p class="text-sm font-semibold text-white leading-tight truncate">{pack.name}</p>
      <p class="text-xs text-gray-500 mt-0.5">{pack.version} · {pack.domain}</p>
    {:else}
      <div class="h-10 bg-surface-2 rounded animate-pulse"></div>
    {/if}
  </div>

  <!-- Nav -->
  <nav class="flex-1 overflow-y-auto py-2">
    {#each panels as panel (panel.id)}
      {#if !pack || panel.show(pack)}
        <button
          class="w-full flex items-center gap-3 px-4 py-2 text-sm text-left transition-colors
            {activePanel === panel.id
              ? 'bg-surface-2 text-white border-r-2 border-accent-blue'
              : 'text-gray-400 hover:text-white hover:bg-surface-2/50'}"
          onclick={() => onselect(panel.id)}
        >
          <span class="text-xs text-gray-600 w-3">{panel.key}</span>
          {panel.label}
        </button>
      {/if}
    {/each}
  </nav>

  <div class="p-3 border-t border-surface-3">
    <p class="text-xs text-gray-600">Press 1–7 to switch panels</p>
  </div>
</aside>

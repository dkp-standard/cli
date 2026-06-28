<script lang="ts">
  import { onMount } from 'svelte'
  import { api, type PackSummary } from './lib/api.ts'
  import Sidebar from './lib/components/Sidebar.svelte'
  import SearchBar from './lib/components/SearchBar.svelte'
  import AssetsPanel from './lib/components/panels/AssetsPanel.svelte'
  import GlossaryPanel from './lib/components/panels/GlossaryPanel.svelte'
  import RulesPanel from './lib/components/panels/RulesPanel.svelte'
  import ChunksPanel from './lib/components/panels/ChunksPanel.svelte'
  import EvalPanel from './lib/components/panels/EvalPanel.svelte'
  import McpPanel from './lib/components/panels/McpPanel.svelte'
  import GraphPanel from './lib/components/panels/GraphPanel.svelte'

  type Panel = 'assets' | 'glossary' | 'rules' | 'chunks' | 'eval' | 'mcp' | 'graph'

  let pack = $state<PackSummary | null>(null)
  let activePanel = $state<Panel>('assets')
  let error = $state<string | null>(null)

  onMount(async () => {
    try {
      pack = await api.pack()
    } catch (e) {
      error = String(e)
    }
  })

  function handleKeydown(e: KeyboardEvent) {
    if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return
    const panels: Panel[] = ['assets', 'glossary', 'rules', 'chunks', 'eval', 'mcp', 'graph']
    const num = parseInt(e.key)
    if (num >= 1 && num <= panels.length) {
      activePanel = panels[num - 1]
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if error}
  <div class="flex items-center justify-center h-full text-accent-red font-mono p-8">
    <div>
      <p class="text-lg font-semibold mb-2">Failed to load pack</p>
      <p class="text-sm opacity-70">{error}</p>
    </div>
  </div>
{:else}
  <div class="flex h-full overflow-hidden bg-surface-0">
    <Sidebar {pack} {activePanel} onselect={(p) => (activePanel = p)} />

    <div class="flex-1 flex flex-col overflow-hidden">
      <SearchBar {pack} onselect={(p) => (activePanel = p)} />

      <main class="flex-1 overflow-auto p-6">
        {#if activePanel === 'assets'}
          <AssetsPanel {pack} onnavigate={(p) => (activePanel = p)} />
        {:else if activePanel === 'glossary'}
          <GlossaryPanel />
        {:else if activePanel === 'rules'}
          <RulesPanel />
        {:else if activePanel === 'chunks'}
          <ChunksPanel />
        {:else if activePanel === 'eval'}
          <EvalPanel />
        {:else if activePanel === 'mcp'}
          <McpPanel />
        {:else if activePanel === 'graph'}
          <GraphPanel />
        {/if}
      </main>
    </div>
  </div>
{/if}

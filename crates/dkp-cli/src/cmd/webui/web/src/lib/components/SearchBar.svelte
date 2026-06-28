<script lang="ts">
  import { api, type PackSummary, type SearchResult } from '../api.ts'

  type Panel = 'assets' | 'glossary' | 'rules' | 'chunks' | 'eval' | 'mcp' | 'graph'

  const ASSET_TYPE_TO_PANEL: Record<string, Panel> = {
    chunk: 'chunks',
    chunks: 'chunks',
    glossary: 'glossary',
    rule: 'rules',
    rules: 'rules',
    eval: 'eval',
    graph: 'graph',
    mcp: 'mcp',
  }

  let { pack, onselect }: { pack: PackSummary | null; onselect?: (panel: Panel) => void } = $props()

  let query = $state('')
  let results = $state<SearchResult[]>([])
  let loading = $state(false)
  let open = $state(false)
  let inputEl: HTMLInputElement

  let debounce: ReturnType<typeof setTimeout>

  $effect(() => {
    const q = query.trim()
    clearTimeout(debounce)
    if (!q) { results = []; open = false; return }
    debounce = setTimeout(async () => {
      loading = true
      try {
        results = await api.search(q)
        open = results.length > 0
      } catch { results = [] }
      loading = false
    }, 200)
  })

  function handleKeydown(e: KeyboardEvent) {
    if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
      e.preventDefault()
      inputEl?.focus()
    }
    if (e.key === 'Escape') { query = ''; open = false; inputEl?.blur() }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<header class="bg-surface-1 border-b border-surface-3 px-6 py-3 flex items-center gap-4">
  {#if pack}
    <span class="text-xs text-gray-500 hidden md:block">
      {pack.name} {pack.version}
    </span>
    <span class="text-gray-700">·</span>
  {/if}

  <div class="relative flex-1 max-w-xl">
    <div class="flex items-center gap-2 bg-surface-2 border border-surface-3 rounded-lg px-3 py-1.5
      focus-within:border-accent-blue transition-colors">
      <svg class="w-4 h-4 text-gray-500 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
          d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
      </svg>
      <input
        bind:this={inputEl}
        bind:value={query}
        type="text"
        placeholder="Search pack… (⌘K)"
        class="flex-1 bg-transparent text-sm text-white placeholder-gray-500 outline-none"
      />
      {#if loading}
        <span class="text-xs text-gray-500 animate-pulse">…</span>
      {/if}
      {#if !loading && query}
        <button class="text-gray-500 hover:text-white" onclick={() => { query = ''; open = false }}>✕</button>
      {/if}
    </div>

    {#if open}
      <div class="absolute top-full mt-1 left-0 right-0 bg-surface-1 border border-surface-3 rounded-lg
        shadow-xl z-50 max-h-80 overflow-y-auto">
        {#each results as r (r.id)}
          <button
            class="w-full text-left px-4 py-3 hover:bg-surface-2 cursor-pointer border-b border-surface-3 last:border-0"
            onclick={() => {
              const panel = ASSET_TYPE_TO_PANEL[r.asset_type.toLowerCase()]
              if (panel) onselect?.(panel)
              query = ''
              open = false
            }}
          >
            <div class="flex items-center gap-2 mb-0.5">
              <span class="text-xs px-1.5 py-0.5 rounded bg-surface-3 text-accent-cyan">{r.asset_type}</span>
              <span class="text-sm font-medium text-white">{r.title}</span>
              <span class="ml-auto text-xs text-gray-600">{r.score.toFixed(2)}</span>
            </div>
            <p class="text-xs text-gray-500 line-clamp-2">{r.excerpt}</p>
          </button>
        {/each}
      </div>
    {/if}
  </div>
</header>

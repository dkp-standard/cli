<script lang="ts">
  import { onMount } from 'svelte'
  import { api, type RetrievalChunk } from '../../api.ts'

  let chunks = $state<RetrievalChunk[]>([])
  let filter = $state('')
  let expanded = $state<string | null>(null)
  let loading = $state(true)

  onMount(async () => {
    try { chunks = await api.chunks() } catch {}
    loading = false
  })

  let filtered = $derived(
    filter.trim()
      ? chunks.filter(c =>
          c.title.toLowerCase().includes(filter.toLowerCase()) ||
          c.chunk_text.toLowerCase().includes(filter.toLowerCase()) ||
          c.tags?.some(t => t.toLowerCase().includes(filter.toLowerCase()))
        )
      : chunks
  )

  function confidenceColor(conf?: number) {
    if (conf == null) return 'bg-gray-600/20 text-gray-400'
    if (conf >= 0.8) return 'bg-accent-green/10 text-accent-green'
    if (conf >= 0.5) return 'bg-accent-yellow/10 text-accent-yellow'
    return 'bg-accent-red/10 text-accent-red'
  }
</script>

<div>
  <div class="flex items-center justify-between mb-6">
    <h2 class="text-lg font-semibold text-white">Chunks <span class="text-gray-500 text-sm ml-1">({chunks.length})</span></h2>
    <input
      bind:value={filter}
      placeholder="Filter chunks…"
      class="bg-surface-2 border border-surface-3 rounded-lg px-3 py-1.5 text-sm text-white placeholder-gray-500 outline-none focus:border-accent-blue w-48 transition-colors"
    />
  </div>

  {#if loading}
    <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
      {#each Array(6) as _}
        <div class="h-32 bg-surface-1 rounded-lg animate-pulse"></div>
      {/each}
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
      {#each filtered as chunk (chunk.id)}
        <div
          class="bg-surface-1 border border-surface-3 rounded-lg overflow-hidden cursor-pointer
            hover:border-surface-3 transition-all {expanded === chunk.id ? 'md:col-span-2 border-accent-purple/50' : ''}"
          onclick={() => expanded = expanded === chunk.id ? null : chunk.id}
          role="button"
          tabindex="0"
          onkeydown={(e) => e.key === 'Enter' && (expanded = expanded === chunk.id ? null : chunk.id)}
        >
          <div class="p-4">
            <div class="flex items-start gap-2 mb-2 flex-wrap">
              <span class="font-medium text-white text-sm flex-1">{chunk.title}</span>
              {#if chunk.confidence != null}
                <span class="text-xs px-1.5 py-0.5 rounded {confidenceColor(chunk.confidence)} flex-shrink-0">
                  {(chunk.confidence * 100).toFixed(0)}%
                </span>
              {/if}
              {#if chunk.retrieval_priority}
                <span class="text-xs px-1.5 py-0.5 rounded bg-surface-3 text-gray-400">{chunk.retrieval_priority}</span>
              {/if}
            </div>

            {#if chunk.summary}
              <p class="text-xs text-gray-500 mb-2">{chunk.summary}</p>
            {/if}

            <p class="text-xs text-gray-400 {expanded === chunk.id ? '' : 'line-clamp-3'} font-mono leading-relaxed">
              {chunk.chunk_text}
            </p>

            {#if (chunk.tags ?? []).length > 0}
              <div class="flex flex-wrap gap-1 mt-3">
                {#each (chunk.tags ?? []).slice(0, 5) as tag}
                  <span class="text-xs px-1.5 py-0.5 rounded bg-accent-purple/10 text-accent-purple">{tag}</span>
                {/each}
              </div>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

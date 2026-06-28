<script lang="ts">
  import { onMount } from 'svelte'
  import { api, type GlossaryTerm } from '../../api.ts'

  let terms = $state<GlossaryTerm[]>([])
  let filter = $state('')
  let expanded = $state<string | null>(null)
  let loading = $state(true)

  onMount(async () => {
    try { terms = await api.glossary() } catch {}
    loading = false
  })

  let filtered = $derived(
    filter.trim()
      ? terms.filter(t =>
          t.term.toLowerCase().includes(filter.toLowerCase()) ||
          t.definition.toLowerCase().includes(filter.toLowerCase()) ||
          t.tags?.some(tag => tag.toLowerCase().includes(filter.toLowerCase()))
        )
      : terms
  )
</script>

<div>
  <div class="flex items-center justify-between mb-6">
    <h2 class="text-lg font-semibold text-white">Glossary <span class="text-gray-500 text-sm ml-1">({terms.length})</span></h2>
    <input
      bind:value={filter}
      placeholder="Filter terms…"
      class="bg-surface-2 border border-surface-3 rounded-lg px-3 py-1.5 text-sm text-white placeholder-gray-500 outline-none focus:border-accent-blue w-48 transition-colors"
    />
  </div>

  {#if loading}
    <div class="space-y-2">
      {#each Array(6) as _}
        <div class="h-16 bg-surface-1 rounded-lg animate-pulse"></div>
      {/each}
    </div>
  {:else}
    <div class="space-y-2">
      {#each filtered as term (term.id)}
        <div
          class="bg-surface-1 border border-surface-3 rounded-lg overflow-hidden transition-all"
          class:border-accent-blue={expanded === term.id}
        >
          <button
            class="w-full flex items-start gap-4 p-4 text-left hover:bg-surface-2 transition-colors"
            onclick={() => expanded = expanded === term.id ? null : term.id}
          >
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2 mb-1 flex-wrap">
                <span class="font-medium text-white">{term.term}</span>
                {#if term.stability}
                  <span class="text-xs px-1.5 py-0.5 rounded bg-surface-3 text-gray-400">{term.stability}</span>
                {/if}
                {#each (term.tags ?? []).slice(0, 3) as tag}
                  <span class="text-xs px-1.5 py-0.5 rounded bg-accent-blue/10 text-accent-blue">{tag}</span>
                {/each}
              </div>
              <p class="text-sm text-gray-400 line-clamp-2">{term.definition}</p>
            </div>
            <span class="text-gray-600 text-xs flex-shrink-0 mt-1">{expanded === term.id ? '▲' : '▼'}</span>
          </button>

          {#if expanded === term.id}
            <div class="px-4 pb-4 border-t border-surface-3 pt-3 space-y-2">
              {#if term.aliases?.length}
                <p class="text-xs text-gray-500">Aliases: <span class="text-gray-300">{term.aliases.join(', ')}</span></p>
              {/if}
              {#if term.related?.length}
                <p class="text-xs text-gray-500">Related: <span class="text-gray-300">{term.related.join(', ')}</span></p>
              {/if}
              {#if term.audience?.length}
                <p class="text-xs text-gray-500">Audience: <span class="text-gray-300">{term.audience.join(', ')}</span></p>
              {/if}
              <p class="text-xs text-gray-600 font-mono">{term.id}</p>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>

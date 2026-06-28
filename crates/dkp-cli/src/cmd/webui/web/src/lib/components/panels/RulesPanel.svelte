<script lang="ts">
  import { onMount } from 'svelte'
  import { api, type Rule } from '../../api.ts'

  let rules = $state<Rule[]>([])
  let filter = $state('')
  let loading = $state(true)
  let expanded = $state<string | null>(null)

  onMount(async () => {
    try { rules = await api.rules() } catch {}
    loading = false
  })

  let filtered = $derived(
    filter.trim()
      ? rules.filter(r =>
          r.title.toLowerCase().includes(filter.toLowerCase()) ||
          r.description.toLowerCase().includes(filter.toLowerCase())
        )
      : rules
  )
</script>

<div>
  <div class="flex items-center justify-between mb-6">
    <h2 class="text-lg font-semibold text-white">Rules <span class="text-gray-500 text-sm ml-1">({rules.length})</span></h2>
    <input
      bind:value={filter}
      placeholder="Filter rules…"
      class="bg-surface-2 border border-surface-3 rounded-lg px-3 py-1.5 text-sm text-white placeholder-gray-500 outline-none focus:border-accent-blue w-48 transition-colors"
    />
  </div>

  {#if loading}
    <div class="space-y-2">
      {#each Array(5) as _}
        <div class="h-16 bg-surface-1 rounded-lg animate-pulse"></div>
      {/each}
    </div>
  {:else}
    <div class="space-y-2">
      {#each filtered as rule (rule.id)}
        <div
          class="bg-surface-1 border border-surface-3 rounded-lg overflow-hidden"
          class:border-accent-green={expanded === rule.id && rule.polarity === 'Affirmative'}
          class:border-accent-red={expanded === rule.id && rule.polarity === 'Prohibitive'}
        >
          <button
            class="w-full flex items-start gap-4 p-4 text-left hover:bg-surface-2 transition-colors"
            onclick={() => expanded = expanded === rule.id ? null : rule.id}
          >
            <span
              class="flex-shrink-0 text-xs px-2 py-0.5 rounded-full font-semibold mt-0.5
                {rule.polarity === 'Affirmative' ? 'bg-accent-green/10 text-accent-green' : 'bg-accent-red/10 text-accent-red'}"
            >
              {rule.polarity === 'Affirmative' ? 'DO' : 'DON\'T'}
            </span>
            <div class="flex-1 min-w-0">
              <p class="font-medium text-white mb-0.5">{rule.title}</p>
              <p class="text-sm text-gray-400 line-clamp-2">{rule.description}</p>
            </div>
            {#if rule.confidence != null}
              <span class="text-xs text-gray-600 flex-shrink-0 mt-1">{(rule.confidence * 100).toFixed(0)}%</span>
            {/if}
          </button>

          {#if expanded === rule.id}
            <div class="px-4 pb-4 border-t border-surface-3 pt-3 space-y-2">
              {#each (rule.tags ?? []) as tag}
                <span class="inline-block text-xs px-1.5 py-0.5 rounded bg-surface-3 text-gray-400 mr-1">{tag}</span>
              {/each}
              {#if rule.audience?.length}
                <p class="text-xs text-gray-500 mt-2">Audience: <span class="text-gray-300">{rule.audience.join(', ')}</span></p>
              {/if}
              <p class="text-xs text-gray-600 font-mono mt-1">{rule.id}</p>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>

<script lang="ts">
  import { onMount } from 'svelte'
  import { api, type EvalCase } from '../../api.ts'

  let cases = $state<EvalCase[]>([])
  let loading = $state(true)
  let expanded = $state<number | null>(null)

  onMount(async () => {
    try { cases = await api.eval() } catch {}
    loading = false
  })
</script>

<div>
  <h2 class="text-lg font-semibold text-white mb-6">
    Eval Cases <span class="text-gray-500 text-sm ml-1">({cases.length})</span>
  </h2>

  {#if loading}
    <div class="space-y-2">
      {#each Array(4) as _}
        <div class="h-20 bg-surface-1 rounded-lg animate-pulse"></div>
      {/each}
    </div>
  {:else}
    <div class="space-y-2">
      {#each cases as c, i (i)}
        <div
          class="bg-surface-1 border border-surface-3 rounded-lg overflow-hidden"
          class:border-accent-yellow={expanded === i}
        >
          <button
            class="w-full flex items-start gap-4 p-4 text-left hover:bg-surface-2 transition-colors"
            onclick={() => expanded = expanded === i ? null : i}
          >
            <span class="text-xs text-gray-600 flex-shrink-0 mt-1 w-6">#{i + 1}</span>
            <div class="flex-1 min-w-0">
              <p class="font-medium text-white text-sm mb-1">{c.query}</p>
              <div class="flex flex-wrap gap-1">
                {#each (c.tags ?? []).slice(0, 4) as tag}
                  <span class="text-xs px-1.5 py-0.5 rounded bg-accent-yellow/10 text-accent-yellow">{tag}</span>
                {/each}
              </div>
            </div>
            <span class="text-gray-600 text-xs flex-shrink-0">{expanded === i ? '▲' : '▼'}</span>
          </button>

          {#if expanded === i}
            <div class="px-4 pb-4 border-t border-surface-3 pt-3 space-y-4">
              {#if c.expected_dimensions?.length}
                <div>
                  <p class="text-xs font-semibold text-gray-500 uppercase mb-1">Expected dimensions</p>
                  <ul class="space-y-1">
                    {#each c.expected_dimensions as d}
                      <li class="text-sm text-gray-300 flex gap-2"><span class="text-accent-green">✓</span>{d}</li>
                    {/each}
                  </ul>
                </div>
              {/if}
              {#if c.critical_must_include?.length}
                <div>
                  <p class="text-xs font-semibold text-gray-500 uppercase mb-1">Must include</p>
                  <ul class="space-y-1">
                    {#each c.critical_must_include as m}
                      <li class="text-sm text-gray-300 flex gap-2"><span class="text-accent-red">!</span>{m}</li>
                    {/each}
                  </ul>
                </div>
              {/if}
              {#if c.scoring_rubric}
                <div>
                  <p class="text-xs font-semibold text-gray-500 uppercase mb-1">Scoring rubric</p>
                  <p class="text-sm text-gray-400">{c.scoring_rubric}</p>
                </div>
              {/if}
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>

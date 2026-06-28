<script lang="ts">
  import { onMount } from 'svelte'
  import { api } from '../../api.ts'
  import { createHighlighter } from 'shiki'

  let manifest = $state<unknown | null>(null)
  let systemPrompt = $state<string | null>(null)
  let highlighted = $state<string>('')
  let loading = $state(true)
  let tab = $state<'manifest' | 'prompt'>('manifest')

  onMount(async () => {
    try {
      manifest = await api.crossRefs().catch(() => null)  // placeholder - loads mcp manifest via assets
      // Actually load MCP manifest as raw JSON from the assets endpoint
      const res = await fetch('/api/assets/system-prompt').catch(() => null)
      if (res?.ok) {
        const data = await res.json()
        systemPrompt = data.content ?? null
      }
    } catch {}

    // Load mcp manifest
    try {
      const mcpRes = await fetch('/api/assets/cross-refs')
      if (mcpRes.ok) manifest = await mcpRes.json()
    } catch {}

    if (manifest) {
      try {
        const highlighter = await createHighlighter({
          themes: ['github-dark'],
          langs: ['json'],
        })
        highlighted = highlighter.codeToHtml(JSON.stringify(manifest, null, 2), {
          lang: 'json',
          theme: 'github-dark',
        })
      } catch {
        highlighted = `<pre class="text-gray-300 text-xs">${JSON.stringify(manifest, null, 2)}</pre>`
      }
    }

    loading = false
  })
</script>

<div>
  <div class="flex items-center gap-4 mb-6">
    <h2 class="text-lg font-semibold text-white">MCP</h2>
    <div class="flex gap-1 bg-surface-2 rounded-lg p-0.5">
      <button
        class="px-3 py-1 text-sm rounded-md transition-colors {tab === 'manifest' ? 'bg-surface-3 text-white' : 'text-gray-500 hover:text-white'}"
        onclick={() => tab = 'manifest'}
      >Manifest</button>
      <button
        class="px-3 py-1 text-sm rounded-md transition-colors {tab === 'prompt' ? 'bg-surface-3 text-white' : 'text-gray-500 hover:text-white'}"
        onclick={() => tab = 'prompt'}
      >System Prompt</button>
    </div>
  </div>

  {#if loading}
    <div class="h-64 bg-surface-1 rounded-lg animate-pulse"></div>
  {:else if tab === 'manifest'}
    {#if manifest}
      <div class="bg-surface-1 border border-surface-3 rounded-lg overflow-auto max-h-[70vh]">
        {#if highlighted}
          <!-- eslint-disable-next-line svelte/no-at-html-tags -->
          {@html highlighted}
        {:else}
          <pre class="p-4 text-xs text-gray-300 font-mono">{JSON.stringify(manifest, null, 2)}</pre>
        {/if}
      </div>
    {:else}
      <p class="text-gray-500 text-sm">No MCP manifest found.</p>
    {/if}
  {:else}
    {#if systemPrompt}
      <div class="bg-surface-1 border border-surface-3 rounded-lg p-4 overflow-auto max-h-[70vh]">
        <pre class="text-xs text-gray-300 font-mono whitespace-pre-wrap">{systemPrompt}</pre>
      </div>
    {:else}
      <p class="text-gray-500 text-sm">No system prompt found.</p>
    {/if}
  {/if}
</div>

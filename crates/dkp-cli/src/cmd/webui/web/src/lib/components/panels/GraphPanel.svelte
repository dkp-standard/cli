<script lang="ts">
  import { onDestroy } from 'svelte'
  import { api, type KnowledgeGraph } from '../../api.ts'

  let container = $state<HTMLDivElement | undefined>(undefined)
  let graph = $state<KnowledgeGraph | null>(null)
  let loading = $state(true)
  let error = $state<string | null>(null)
  let cy: unknown = null

  interface NodeInfo {
    id: string
    label: string
    type: string
    description: string
    x: number
    y: number
  }
  let selectedNode = $state<NodeInfo | null>(null)

  // Load graph data
  async function loadGraph() {
    try {
      graph = await api.graph()
    } catch (e) {
      error = String(e)
    } finally {
      loading = false
    }
  }
  loadGraph()

  // Initialize Cytoscape once the container is in the DOM and graph is loaded
  $effect(() => {
    if (!container || !graph || !graph.nodes?.length) return

    // Destroy previous instance if re-running
    if (cy && typeof (cy as any).destroy === 'function') {
      (cy as any).destroy()
      cy = null
    }

    const nodes = (graph.nodes ?? []).map(n => ({
      data: { id: n.id, label: n.label, type: n.node_type, description: n.description ?? '' }
    }))

    const edges = (graph.edges ?? []).map((e, i) => ({
      data: {
        id: `e${i}`,
        source: e.source,
        target: e.target,
        label: typeof e.relation === 'string' ? e.relation : JSON.stringify(e.relation),
        weight: e.weight ?? 1,
      }
    }))

    import('cytoscape').then(mod => {
      const cytoscape = mod.default
      const cyInstance = cytoscape({
        container,
        elements: { nodes, edges },
        style: [
          {
            selector: 'node',
            style: {
              'background-color': '#58a6ff',
              'label': 'data(label)',
              'color': '#e6edf3',
              'font-family': 'JetBrains Mono, monospace',
              'font-size': '10px',
              'text-valign': 'bottom',
              'text-margin-y': 4,
              'width': 28,
              'height': 28,
              'border-width': 1.5,
              'border-color': '#30363d',
            },
          },
          {
            selector: 'edge',
            style: {
              'line-color': '#30363d',
              'target-arrow-color': '#30363d',
              'target-arrow-shape': 'triangle',
              'curve-style': 'bezier',
              'label': 'data(label)',
              'font-size': '8px',
              'color': '#484f58',
              'text-rotation': 'autorotate',
            },
          },
          {
            selector: 'node:selected',
            style: { 'background-color': '#bc8cff', 'border-color': '#bc8cff' },
          },
          {
            selector: 'edge:selected',
            style: { 'line-color': '#bc8cff', 'target-arrow-color': '#bc8cff' },
          },
        ],
        layout: {
          name: 'cose',
          animate: false,
          nodeRepulsion: () => 400000,
          idealEdgeLength: () => 200,
          edgeElasticity: () => 100,
          gravity: 5,
          padding: 80,
        },
      })

      cyInstance.on('tap', 'node', (evt: any) => {
        const node = evt.target
        const pos = node.renderedPosition()
        selectedNode = {
          id: node.data('id'),
          label: node.data('label'),
          type: node.data('type'),
          description: node.data('description'),
          x: pos.x,
          y: pos.y,
        }
      })

      cyInstance.on('tap', (evt: any) => {
        if (evt.target === cyInstance) selectedNode = null
      })

      cy = cyInstance
    })
  })

  onDestroy(() => {
    if (cy && typeof (cy as any).destroy === 'function') (cy as any).destroy()
  })
</script>

<div class="flex flex-col h-full" style="min-height: 600px">
  <div class="flex items-center justify-between mb-4">
    <h2 class="text-lg font-semibold text-white">
      Knowledge Graph
      {#if graph}
        <span class="text-gray-500 text-sm ml-1">
          ({graph.nodes?.length ?? 0} nodes · {graph.edges?.length ?? 0} edges)
        </span>
      {/if}
    </h2>
    <p class="text-xs text-gray-500">Scroll to zoom · drag to pan · click node to select</p>
  </div>

  {#if loading}
    <div class="flex-1 bg-surface-1 rounded-lg animate-pulse" style="min-height: 500px"></div>
  {:else if error}
    <p class="text-accent-red text-sm">{error}</p>
  {:else if !graph || !graph.nodes?.length}
    <p class="text-gray-500 text-sm">No knowledge graph data found.</p>
  {:else}
    <div class="relative flex-1" style="min-height: 500px">
      <div
        bind:this={container}
        class="absolute inset-0 bg-surface-1 border border-surface-3 rounded-lg"
      ></div>

      {#if selectedNode}
        <div
          class="absolute z-10 pointer-events-none"
          style="left: {Math.min(selectedNode.x + 16, 9999)}px; top: {Math.min(selectedNode.y - 8, 9999)}px; transform: translateY(-100%)"
        >
          <div class="bg-surface-0 border border-surface-3 rounded-lg shadow-xl p-3 min-w-48 max-w-72 pointer-events-auto">
            <button
              class="absolute top-2 right-2 text-gray-500 hover:text-white text-xs leading-none"
              onclick={() => selectedNode = null}
            >✕</button>
            <div class="flex items-center gap-2 mb-2 pr-4">
              <span class="text-xs px-1.5 py-0.5 rounded bg-surface-3 text-accent-cyan font-mono">{selectedNode.type}</span>
              <span class="text-sm font-semibold text-white truncate">{selectedNode.label}</span>
            </div>
            {#if selectedNode.description}
              <p class="text-xs text-gray-400 leading-relaxed">{selectedNode.description}</p>
            {:else}
              <p class="text-xs text-gray-600 italic">No description</p>
            {/if}
            <p class="text-xs text-gray-700 mt-2 font-mono">{selectedNode.id}</p>
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>

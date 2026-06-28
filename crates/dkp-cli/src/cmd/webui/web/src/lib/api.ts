export interface PackSummary {
  name: string
  version: string
  domain: string
  update_date: string
  description?: string
  counts: AssetCounts
}

export interface AssetCounts {
  glossary: number
  rules: number
  chunks: number
  eval_cases: number
  has_graph: boolean
  has_skills: boolean
  has_l10n: boolean
  has_mcp_manifest: boolean
  has_cross_refs: boolean
  has_ontology: boolean
  has_constraints: boolean
  has_system_prompt: boolean
}

export interface GlossaryTerm {
  id: string
  term: string
  definition: string
  aliases?: string[]
  related?: string[]
  tags?: string[]
  stability?: string
  audience?: string[]
}

export interface Rule {
  id: string
  title: string
  description: string
  polarity: 'Affirmative' | 'Prohibitive'
  tags?: string[]
  confidence?: number
  stability?: string
  audience?: string[]
}

export interface RetrievalChunk {
  id: string
  title: string
  chunk_text: string
  tags?: string[]
  confidence?: number
  summary?: string
  retrieval_priority?: string
  stability?: string
  audience?: string[]
}

export interface EvalCase {
  query: string
  expected_dimensions: string[]
  critical_must_include?: string[]
  scoring_rubric: string
  tags?: string[]
  audience?: string[]
}

export interface KnowledgeGraph {
  nodes: KgNode[]
  edges: KgEdge[]
}

export interface KgNode {
  id: string
  node_type: string
  label: string
  description?: string
}

export interface KgEdge {
  source: string
  relation: string
  target: string
  weight?: number
  description?: string
}

export interface SearchResult {
  id: string
  asset_type: string
  title: string
  excerpt: string
  score: number
  source_file: string
}

async function get<T>(path: string): Promise<T> {
  const res = await fetch(path)
  if (!res.ok) throw new Error(`${res.status} ${res.statusText}`)
  return res.json() as Promise<T>
}

export const api = {
  pack: () => get<PackSummary>('/api/pack'),
  glossary: () => get<GlossaryTerm[]>('/api/assets/glossary'),
  rules: () => get<Rule[]>('/api/assets/rules'),
  chunks: () => get<RetrievalChunk[]>('/api/assets/chunks'),
  eval: () => get<EvalCase[]>('/api/assets/eval'),
  ontology: () => get<unknown[]>('/api/assets/ontology'),
  constraints: () => get<unknown>('/api/assets/constraints'),
  crossRefs: () => get<unknown>('/api/assets/cross-refs'),
  systemPrompt: () => get<{ content: string }>('/api/assets/system-prompt'),
  graph: () => get<KnowledgeGraph>('/api/graph'),
  search: (q: string, limit = 20) =>
    get<SearchResult[]>(`/api/search?q=${encodeURIComponent(q)}&limit=${limit}`),
}

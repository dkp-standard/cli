use dkp_core::{
    search::{SearchIndex, SearchResult},
    types::{
        chunks::RetrievalChunk, eval::EvalCase, manifest::McpConfig, mcp_manifest::McpManifest,
    },
};

use super::event::InputEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Assets = 0,
    Search = 1,
    Chunks = 2,
    Eval = 3,
    Mcp = 4,
}

impl Panel {
    const COUNT: usize = 5;

    pub fn next(self) -> Self {
        Self::from_index((self as usize + 1) % Self::COUNT)
    }

    pub fn prev(self) -> Self {
        Self::from_index((self as usize + Self::COUNT - 1) % Self::COUNT)
    }

    pub fn index(self) -> usize {
        self as usize
    }

    fn from_index(i: usize) -> Self {
        match i {
            0 => Self::Assets,
            1 => Self::Search,
            2 => Self::Chunks,
            3 => Self::Eval,
            4 => Self::Mcp,
            _ => Self::Assets,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Assets => "Assets",
            Self::Search => "Search",
            Self::Chunks => "Chunks",
            Self::Eval => "Eval",
            Self::Mcp => "MCP",
        }
    }
}

pub struct AssetSummary {
    pub glossary_count: usize,
    pub chunk_count: usize,
    pub rule_count: usize,
    pub eval_count: usize,
    pub has_graph: bool,
    pub has_skills: bool,
    pub has_l10n: bool,
    pub has_mcp_manifest: bool,
    pub has_assets_file: bool,
    pub has_cross_refs: bool,
    pub has_okf: bool,
}

pub enum DetailItem {
    Chunk(RetrievalChunk),
    SearchResult(SearchResult),
    EvalCase(EvalCase),
}

pub struct AppState {
    // Pack identity
    pub pack_name: String,
    pub pack_version: String,
    pub pack_domain: String,
    pub pack_update_date: String,

    // Navigation
    pub active_panel: Panel,
    pub detail_view: Option<DetailItem>,

    // Assets panel
    pub asset_summary: AssetSummary,
    pub assets_selected: usize,

    // Search panel
    pub search_input: String,
    pub search_input_focused: bool,
    pub search_results: Vec<SearchResult>,
    pub search_selected: usize,
    pub search_index: SearchIndex,

    // Chunks panel
    pub chunks: Vec<RetrievalChunk>,
    pub chunks_selected: usize,
    pub chunks_offset: usize,

    // Eval panel
    pub eval_cases: Vec<EvalCase>,
    pub eval_selected: usize,
    pub eval_offset: usize,

    // MCP panel
    pub mcp_config: Option<McpConfig>,
    pub mcp_manifest: Option<McpManifest>,
    pub mcp_selected: usize,

    pub should_quit: bool,
}

impl AppState {
    pub fn handle_event(&mut self, ev: InputEvent) {
        if self.detail_view.is_some() {
            match ev {
                InputEvent::Escape | InputEvent::Quit => {
                    self.detail_view = None;
                }
                _ => {}
            }
            return;
        }

        match ev {
            InputEvent::Quit => self.should_quit = true,
            InputEvent::Tab => {
                self.active_panel = self.active_panel.next();
                self.search_input_focused = false;
            }
            InputEvent::BackTab => {
                self.active_panel = self.active_panel.prev();
                self.search_input_focused = false;
            }
            InputEvent::SearchFocus => {
                self.active_panel = Panel::Search;
                self.search_input_focused = true;
            }
            InputEvent::Escape => {
                self.search_input_focused = false;
            }
            InputEvent::Enter => self.open_detail(),
            InputEvent::Up => self.scroll_up(),
            InputEvent::Down => self.scroll_down(),
            InputEvent::Char(c) => {
                if self.active_panel == Panel::Search && self.search_input_focused {
                    self.search_input.push(c);
                    self.run_search();
                }
            }
            InputEvent::Backspace
                if self.active_panel == Panel::Search && self.search_input_focused =>
            {
                self.search_input.pop();
                self.run_search();
            }
            _ => {}
        }
    }

    fn run_search(&mut self) {
        if self.search_input.trim().is_empty() {
            self.search_results.clear();
            self.search_selected = 0;
            return;
        }
        self.search_results = self
            .search_index
            .search(&self.search_input, 20)
            .unwrap_or_default();
        self.search_selected = 0;
    }

    fn scroll_up(&mut self) {
        match self.active_panel {
            Panel::Assets => {
                if self.assets_selected > 0 {
                    self.assets_selected -= 1;
                }
            }
            Panel::Search => {
                if self.search_selected > 0 {
                    self.search_selected -= 1;
                }
            }
            Panel::Chunks => {
                if self.chunks_selected > 0 {
                    self.chunks_selected -= 1;
                    if self.chunks_selected < self.chunks_offset {
                        self.chunks_offset = self.chunks_selected;
                    }
                }
            }
            Panel::Eval => {
                if self.eval_selected > 0 {
                    self.eval_selected -= 1;
                    if self.eval_selected < self.eval_offset {
                        self.eval_offset = self.eval_selected;
                    }
                }
            }
            Panel::Mcp => {
                if self.mcp_selected > 0 {
                    self.mcp_selected -= 1;
                }
            }
        }
    }

    fn scroll_down(&mut self) {
        match self.active_panel {
            Panel::Assets => {
                // 9 items in the asset tree
                if self.assets_selected + 1 < 9 {
                    self.assets_selected += 1;
                }
            }
            Panel::Search => {
                if self.search_selected + 1 < self.search_results.len() {
                    self.search_selected += 1;
                }
            }
            Panel::Chunks => {
                if self.chunks_selected + 1 < self.chunks.len() {
                    self.chunks_selected += 1;
                }
            }
            Panel::Eval => {
                if self.eval_selected + 1 < self.eval_cases.len() {
                    self.eval_selected += 1;
                }
            }
            Panel::Mcp => {
                let max = self
                    .mcp_manifest
                    .as_ref()
                    .map(|m| m.resources.len() + m.tools.len())
                    .unwrap_or(0);
                if self.mcp_selected + 1 < max {
                    self.mcp_selected += 1;
                }
            }
        }
    }

    fn open_detail(&mut self) {
        self.detail_view = match self.active_panel {
            Panel::Chunks => self
                .chunks
                .get(self.chunks_selected)
                .cloned()
                .map(DetailItem::Chunk),
            Panel::Search => self
                .search_results
                .get(self.search_selected)
                .cloned()
                .map(DetailItem::SearchResult),
            Panel::Eval => self
                .eval_cases
                .get(self.eval_selected)
                .cloned()
                .map(DetailItem::EvalCase),
            _ => None,
        };
    }
}

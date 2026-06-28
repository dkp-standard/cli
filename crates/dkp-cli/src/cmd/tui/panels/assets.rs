use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::cmd::tui::app::AppState;

pub fn draw(f: &mut Frame, area: Rect, state: &AppState) {
    let s = &state.asset_summary;

    let check = |b: bool| if b { "✓" } else { "–" };

    let items: Vec<ListItem> = vec![
        ListItem::new(Line::from(Span::styled(
            "  machine/",
            Style::default().add_modifier(Modifier::BOLD),
        ))),
        ListItem::new(format!(
            "    retrieval_chunks.jsonl   ({} chunks)",
            s.chunk_count
        )),
        ListItem::new(format!(
            "    glossary.json            ({} terms)",
            s.glossary_count
        )),
        ListItem::new(format!(
            "    rules.json               ({} rules)",
            s.rule_count
        )),
        ListItem::new(format!(
            "    eval_set.jsonl           ({} cases)",
            s.eval_count
        )),
        ListItem::new(format!(
            "    knowledge_graph.json     {}",
            check(s.has_graph)
        )),
        ListItem::new(format!(
            "    assets.json              {}",
            check(s.has_assets_file)
        )),
        ListItem::new(format!(
            "    cross_refs.json          {}",
            check(s.has_cross_refs)
        )),
        ListItem::new(format!(
            "    mcp_manifest.json        {}",
            check(s.has_mcp_manifest)
        )),
        ListItem::new(Line::from(Span::styled(
            "  okf/",
            Style::default().add_modifier(Modifier::BOLD),
        ))),
        ListItem::new(format!("    bundle                   {}", check(s.has_okf))),
        ListItem::new(Line::from(Span::styled(
            "  skills/",
            Style::default().add_modifier(Modifier::BOLD),
        ))),
        ListItem::new(format!(
            "    present                  {}",
            check(s.has_skills)
        )),
        ListItem::new(Line::from(Span::styled(
            "  l10n/",
            Style::default().add_modifier(Modifier::BOLD),
        ))),
        ListItem::new(format!(
            "    present                  {}",
            check(s.has_l10n)
        )),
    ];

    let mut list_state = ListState::default();
    list_state.select(Some(state.assets_selected));

    let list = List::new(items)
        .block(Block::default().title("Assets").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, area, &mut list_state);
}

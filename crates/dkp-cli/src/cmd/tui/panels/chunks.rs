use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::cmd::tui::app::AppState;

pub fn draw(f: &mut Frame, area: Rect, state: &AppState) {
    let panes =
        Layout::vertical([Constraint::Percentage(40), Constraint::Percentage(60)]).split(area);

    // Chunk list
    let items: Vec<ListItem> = state
        .chunks
        .iter()
        .map(|c| {
            let priority = c
                .retrieval_priority
                .as_ref()
                .map(|p| format!("{p:?}").to_uppercase())
                .unwrap_or_else(|| "NORMAL".to_string());
            let stability = c
                .stability
                .as_ref()
                .map(|s| format!("{s:?}").to_lowercase())
                .unwrap_or_default();

            let line = Line::from(vec![
                Span::styled(format!("{:<14}", &c.id), Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!(" {:>8} ", priority),
                    Style::default().fg(Color::Yellow),
                ),
                Span::styled(
                    format!("{:<10} ", stability),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::raw(truncate(&c.title, 40)),
            ]);
            ListItem::new(line)
        })
        .collect();

    let mut list_state = ListState::default();
    if !state.chunks.is_empty() {
        list_state.select(Some(state.chunks_selected));
    }

    let list = List::new(items)
        .block(
            Block::default()
                .title(format!(
                    "Chunks ({})  ↑↓ navigate  Enter: full detail",
                    state.chunks.len()
                ))
                .borders(Borders::ALL),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );
    f.render_stateful_widget(list, panes[0], &mut list_state);

    // Preview pane
    let preview_text = state
        .chunks
        .get(state.chunks_selected)
        .map(|c| {
            let conf = c
                .confidence
                .map(|v| format!("{v:.2}"))
                .unwrap_or_else(|| "—".to_string());
            let tokens = c
                .token_count
                .map(|n| n.to_string())
                .unwrap_or_else(|| "—".to_string());
            format!(
                "ID: {}  Source: {}  Confidence: {}  Tokens: {}\nTags: {}\n\n{}",
                c.id,
                c.source_ref,
                conf,
                tokens,
                c.tags.join(", "),
                c.chunk_text,
            )
        })
        .unwrap_or_default();

    let preview = Paragraph::new(preview_text)
        .block(Block::default().title("Preview").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(preview, panes[1]);
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max.saturating_sub(1)])
    }
}

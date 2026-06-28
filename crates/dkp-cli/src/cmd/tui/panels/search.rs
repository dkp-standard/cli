use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::cmd::tui::app::AppState;

pub fn draw(f: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(area);

    // Input box
    let cursor = if state.search_input_focused {
        "█"
    } else {
        ""
    };
    let input_text = format!("{}{}", state.search_input, cursor);
    let input = Paragraph::new(input_text).block(
        Block::default()
            .title("Search  (/ to focus, Esc to blur)")
            .borders(Borders::ALL)
            .border_style(if state.search_input_focused {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            }),
    );
    f.render_widget(input, chunks[0]);

    // Results list
    let items: Vec<ListItem> = state
        .search_results
        .iter()
        .map(|r| {
            let line = Line::from(vec![
                Span::styled(
                    format!("[{}] ", r.asset_type),
                    Style::default().fg(Color::Cyan),
                ),
                Span::raw(truncate(&r.title, 40)),
                Span::styled(" — ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    truncate(&r.excerpt, 60),
                    Style::default().fg(Color::DarkGray),
                ),
            ]);
            ListItem::new(line)
        })
        .collect();

    let placeholder = if state.search_input.is_empty() {
        "Type to search across terms, chunks, rules, and constraints"
    } else if state.search_results.is_empty() {
        "No results"
    } else {
        ""
    };

    let mut list_state = ListState::default();
    if !state.search_results.is_empty() {
        list_state.select(Some(state.search_selected));
    }

    if items.is_empty() {
        let p = Paragraph::new(placeholder).block(
            Block::default()
                .title(format!("Results ({})", state.search_results.len()))
                .borders(Borders::ALL),
        );
        f.render_widget(p, chunks[1]);
    } else {
        let list = List::new(items)
            .block(
                Block::default()
                    .title(format!("Results ({})", state.search_results.len()))
                    .borders(Borders::ALL),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            );
        f.render_stateful_widget(list, chunks[1], &mut list_state);
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max.saturating_sub(1)])
    }
}

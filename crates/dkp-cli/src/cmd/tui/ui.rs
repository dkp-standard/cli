use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use super::{
    app::{AppState, Panel},
    panels,
};

pub fn draw(f: &mut Frame, state: &AppState) {
    let area = f.area();

    let rows = Layout::vertical([
        Constraint::Length(1), // title bar
        Constraint::Length(3), // tab bar
        Constraint::Min(0),    // panel content
        Constraint::Length(1), // help bar
    ])
    .split(area);

    // Title bar
    let title = Paragraph::new(format!(
        " DKP TUI  —  {} v{}  [{}]  updated: {}",
        state.pack_name, state.pack_version, state.pack_domain, state.pack_update_date
    ))
    .style(
        Style::default()
            .bg(Color::Blue)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    );
    f.render_widget(title, rows[0]);

    // Tab bar
    let tab_labels: Vec<Span> = [
        Panel::Assets,
        Panel::Search,
        Panel::Chunks,
        Panel::Eval,
        Panel::Mcp,
    ]
    .iter()
    .map(|p| Span::raw(p.label()))
    .collect();

    let tabs = Tabs::new(tab_labels)
        .block(Block::default().borders(Borders::ALL))
        .select(state.active_panel.index())
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .divider(Span::raw(" │ "));
    f.render_widget(tabs, rows[1]);

    // Panel content
    match state.active_panel {
        Panel::Assets => panels::assets::draw(f, rows[2], state),
        Panel::Search => panels::search::draw(f, rows[2], state),
        Panel::Chunks => panels::chunks::draw(f, rows[2], state),
        Panel::Eval => panels::eval::draw(f, rows[2], state),
        Panel::Mcp => panels::mcp::draw(f, rows[2], state),
    }

    // Detail overlay (drawn on top of content)
    if state.detail_view.is_some() {
        panels::detail::draw(f, rows[2], state);
    }

    // Help bar
    let help = Paragraph::new(
        "  Tab/S-Tab: switch panel   /: search   ↑↓ or j/k: navigate   Enter: detail   Esc: back   q: quit",
    )
    .style(Style::default().fg(Color::DarkGray));
    f.render_widget(help, rows[3]);
}

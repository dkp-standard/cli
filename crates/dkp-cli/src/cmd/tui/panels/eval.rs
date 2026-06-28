use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::cmd::tui::app::AppState;

pub fn draw(f: &mut Frame, area: Rect, state: &AppState) {
    let panes = Layout::vertical([
        Constraint::Length(2),
        Constraint::Percentage(40),
        Constraint::Percentage(58),
    ])
    .split(area);

    // Notice banner
    let notice = Paragraph::new(
        "  LLM eval not available in TUI — showing eval cases only  (use `dkp eval` to run scores)",
    )
    .style(Style::default().fg(Color::Yellow));
    f.render_widget(notice, panes[0]);

    // Cases list
    let items: Vec<ListItem> = state
        .eval_cases
        .iter()
        .map(|c| {
            let line = Line::from(vec![
                Span::raw(truncate(&c.query, 60)),
                Span::styled(
                    format!("  [{} dims]", c.expected_dimensions.len()),
                    Style::default().fg(Color::DarkGray),
                ),
            ]);
            ListItem::new(line)
        })
        .collect();

    let mut list_state = ListState::default();
    if !state.eval_cases.is_empty() {
        list_state.select(Some(state.eval_selected));
    }

    let list = List::new(items)
        .block(
            Block::default()
                .title(format!("Eval Cases ({})", state.eval_cases.len()))
                .borders(Borders::ALL),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );
    f.render_stateful_widget(list, panes[1], &mut list_state);

    // Detail pane
    let detail_text = state
        .eval_cases
        .get(state.eval_selected)
        .map(|c| {
            format!(
                "Query:\n  {}\n\nExpected Dimensions:\n{}\n\nMust Include:\n{}\n\nRubric:\n  {}",
                c.query,
                c.expected_dimensions
                    .iter()
                    .map(|d| format!("  • {d}"))
                    .collect::<Vec<_>>()
                    .join("\n"),
                c.critical_must_include
                    .iter()
                    .map(|d| format!("  • {d}"))
                    .collect::<Vec<_>>()
                    .join("\n"),
                c.scoring_rubric,
            )
        })
        .unwrap_or_default();

    let detail = Paragraph::new(detail_text)
        .block(Block::default().title("Detail").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(detail, panes[2]);
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max.saturating_sub(1)])
    }
}

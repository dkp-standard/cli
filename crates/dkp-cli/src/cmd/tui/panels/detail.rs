use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::cmd::tui::app::{AppState, DetailItem};

pub fn draw(f: &mut Frame, area: Rect, state: &AppState) {
    let Some(item) = &state.detail_view else {
        return;
    };

    let popup_area = centered_rect(82, 80, area);
    f.render_widget(Clear, popup_area);

    let (title, body) = match item {
        DetailItem::Chunk(c) => {
            let conf = c
                .confidence
                .map(|v| format!("{v:.2}"))
                .unwrap_or_else(|| "—".to_string());
            let tokens = c
                .token_count
                .map(|n| n.to_string())
                .unwrap_or_else(|| "—".to_string());
            let priority = c
                .retrieval_priority
                .as_ref()
                .map(|p| format!("{p:?}"))
                .unwrap_or_else(|| "Normal".to_string());
            let stability = c
                .stability
                .as_ref()
                .map(|s| format!("{s:?}"))
                .unwrap_or_default();

            let body = format!(
                "ID:         {}\nSource:     {}\nConfidence: {}  Tokens: {}  Priority: {}  Stability: {}\nTags:       {}\n\n{}",
                c.id,
                c.source_ref,
                conf,
                tokens,
                priority,
                stability,
                c.tags.join(", "),
                c.chunk_text,
            );
            (format!(" {} ", c.title), body)
        }
        DetailItem::SearchResult(r) => {
            let body = format!(
                "ID:     {}\nType:   {}\nScore:  {:.3}\nSource: {}\n\n{}",
                r.id, r.asset_type, r.score, r.source_file, r.excerpt,
            );
            (format!(" {} ", r.title), body)
        }
        DetailItem::EvalCase(c) => {
            let body = format!(
                "Query:\n  {}\n\nExpected Dimensions:\n{}\n\nMust Include:\n{}\n\nScoring Rubric:\n  {}\n\nDataset: {}  Model: {}",
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
                c.version_meta.dataset_version,
                c.version_meta.model_version,
            );
            (" Eval Case ".to_string(), body)
        }
    };

    let block = Block::default()
        .title(title)
        .title_style(Style::default().add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let para = Paragraph::new(body).block(block).wrap(Wrap { trim: true });

    f.render_widget(para, popup_area);

    // Dismiss hint in bottom-right corner of popup
    let hint = " Esc: close ";
    let hint_area = Rect {
        x: popup_area.x + popup_area.width.saturating_sub(hint.len() as u16 + 1),
        y: popup_area.y + popup_area.height.saturating_sub(1),
        width: hint.len() as u16,
        height: 1,
    };
    f.render_widget(
        Paragraph::new(hint).style(Style::default().fg(Color::DarkGray)),
        hint_area,
    );
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(area);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(vertical[1])[1]
}

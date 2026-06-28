use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::cmd::tui::app::AppState;

pub fn draw(f: &mut Frame, area: Rect, state: &AppState) {
    // Split into resources (left) and tools (right)
    let cols =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);

    // Resources
    let resource_items: Vec<ListItem> = if let Some(mcp) = &state.mcp_manifest {
        mcp.resources
            .iter()
            .map(|r| {
                let line = Line::from(vec![
                    Span::styled(
                        format!("{:<30}", truncate(&r.uri_template, 28)),
                        Style::default().fg(Color::Cyan),
                    ),
                    Span::styled(
                        format!(" {:>8}", r.resource_type),
                        Style::default().fg(Color::Yellow),
                    ),
                    Span::styled(
                        format!("  {} items", r.count),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]);
                ListItem::new(line)
            })
            .collect()
    } else {
        // Fall back to manifest.mcp config
        state
            .mcp_config
            .as_ref()
            .and_then(|m| m.resource_server.as_ref())
            .map(|rs| {
                vec![ListItem::new(format!(
                    "URI scheme: {}",
                    rs.uri_scheme.as_deref().unwrap_or("—")
                ))]
            })
            .unwrap_or_else(|| {
                vec![ListItem::new(Span::styled(
                    "No mcp_manifest.json — run `dkp mcp-manifest` to generate",
                    Style::default().fg(Color::Yellow),
                ))]
            })
    };

    let resources = List::new(resource_items)
        .block(
            Block::default()
                .title("Resources")
                .borders(Borders::ALL)
                .border_style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .highlight_style(Style::default().bg(Color::DarkGray));
    f.render_widget(resources, cols[0]);

    // Tools
    let tool_items: Vec<ListItem> = if let Some(mcp) = &state.mcp_manifest {
        mcp.tools
            .iter()
            .map(|t| {
                let line = Line::from(vec![
                    Span::styled(
                        format!("{:<20}", truncate(&t.name, 18)),
                        Style::default().fg(Color::Green),
                    ),
                    Span::raw("  "),
                    Span::styled(
                        truncate(&t.description, 50),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]);
                ListItem::new(line)
            })
            .collect()
    } else {
        state
            .mcp_config
            .as_ref()
            .and_then(|m| m.tool_provider.as_ref())
            .map(|tp| {
                tp.tools
                    .iter()
                    .map(|name| {
                        ListItem::new(Span::styled(
                            name.clone(),
                            Style::default().fg(Color::Green),
                        ))
                    })
                    .collect()
            })
            .unwrap_or_default()
    };

    let tools = List::new(tool_items)
        .block(
            Block::default()
                .title("Tools")
                .borders(Borders::ALL)
                .border_style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .highlight_style(Style::default().bg(Color::DarkGray));
    f.render_widget(tools, cols[1]);
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max.saturating_sub(1)])
    }
}

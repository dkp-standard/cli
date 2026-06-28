use serde::Serialize;

/// The three output modes all commands support via `--output`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum, Default)]
pub enum OutputFormat {
    /// Human-readable plain text (default)
    #[default]
    Plain,
    /// Aligned table using comfy-table
    Table,
    /// Pretty-printed JSON
    Json,
}

/// Anything a command can render in all three output modes.
///
/// Implement this trait on each command's output struct. The `--output` flag
/// dispatches to the right renderer automatically via `render()` / `print()`.
pub trait Render: Serialize {
    fn render_plain(&self) -> String;

    fn render_table(&self) -> String {
        self.render_plain()
    }

    fn render(&self, fmt: OutputFormat) -> String {
        match fmt {
            OutputFormat::Plain => self.render_plain(),
            OutputFormat::Table => self.render_table(),
            OutputFormat::Json => serde_json::to_string_pretty(self)
                .unwrap_or_else(|e| format!("{{\"error\": \"{e}\"}}",)),
        }
    }

    fn print(&self, fmt: OutputFormat) {
        println!("{}", self.render(fmt));
    }
}

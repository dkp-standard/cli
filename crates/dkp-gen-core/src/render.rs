//! Renders `human/handbook.md` into `handbook.pdf` and `handbook.epub`,
//! per DKP spec §11.2. Both formats are produced in-process by pure-Rust
//! crates (`markdown2pdf`, `epub-builder` + `pulldown-cmark`) — no
//! external binaries (no pandoc/xelatex) are shelled out to, so `dkp`
//! stays a single self-contained binary.

use std::fs::File;
use std::path::Path;

use epub_builder::{EpubBuilder, EpubContent, ZipLibrary};

use crate::error::GenResult;

/// Bundled Unicode body font (Bitstream Vera-licensed, see
/// `assets/DejaVuSans-LICENSE.txt`). The renderer's built-in fallback
/// (Type 1 Helvetica) only covers WinAnsi/Latin-1, so em-dashes,
/// non-breaking hyphens, smart quotes, etc. would render as `?` without
/// an explicit Unicode-capable font. Embedding rather than probing the
/// host's installed fonts keeps rendering deterministic across machines/CI.
static DEJAVU_SANS: &[u8] = include_bytes!("../assets/DejaVuSans.ttf");

#[derive(Debug, Default, Clone)]
pub struct RenderReport {
    pub epub_written: bool,
    pub pdf_written: bool,
    pub warnings: Vec<String>,
}

/// Renders `handbook_md` (the contents of `human/handbook.md`) to
/// `handbook.pdf` and `handbook.epub` inside `human_dir`. A failure in
/// one format is recorded in `RenderReport.warnings` rather than
/// propagated, so it doesn't block the other format or the caller's
/// generation/build step — these renderings are optional per spec.
pub fn render_handbook_formats(handbook_md: &str, human_dir: &Path) -> GenResult<RenderReport> {
    let mut report = RenderReport::default();

    match render_pdf(handbook_md, &human_dir.join("handbook.pdf")) {
        Ok(()) => report.pdf_written = true,
        Err(e) => report.warnings.push(format!("handbook.pdf: {e}")),
    }

    match render_epub(handbook_md, &human_dir.join("handbook.epub")) {
        Ok(()) => report.epub_written = true,
        Err(e) => report.warnings.push(format!("handbook.epub: {e}")),
    }

    Ok(report)
}

fn render_pdf(markdown: &str, out: &Path) -> Result<(), String> {
    let mut font_config = markdown2pdf::fonts::FontConfig::new();
    font_config.default_font_source = Some(markdown2pdf::fonts::FontSource::bytes(DEJAVU_SANS));

    markdown2pdf::parse_into_file(
        markdown.to_string(),
        out,
        markdown2pdf::config::ConfigSource::Default,
        Some(&font_config),
    )
    .map_err(|e| e.to_string())
}

fn render_epub(markdown: &str, out: &Path) -> Result<(), String> {
    let mut html_body = String::new();
    pulldown_cmark::html::push_html(&mut html_body, pulldown_cmark::Parser::new(markdown));
    let xhtml = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <!DOCTYPE html>\n\
         <html xmlns=\"http://www.w3.org/1999/xhtml\">\n\
         <head><title>Handbook</title></head>\n\
         <body>\n{html_body}\n</body>\n</html>"
    );

    let zip = ZipLibrary::new().map_err(|e| e.to_string())?;
    let mut builder = EpubBuilder::new(zip).map_err(|e| e.to_string())?;
    builder
        .metadata("title", "Handbook")
        .map_err(|e| e.to_string())?;
    builder
        .add_content(EpubContent::new("handbook.xhtml", xhtml.as_bytes()).title("Handbook"))
        .map_err(|e| e.to_string())?;

    let mut file = File::create(out).map_err(|e| e.to_string())?;
    builder.generate(&mut file).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn renders_both_formats_for_simple_markdown() {
        let tmp = TempDir::new().unwrap();
        let markdown = "# Handbook\n\nSome **bold** text and a list:\n\n- one\n- two\n";

        let report = render_handbook_formats(markdown, tmp.path()).unwrap();

        assert!(report.warnings.is_empty(), "warnings: {:?}", report.warnings);
        assert!(report.pdf_written);
        assert!(report.epub_written);

        let pdf = tmp.path().join("handbook.pdf");
        let epub = tmp.path().join("handbook.epub");
        assert!(pdf.exists());
        assert!(epub.exists());
        assert!(std::fs::metadata(&pdf).unwrap().len() > 0);
        assert!(std::fs::metadata(&epub).unwrap().len() > 0);
    }

    #[test]
    fn renders_empty_markdown_without_error() {
        let tmp = TempDir::new().unwrap();
        let report = render_handbook_formats("", tmp.path()).unwrap();
        assert!(report.warnings.is_empty(), "warnings: {:?}", report.warnings);
        assert!(report.pdf_written);
        assert!(report.epub_written);
    }
}

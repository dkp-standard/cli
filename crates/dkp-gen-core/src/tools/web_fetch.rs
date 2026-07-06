use crate::error::{GenError, GenResult};

const MAX_CHARS: usize = 16_000;
const USER_AGENT: &str = concat!("dkp-cli/", env!("CARGO_PKG_VERSION"));

/// Fetches a URL and returns its content as plain text, truncated to a
/// token-ish budget. HTML is converted to text; other content types pass
/// through as-is (still truncated).
pub struct WebFetchTool {
    http: reqwest::Client,
}

impl WebFetchTool {
    pub fn new(timeout_secs: u64) -> GenResult<Self> {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(timeout_secs))
            .user_agent(USER_AGENT)
            .build()
            .map_err(|e| GenError::ToolFailed {
                name: "web_fetch".to_string(),
                message: format!("failed to build HTTP client: {e}"),
            })?;
        Ok(Self { http })
    }

    pub async fn fetch(&self, url: &str) -> GenResult<String> {
        let resp = self
            .http
            .get(url)
            .send()
            .await
            .map_err(|e| GenError::ToolFailed {
                name: "web_fetch".to_string(),
                message: format!("request to {url} failed: {e}"),
            })?;

        let status = resp.status();
        let content_type = resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        let body = resp.text().await.map_err(|e| GenError::ToolFailed {
            name: "web_fetch".to_string(),
            message: format!("failed to read response body from {url}: {e}"),
        })?;

        if !status.is_success() {
            return Err(GenError::ToolFailed {
                name: "web_fetch".to_string(),
                message: format!("{url} returned HTTP {}", status.as_u16()),
            });
        }

        let text = if content_type.contains("html") {
            html2text::from_read(body.as_bytes(), 120).unwrap_or(body)
        } else {
            body
        };

        Ok(truncate(&text, MAX_CHARS))
    }
}

fn truncate(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let mut truncated: String = text.chars().take(max_chars).collect();
    truncated.push_str("\n[truncated]");
    truncated
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn fetch_converts_html_to_text() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                "<html><body><h1>Title</h1><p>Hello world.</p></body></html>",
                "text/html",
            ))
            .mount(&server)
            .await;

        let tool = WebFetchTool::new(30).unwrap();
        let text = tool.fetch(&server.uri()).await.unwrap();
        assert!(text.contains("Title"));
        assert!(text.contains("Hello world."));
        assert!(!text.contains("<h1>") && !text.contains("<p>"));
    }

    #[tokio::test]
    async fn fetch_truncates_long_content() {
        let server = MockServer::start().await;
        let long_body = "a".repeat(MAX_CHARS + 500);
        Mock::given(method("GET"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(long_body)
                    .insert_header("content-type", "text/plain"),
            )
            .mount(&server)
            .await;

        let tool = WebFetchTool::new(30).unwrap();
        let text = tool.fetch(&server.uri()).await.unwrap();
        assert!(text.ends_with("[truncated]"));
        assert!(text.chars().count() <= MAX_CHARS + "\n[truncated]".len());
    }

    #[tokio::test]
    async fn fetch_errors_on_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let tool = WebFetchTool::new(30).unwrap();
        let err = tool.fetch(&server.uri()).await.unwrap_err();
        assert!(matches!(err, GenError::ToolFailed { .. }));
    }
}

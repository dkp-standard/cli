use async_trait::async_trait;
use serde_json::{json, Value};

use crate::config::GenConfig;
use crate::error::{GenError, GenResult};
use crate::llm::client::LlmClient;

pub struct OpenAiClient {
    http: reqwest::Client,
    base_url: String,
    api_key: String,
    model: String,
}

impl OpenAiClient {
    pub fn new(config: &GenConfig) -> GenResult<Self> {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()
            .map_err(|e| GenError::Http {
                status: 0,
                body: e.to_string(),
            })?;
        Ok(Self {
            http,
            base_url: config.base_url.trim_end_matches('/').to_string(),
            api_key: config.api_key.clone(),
            model: config.model.clone(),
        })
    }
}

#[async_trait]
impl LlmClient for OpenAiClient {
    async fn complete(&self, system: &str, user: &str) -> GenResult<String> {
        let url = format!("{}/chat/completions", self.base_url);
        let body = json!({
            "model": self.model,
            "messages": [
                {"role": "system", "content": system},
                {"role": "user",   "content": user}
            ]
        });

        let max_attempts = 4u32;
        let mut attempts = 0u32;
        let mut last_err: GenError = GenError::Http {
            status: 0,
            body: "no attempts made".into(),
        };
        loop {
            if attempts >= max_attempts {
                return Err(last_err);
            }
            if attempts > 0 {
                let delay = std::time::Duration::from_secs(2u64.pow(attempts));
                tokio::time::sleep(delay).await;
            }
            attempts += 1;

            let send_result = self
                .http
                .post(&url)
                .bearer_auth(&self.api_key)
                .json(&body)
                .send()
                .await;

            let resp = match send_result {
                Ok(r) => r,
                Err(e) => {
                    last_err = GenError::Http {
                        status: 0,
                        body: e.to_string(),
                    };
                    continue;
                }
            };

            let status = resp.status().as_u16();
            if status == 429 || status == 503 {
                last_err = GenError::Http {
                    status,
                    body: "rate limit / unavailable".into(),
                };
                continue;
            }

            if !resp.status().is_success() {
                let body = resp.text().await.unwrap_or_default();
                return Err(GenError::Http { status, body });
            }

            let json_result: Result<Value, _> = resp.json().await;
            let json = match json_result {
                Ok(v) => v,
                Err(e) => {
                    last_err = GenError::Http {
                        status: 0,
                        body: format!("error decoding response body: {e}"),
                    };
                    continue;
                }
            };

            let text = json["choices"][0]["message"]["content"]
                .as_str()
                .ok_or_else(|| GenError::LlmRefusal("no content in response".into()))?
                .to_string();

            return Ok(text);
        }
    }
}

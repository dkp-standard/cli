use async_trait::async_trait;
use serde_json::{json, Value};

use crate::config::GenConfig;
use crate::error::{GenError, GenResult};
use crate::llm::client::LlmClient;
use crate::tools::ToolExecutor;

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

impl OpenAiClient {
    /// POST a chat/completions body with the existing retry/backoff policy
    /// (retries on send errors, 429, and 503; up to 4 attempts total).
    async fn post_chat(&self, body: &Value) -> GenResult<Value> {
        let url = format!("{}/chat/completions", self.base_url);

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
                .json(body)
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

            return Ok(json);
        }
    }
}

#[async_trait]
impl LlmClient for OpenAiClient {
    async fn complete(&self, system: &str, user: &str) -> GenResult<String> {
        let body = json!({
            "model": self.model,
            "messages": [
                {"role": "system", "content": system},
                {"role": "user",   "content": user}
            ]
        });
        let json = self.post_chat(&body).await?;
        let text = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| GenError::LlmRefusal("no content in response".into()))?
            .to_string();
        Ok(text)
    }

    async fn complete_with_tools(
        &self,
        system: &str,
        user: &str,
        tools: &dyn ToolExecutor,
        max_turns: u32,
    ) -> GenResult<String> {
        let tool_specs: Vec<Value> = tools.specs().iter().map(|t| t.to_openai()).collect();
        let mut messages = vec![
            json!({"role": "system", "content": system}),
            json!({"role": "user", "content": user}),
        ];

        for _ in 0..max_turns {
            let body = json!({
                "model": self.model,
                "messages": messages,
                "tools": tool_specs,
            });
            let json = self.post_chat(&body).await?;
            let message = json["choices"][0]["message"].clone();

            let tool_calls = message["tool_calls"]
                .as_array()
                .cloned()
                .unwrap_or_default();
            if tool_calls.is_empty() {
                if let Some(text) = message["content"].as_str() {
                    return Ok(text.to_string());
                }
                return Err(GenError::LlmRefusal("no content in response".into()));
            }

            // Thread the assistant's tool-call turn back verbatim before
            // answering each call — every tool_call_id must get a matching
            // role:"tool" reply before the next request.
            messages.push(message);
            for call in &tool_calls {
                let call_id = call["id"].as_str().unwrap_or_default().to_string();
                let name = call["function"]["name"].as_str().unwrap_or_default();
                // `arguments` is a JSON-encoded string, not an object.
                let args_str = call["function"]["arguments"].as_str().unwrap_or("{}");
                let args: Value = serde_json::from_str(args_str).unwrap_or(json!({}));

                let result = match tools.execute(name, &args).await {
                    Ok(text) => text,
                    Err(e) => format!("ERROR: {e}"),
                };

                messages.push(json!({
                    "role": "tool",
                    "tool_call_id": call_id,
                    "content": result,
                }));
            }
        }

        // Tool turns exhausted: force one final text-only reply.
        let body = json!({
            "model": self.model,
            "messages": messages,
            "tool_choice": "none",
        });
        let json = self.post_chat(&body).await?;
        json["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| GenError::ToolLoop("max tool turns exceeded".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_config(base_url: String) -> GenConfig {
        GenConfig {
            base_url,
            api_key: "test-key".to_string(),
            model: "test-model".to_string(),
            overwrite: false,
            timeout_secs: 30,
            ..Default::default()
        }
    }

    /// Single retry-then-success: first call gets a 429, second succeeds.
    /// Deliberately the only retry scenario tested here — the client's real
    /// exponential backoff means each additional retry adds real wall-clock
    /// delay, so we keep this to exactly one retry to stay fast in CI.
    #[tokio::test]
    async fn complete_retries_once_on_429_then_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/chat/completions"))
            .respond_with(ResponseTemplate::new(429))
            .up_to_n_times(1)
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "choices": [{"message": {"content": "Hello from the mock."}}]
            })))
            .mount(&server)
            .await;

        let config = test_config(server.uri());
        let client = OpenAiClient::new(&config).unwrap();
        let result = client
            .complete("system prompt", "user prompt")
            .await
            .unwrap();
        assert_eq!(result, "Hello from the mock.");
    }

    #[tokio::test]
    async fn complete_non_retryable_error_fails_immediately() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/chat/completions"))
            .respond_with(ResponseTemplate::new(400).set_body_string("bad request"))
            .mount(&server)
            .await;

        let config = test_config(server.uri());
        let client = OpenAiClient::new(&config).unwrap();
        let err = client.complete("system", "user").await.unwrap_err();
        assert!(matches!(err, GenError::Http { status: 400, .. }));
    }

    struct EchoExecutor;

    #[async_trait]
    impl ToolExecutor for EchoExecutor {
        fn specs(&self) -> Vec<crate::tools::ToolSpec> {
            vec![crate::tools::ToolSpec {
                name: "echo".to_string(),
                description: "Echoes the input.".to_string(),
                parameters: json!({"type": "object", "properties": {"text": {"type": "string"}}}),
            }]
        }

        async fn execute(&self, name: &str, arguments: &Value) -> GenResult<String> {
            assert_eq!(name, "echo");
            Ok(format!(
                "echoed: {}",
                arguments["text"].as_str().unwrap_or_default()
            ))
        }
    }

    #[tokio::test]
    async fn complete_with_tools_threads_tool_call_round_trip() {
        let server = MockServer::start().await;

        // First response: a tool call.
        Mock::given(method("POST"))
            .and(path("/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "choices": [{"message": {
                    "role": "assistant",
                    "content": null,
                    "tool_calls": [{
                        "id": "call_1",
                        "type": "function",
                        "function": {"name": "echo", "arguments": "{\"text\":\"hi\"}"}
                    }]
                }}]
            })))
            .up_to_n_times(1)
            .mount(&server)
            .await;

        // Second response: final text answer.
        Mock::given(method("POST"))
            .and(path("/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "choices": [{"message": {"content": "done: echoed: hi"}}]
            })))
            .mount(&server)
            .await;

        let config = test_config(server.uri());
        let client = OpenAiClient::new(&config).unwrap();
        let result = client
            .complete_with_tools("system", "user", &EchoExecutor, 6)
            .await
            .unwrap();
        assert_eq!(result, "done: echoed: hi");

        // Verify the second request threaded the assistant tool_calls message
        // and a matching role:"tool" reply back to the server.
        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests.len(), 2);
        let second_body: Value = requests[1].body_json().unwrap();
        let messages = second_body["messages"].as_array().unwrap();
        assert!(messages
            .iter()
            .any(|m| m["role"] == "assistant" && m["tool_calls"].is_array()));
        assert!(messages
            .iter()
            .any(|m| m["role"] == "tool" && m["tool_call_id"] == "call_1"));
    }

    #[tokio::test]
    async fn complete_with_tools_forces_final_answer_on_turn_exhaustion() {
        let server = MockServer::start().await;
        // Always returns a tool call — the loop should exhaust max_turns and
        // send one final tool_choice:"none" request.
        Mock::given(method("POST"))
            .and(path("/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "choices": [{"message": {
                    "role": "assistant",
                    "content": null,
                    "tool_calls": [{
                        "id": "call_x",
                        "type": "function",
                        "function": {"name": "echo", "arguments": "{\"text\":\"x\"}"}
                    }]
                }}]
            })))
            .up_to_n_times(2)
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "choices": [{"message": {"content": "forced final answer"}}]
            })))
            .mount(&server)
            .await;

        let config = test_config(server.uri());
        let client = OpenAiClient::new(&config).unwrap();
        let result = client
            .complete_with_tools("system", "user", &EchoExecutor, 2)
            .await
            .unwrap();
        assert_eq!(result, "forced final answer");

        let requests = server.received_requests().await.unwrap();
        let last_body: Value = requests.last().unwrap().body_json().unwrap();
        assert_eq!(last_body["tool_choice"], "none");
    }
}

use reqwest::Client;
use serde_json::{json, Value};
use futures_util::StreamExt;
use crate::infrastructure::llm::config::LLMConfig;

/// Anthropic Messages API provider
pub struct AnthropicProvider {
    client: Client,
}

impl AnthropicProvider {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Test connection to Anthropic API with minimal request
    pub async fn ping(&self, config: &LLMConfig, api_key: &str) -> Result<(), String> {
        let base_url = config.base_url.as_deref().unwrap_or("https://api.anthropic.com");
        let url = format!("{}/v1/messages", base_url);

        let body = json!({
            "model": config.model,
            "max_tokens": 1,
            "messages": [{"role": "user", "content": "hi"}]
        });

        let response = self
            .client
            .post(&url)
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("network_error: {}", e))?;

        let status = response.status();
        if status.is_success() {
            Ok(())
        } else if status.as_u16() == 401 {
            Err("unauthorized".to_string())
        } else if status.as_u16() == 429 {
            Err("rate_limited".to_string())
        } else {
            let body_text = response.text().await.unwrap_or_default();
            Err(format!("http_{}: {}", status.as_u16(), &body_text[..200.min(body_text.len())]))
        }
    }

    /// Invoke Anthropic API with streaming response
    pub async fn invoke_stream<F>(
        &self,
        config: &LLMConfig,
        api_key: &str,
        messages: &[Value],
        system: Option<&str>,
        on_delta: F,
    ) -> Result<(), String>
    where
        F: Fn(DeltaEvent),
    {
        let base_url = config.base_url.as_deref().unwrap_or("https://api.anthropic.com");
        let url = format!("{}/v1/messages", base_url);

        let mut body = json!({
            "model": config.model,
            "max_tokens": config.params.max_tokens,
            "stream": true,
            "messages": messages
        });

        if let Some(sys) = system {
            body["system"] = json!(sys);
        }

        let response = self
            .client
            .post(&url)
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .header("accept", "text/event-stream")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("network_error: {}", e))?;

        let status = response.status();
        if !status.is_success() {
            let body_text = response.text().await.unwrap_or_default();
            let recoverable = status.as_u16() >= 500 || status.as_u16() == 429;
            on_delta(DeltaEvent::Error {
                recoverable,
                code: format!("http_{}", status.as_u16()),
                message: body_text[..500.min(body_text.len())].to_string(),
            });
            return Ok(());
        }

        // Process SSE stream
        let mut stream = response.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(bytes) => {
                    buffer.push_str(&String::from_utf8_lossy(&bytes));

                    // Parse SSE events
                    while let Some(newline_pos) = buffer.find('\n') {
                        let line = buffer[..newline_pos].trim().to_string();
                        buffer = buffer[newline_pos + 1..].to_string();

                        if line.is_empty() || line.starts_with(':') {
                            continue;
                        }

                        if let Some(data) = line.strip_prefix("data: ") {
                            if let Some(event) = Self::parse_sse_event(data) {
                                on_delta(event);
                            }
                        }
                    }
                }
                Err(e) => {
                    on_delta(DeltaEvent::Error {
                        recoverable: true,
                        code: "stream_error".to_string(),
                        message: e.to_string(),
                    });
                    break;
                }
            }
        }

        Ok(())
    }

    /// Parse SSE event data into DeltaEvent
    fn parse_sse_event(data: &str) -> Option<DeltaEvent> {
        let json: Value = serde_json::from_str(data).ok()?;
        let event_type = json.get("type")?.as_str()?;

        match event_type {
            "content_block_delta" => {
                let delta = json.get("delta")?;
                let delta_type = delta.get("type")?.as_str()?;

                match delta_type {
                    "text_delta" => {
                        let text = delta.get("text")?.as_str()?;
                        Some(DeltaEvent::Text { delta: text.to_string() })
                    }
                    "thinking_delta" => {
                        let thinking = delta.get("thinking")?.as_str()?;
                        Some(DeltaEvent::Thinking { delta: thinking.to_string() })
                    }
                    "input_json_delta" => {
                        let partial_json = delta.get("partial_json")?.as_str().unwrap_or("");
                        let id = json.get("index")?.as_u64().unwrap_or(0).to_string();
                        Some(DeltaEvent::ToolUseDelta {
                            id,
                            args_delta: partial_json.to_string(),
                        })
                    }
                    _ => None,
                }
            }
            "content_block_start" => {
                let block = json.get("content_block")?;
                let block_type = block.get("type")?.as_str()?;

                if block_type == "tool_use" {
                    let id = block.get("id")?.as_str()?.to_string();
                    let name = block.get("name")?.as_str()?.to_string();
                    Some(DeltaEvent::ToolUseStart { id, name })
                } else {
                    None
                }
            }
            "content_block_stop" => {
                let id = json.get("index")?.as_u64().unwrap_or(0).to_string();
                Some(DeltaEvent::ToolUseEnd { id })
            }
            "message_delta" => {
                let usage = json.get("usage")?;
                let output_tokens = usage.get("output_tokens")?.as_u64();
                Some(DeltaEvent::Usage {
                    input_tokens: None,
                    output_tokens,
                    cached: None,
                })
            }
            "message_start" => {
                let message = json.get("message")?;
                let usage = message.get("usage")?;
                let input_tokens = usage.get("input_tokens")?.as_u64();
                let cached = usage.get("cache_read_input_tokens").and_then(|v| v.as_u64());
                Some(DeltaEvent::Usage {
                    input_tokens,
                    output_tokens: None,
                    cached,
                })
            }
            "message_stop" => {
                Some(DeltaEvent::Stop { reason: "end_turn".to_string() })
            }
            "error" => {
                let error = json.get("error")?;
                let error_type = error.get("type")?.as_str().unwrap_or("unknown");
                let message = error.get("message")?.as_str().unwrap_or("unknown error");
                let recoverable = error_type.contains("overloaded") || error_type.contains("rate");
                Some(DeltaEvent::Error {
                    recoverable,
                    code: error_type.to_string(),
                    message: message.to_string(),
                })
            }
            _ => None,
        }
    }

    /// Estimate cost for a request (simplified for M1)
    pub fn estimate_cost(&self, config: &LLMConfig, _messages: &[Value]) -> f64 {
        // Simplified cost estimation based on max_tokens
        // Real implementation would count input tokens
        match config.model.as_str() {
            "claude-3-5-sonnet-20241022" => config.params.max_tokens as f64 * 0.000015,
            "claude-3-opus-20240229" => config.params.max_tokens as f64 * 0.000075,
            "claude-3-sonnet-20240229" => config.params.max_tokens as f64 * 0.000015,
            "claude-3-haiku-20240307" => config.params.max_tokens as f64 * 0.000004,
            _ => 0.0, // Unknown model
        }
    }
}

/// Delta events emitted during streaming
#[derive(Debug, Clone)]
pub enum DeltaEvent {
    Text { delta: String },
    Thinking { delta: String },
    ToolUseStart { id: String, name: String },
    ToolUseDelta { id: String, args_delta: String },
    ToolUseEnd { id: String },
    Usage {
        input_tokens: Option<u64>,
        output_tokens: Option<u64>,
        cached: Option<u64>,
    },
    Stop { reason: String },
    Error {
        recoverable: bool,
        code: String,
        message: String,
    },
}

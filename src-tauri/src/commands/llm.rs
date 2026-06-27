//! LLM 子系统 Tauri 命令
//!
//! 暴露给 Renderer 的 IPC 命令，负责：
//! - 配置读写（TOML `[llm.<role>]` 段）
//! - Secret 存取（OS Keyring + fallback）
//!
//! 详见 PRD §6.2 / ARCH §2.2 / design.md D1-D6。

use serde::{Deserialize, Serialize};
use tauri::{command, Emitter};

use crate::infrastructure::llm::{config, secret_store};

// ============================================================================
// Data Structures
// ============================================================================

/// LLM 配置（Tauri 命令返回 / 接收的形状）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfigPayload {
    pub role: String,
    pub provider: String,
    pub model: String,
    pub base_url: Option<String>,
    pub secret_ref: String,
    pub temperature: f64,
    pub max_tokens: u64,
}

impl From<config::LLMConfig> for LLMConfigPayload {
    fn from(cfg: config::LLMConfig) -> Self {
        Self {
            role: cfg.role,
            provider: cfg.provider,
            model: cfg.model,
            base_url: cfg.base_url,
            secret_ref: cfg.secret_ref,
            temperature: cfg.params.temperature,
            max_tokens: cfg.params.max_tokens,
        }
    }
}

impl From<LLMConfigPayload> for config::LLMConfig {
    fn from(p: LLMConfigPayload) -> Self {
        Self {
            role: p.role,
            provider: p.provider,
            model: p.model,
            base_url: p.base_url,
            secret_ref: p.secret_ref,
            params: config::LLMParams {
                temperature: p.temperature,
                max_tokens: p.max_tokens,
            },
        }
    }
}

/// Secret 操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretRefPayload {
    pub secret_ref: String,
}

/// Secret 值（仅内部传递，不会 emit 到 Renderer）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretPayload {
    pub plaintext: String,
}

/// 测试连接结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConnectionPayload {
    pub ok: bool,
    pub reason: Option<String>,
}

/// 通用错误响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPayload {
    pub error: String,
    pub message: String,
}

// ============================================================================
// Commands: Config
// ============================================================================

/// 加载指定 role 的 LLM 配置
#[command]
pub fn llm_load_config(role: String) -> Result<LLMConfigPayload, ErrorPayload> {
    config::load_llm_config(&role)
        .map(LLMConfigPayload::from)
        .map_err(|e| match e {
            config::ConfigError::FileNotFound => ErrorPayload {
                error: "file_not_found".to_string(),
                message: "config.toml not found".to_string(),
            },
            config::ConfigError::ParseError(msg) => ErrorPayload {
                error: "parse_error".to_string(),
                message: msg,
            },
            config::ConfigError::RoleNotFound(role) => ErrorPayload {
                error: "role_not_found".to_string(),
                message: format!("role '{}' not configured", role),
            },
            config::ConfigError::Io(err) => ErrorPayload {
                error: "io_error".to_string(),
                message: err.to_string(),
            },
            config::ConfigError::SerializeError(msg) => ErrorPayload {
                error: "serialize_error".to_string(),
                message: msg,
            },
        })
}

/// 保存指定 role 的 LLM 配置
#[command]
pub fn llm_save_config(role: String, cfg: LLMConfigPayload) -> Result<(), ErrorPayload> {
    config::save_llm_config(&role, &cfg.into()).map_err(|e| match e {
        config::ConfigError::FileNotFound => ErrorPayload {
            error: "file_not_found".to_string(),
            message: "config.toml not found".to_string(),
        },
        config::ConfigError::ParseError(msg) => ErrorPayload {
            error: "parse_error".to_string(),
            message: msg,
        },
        config::ConfigError::RoleNotFound(role) => ErrorPayload {
            error: "role_not_found".to_string(),
            message: format!("role '{}' not configured", role),
        },
        config::ConfigError::Io(err) => ErrorPayload {
            error: "io_error".to_string(),
            message: err.to_string(),
        },
        config::ConfigError::SerializeError(msg) => ErrorPayload {
            error: "serialize_error".to_string(),
            message: msg,
        },
    })
}

// ============================================================================
// Commands: Secret
// ============================================================================

/// 保存 API key 明文，返回 secret_ref
#[command]
pub fn llm_save_secret(role: String, plaintext: String) -> Result<SecretRefPayload, ErrorPayload> {
    secret_store::SecretStore::save_secret(&role, &plaintext)
        .map(|secret_ref| SecretRefPayload { secret_ref })
        .map_err(|e| ErrorPayload {
            error: "secret_error".to_string(),
            message: e.to_string(),
        })
}

/// 通过 secret_ref 取回明文 API key
#[command]
pub fn llm_get_secret(secret_ref: String) -> Result<SecretPayload, ErrorPayload> {
    secret_store::SecretStore::get_secret(&secret_ref)
        .map(|plaintext| SecretPayload { plaintext })
        .map_err(|e| match e {
            secret_store::SecretError::NotFound => ErrorPayload {
                error: "secret_not_found".to_string(),
                message: "secret_ref not found in keyring or file fallback".to_string(),
            },
            other => ErrorPayload {
                error: "secret_error".to_string(),
                message: other.to_string(),
            },
        })
}

/// 删除 secret_ref 对应的条目
#[command]
pub fn llm_delete_secret(secret_ref: String) -> Result<(), ErrorPayload> {
    secret_store::SecretStore::delete_secret(&secret_ref).map_err(|e| match e {
        secret_store::SecretError::NotFound => ErrorPayload {
            error: "secret_not_found".to_string(),
            message: "secret_ref not found".to_string(),
        },
        other => ErrorPayload {
            error: "secret_error".to_string(),
            message: other.to_string(),
        },
    })
}

// ============================================================================
// Commands: Test Connection
// ============================================================================

/// 测试指定 role 的 LLM 连接是否可用
///
/// 支持两种模式：
/// - 有 config 参数：用临时配置测试（不依赖已保存的文件）
/// - 无 config 参数：从已保存的配置加载
#[command]
pub async fn llm_test_connection(
    role: String,
    config: Option<LLMConfigPayload>,
    api_key: Option<String>,
) -> Result<TestConnectionPayload, ErrorPayload> {
    // 1. 加载配置：优先用传入的 config，否则从文件加载
    let cfg: config::LLMConfig = match config {
        Some(payload) => payload.into(),
        None => config::load_llm_config(&role).map_err(|e| match e {
            config::ConfigError::FileNotFound => ErrorPayload {
                error: "file_not_found".to_string(),
                message: "config.toml not found".to_string(),
            },
            config::ConfigError::RoleNotFound(r) => ErrorPayload {
                error: "role_not_found".to_string(),
                message: format!("role '{}' not configured", r),
            },
            other => ErrorPayload {
                error: "config_error".to_string(),
                message: other.to_string(),
            },
        })?,
    };

    // 2. 取 API key：优先用传入的 api_key，否则从 keyring 加载
    let api_key = match api_key {
        Some(key) => key,
        None => secret_store::SecretStore::get_secret(&cfg.secret_ref).map_err(|e| match e {
            secret_store::SecretError::NotFound => ErrorPayload {
                error: "secret_not_found".to_string(),
                message: "API key not configured".to_string(),
            },
            other => ErrorPayload {
                error: "secret_error".to_string(),
                message: other.to_string(),
            },
        })?,
    };

    // 3. 根据 provider 类型做 ping
    match cfg.provider.as_str() {
        "anthropic" => anthropic_ping(&cfg, &api_key).await,
        "openai" => openai_ping(&cfg, &api_key).await,
        other => Ok(TestConnectionPayload {
            ok: false,
            reason: Some(format!("unsupported provider: {}", other)),
        }),
    }
}

/// Anthropic 极小请求 ping
async fn anthropic_ping(
    cfg: &config::LLMConfig,
    api_key: &str,
) -> Result<TestConnectionPayload, ErrorPayload> {
    let base_url = cfg
        .base_url
        .as_deref()
        .unwrap_or("https://api.anthropic.com");
    let url = format!("{}/v1/messages", base_url);

    let body = serde_json::json!({
        "model": cfg.model,
        "max_tokens": 1,
        "messages": [{"role": "user", "content": "hi"}]
    });

    let client = reqwest::Client::new();
    let resp = client
        .post(&url)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await;

    match resp {
        Ok(r) => {
            let status = r.status().as_u16();
            if status >= 200 && status < 300 {
                Ok(TestConnectionPayload {
                    ok: true,
                    reason: None,
                })
            } else if status == 401 || status == 403 {
                Ok(TestConnectionPayload {
                    ok: false,
                    reason: Some("unauthorized".to_string()),
                })
            } else if status == 405 {
                // 405 = server reachable, method not allowed (e.g. Coding Plan key)
                // Still counts as a successful connection verification
                Ok(TestConnectionPayload {
                    ok: true,
                    reason: Some("连接成功（服务端返回 405，API 密钥有效，但当前端点可能不支持此密钥类型）".to_string()),
                })
            } else if status == 429 {
                Ok(TestConnectionPayload {
                    ok: false,
                    reason: Some("rate_limited".to_string()),
                })
            } else {
                let body_text = r.text().await.unwrap_or_default();
                Ok(TestConnectionPayload {
                    ok: false,
                    reason: Some(format!("http_{}: {}", status, truncate(&body_text, 200))),
                })
            }
        }
        Err(e) => Ok(TestConnectionPayload {
            ok: false,
            reason: Some(format!("network_error: {}", e)),
        }),
    }
}

/// 截断字符串到指定长度（用于错误信息）
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}

/// OpenAI 兼容协议极小请求 ping
///
/// 使用 GET /v1/models 验证连接和 API key
async fn openai_ping(
    cfg: &config::LLMConfig,
    api_key: &str,
) -> Result<TestConnectionPayload, ErrorPayload> {
    let base_url = cfg
        .base_url
        .as_deref()
        .unwrap_or("https://api.openai.com");
    let url = format!("{}/v1/models", base_url);

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await;

    match resp {
        Ok(r) => {
            let status = r.status().as_u16();
            if status >= 200 && status < 300 {
                Ok(TestConnectionPayload {
                    ok: true,
                    reason: None,
                })
            } else if status == 401 || status == 403 {
                Ok(TestConnectionPayload {
                    ok: false,
                    reason: Some("unauthorized: API 密钥无效或过期".to_string()),
                })
            } else {
                let body_text = r.text().await.unwrap_or_default();
                Ok(TestConnectionPayload {
                    ok: false,
                    reason: Some(format!("http_{}: {}", status, truncate(&body_text, 200))),
                })
            }
        }
        Err(e) => Ok(TestConnectionPayload {
            ok: false,
            reason: Some(format!("network_error: {}", e)),
        }),
    }
}

// ============================================================================
// Commands: Invoke (streaming)
// ============================================================================

/// LLM 调用请求（Renderer → Rust）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMInvokeRequest {
    pub messages: Vec<LLMMessage>,
    pub system: Option<String>,
    pub stream: bool,
}

/// 单条消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMMessage {
    pub role: String,
    pub content: String,
}

/// Unified Delta（Rust → Renderer 事件）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum UnifiedDelta {
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

/// llm_delta 事件 payload（emit 给 Renderer）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMDeltaEvent {
    pub turn_id: String,
    pub delta: UnifiedDelta,
}

/// llm_invoke 命令的返回值（仅 turn_id，实际流通过事件）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMInvokeResponse {
    pub turn_id: String,
}

/// 调用 LLM 并流式返回 delta 事件
///
/// # Flow
/// 1. 加载 role 配置 + 取 API key
/// 2. 生成 turn_id
/// 3. 异步 spawn：向 provider 发起 streaming 请求
/// 4. 解析 SSE 事件，emit `llm_delta` 给当前窗口
/// 5. 流结束 emit `llm_done`
#[command]
pub async fn llm_invoke(
    app: tauri::AppHandle,
    role: String,
    request: LLMInvokeRequest,
) -> Result<LLMInvokeResponse, ErrorPayload> {
    // 1. 加载配置
    let cfg = config::load_llm_config(&role).map_err(|e| match e {
        config::ConfigError::FileNotFound => ErrorPayload {
            error: "file_not_found".to_string(),
            message: "config.toml not found".to_string(),
        },
        config::ConfigError::RoleNotFound(r) => ErrorPayload {
            error: "role_not_found".to_string(),
            message: format!("role '{}' not configured", r),
        },
        other => ErrorPayload {
            error: "config_error".to_string(),
            message: other.to_string(),
        },
    })?;

    // 2. 取 API key
    let api_key = secret_store::SecretStore::get_secret(&cfg.secret_ref).map_err(|e| match e {
        secret_store::SecretError::NotFound => ErrorPayload {
            error: "secret_not_found".to_string(),
            message: "API key not configured".to_string(),
        },
        other => ErrorPayload {
            error: "secret_error".to_string(),
            message: other.to_string(),
        },
    })?;

    // 3. 生成 turn_id
    let turn_id = uuid::Uuid::new_v4().to_string();
    let turn_id_clone = turn_id.clone();

    // 4. Spawn streaming task
    tauri::async_runtime::spawn(async move {
        match cfg.provider.as_str() {
            "anthropic" => {
                anthropic_stream(&app, &turn_id_clone, &cfg, &api_key, &request).await
            }
            "openai" => {
                openai_stream(&app, &turn_id_clone, &cfg, &api_key, &request).await
            }
            other => {
                let delta = UnifiedDelta::Error {
                    recoverable: false,
                    code: "unsupported_provider".to_string(),
                    message: format!("provider '{}' not implemented in M1", other),
                };
                emit_delta(&app, &turn_id_clone, delta);
                emit_done(&app, &turn_id_clone);
            }
        }
    });

    Ok(LLMInvokeResponse { turn_id })
}

/// Anthropic 流式请求实现
async fn anthropic_stream(
    app: &tauri::AppHandle,
    turn_id: &str,
    cfg: &config::LLMConfig,
    api_key: &str,
    request: &LLMInvokeRequest,
) {
    use futures_util::StreamExt;

    let base_url = cfg
        .base_url
        .as_deref()
        .unwrap_or("https://api.anthropic.com");
    let url = format!("{}/v1/messages", base_url);

    let body = serde_json::json!({
        "model": cfg.model,
        "max_tokens": cfg.params.max_tokens,
        "stream": true,
        "system": request.system,
        "messages": request.messages.iter().map(|m| serde_json::json!({
            "role": m.role,
            "content": m.content,
        })).collect::<Vec<_>>()
    });

    let client = reqwest::Client::new();
    let resp = match client
        .post(&url)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .header("accept", "text/event-stream")
        .json(&body)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            emit_delta(
                app,
                turn_id,
                UnifiedDelta::Error {
                    recoverable: true,
                    code: "network_error".to_string(),
                    message: e.to_string(),
                },
            );
            emit_done(app, turn_id);
            return;
        }
    };

    let status = resp.status().as_u16();
    if status >= 400 {
        let body_text = resp.text().await.unwrap_or_default();
        let recoverable = status >= 500 || status == 429;
        emit_delta(
            app,
            turn_id,
            UnifiedDelta::Error {
                recoverable,
                code: format!("http_{}", status),
                message: truncate(&body_text, 500),
            },
        );
        emit_done(app, turn_id);
        return;
    }

    // 解析 SSE 流
    let mut stream = resp.bytes_stream();
    let mut buffer = String::new();

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(bytes) => {
                buffer.push_str(&String::from_utf8_lossy(&bytes));
                // 按行解析 SSE
                while let Some(newline_pos) = buffer.find('\n') {
                    let line = buffer[..newline_pos].trim().to_string();
                    buffer = buffer[newline_pos + 1..].to_string();

                    if line.is_empty() || line.starts_with(':') {
                        continue; // 跳过空行和注释
                    }

                    if let Some(data) = line.strip_prefix("data: ") {
                        if let Some(delta) = parse_anthropic_event(data) {
                            emit_delta(app, turn_id, delta);
                        }
                    }
                }
            }
            Err(e) => {
                emit_delta(
                    app,
                    turn_id,
                    UnifiedDelta::Error {
                        recoverable: true,
                        code: "stream_error".to_string(),
                        message: e.to_string(),
                    },
                );
                break;
            }
        }
    }

    emit_done(app, turn_id);
}

/// 解析 Anthropic SSE 事件为 UnifiedDelta
fn parse_anthropic_event(data: &str) -> Option<UnifiedDelta> {
    let json: serde_json::Value = serde_json::from_str(data).ok()?;
    let event_type = json.get("type")?.as_str()?;

    match event_type {
        "content_block_delta" => {
            let delta = json.get("delta")?;
            let delta_type = delta.get("type")?.as_str()?;
            match delta_type {
                "text_delta" => {
                    let text = delta.get("text")?.as_str()?;
                    Some(UnifiedDelta::Text {
                        delta: text.to_string(),
                    })
                }
                "thinking_delta" => {
                    let thinking = delta.get("thinking")?.as_str()?;
                    Some(UnifiedDelta::Thinking {
                        delta: thinking.to_string(),
                    })
                }
                "input_json_delta" => {
                    let partial_json = delta.get("partial_json")?.as_str().unwrap_or("");
                    let id = json.get("index")?.as_u64().unwrap_or(0).to_string();
                    Some(UnifiedDelta::ToolUseDelta {
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
                Some(UnifiedDelta::ToolUseStart { id, name })
            } else {
                None
            }
        }
        "content_block_stop" => {
            let id = json.get("index")?.as_u64().unwrap_or(0).to_string();
            Some(UnifiedDelta::ToolUseEnd { id })
        }
        "message_delta" => {
            let usage = json.get("usage")?;
            let output_tokens = usage.get("output_tokens")?.as_u64();
            Some(UnifiedDelta::Usage {
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
            Some(UnifiedDelta::Usage {
                input_tokens,
                output_tokens: None,
                cached,
            })
        }
        "message_stop" => Some(UnifiedDelta::Stop {
            reason: "end_turn".to_string(),
        }),
        "error" => {
            let error = json.get("error")?;
            let err_type = error.get("type")?.as_str().unwrap_or("unknown");
            let message = error.get("message")?.as_str().unwrap_or("unknown error");
            let recoverable = err_type.contains("overloaded") || err_type.contains("rate");
            Some(UnifiedDelta::Error {
                recoverable,
                code: err_type.to_string(),
                message: message.to_string(),
            })
        }
        _ => None,
    }
}

/// OpenAI 兼容协议流式请求实现
async fn openai_stream(
    app: &tauri::AppHandle,
    turn_id: &str,
    cfg: &config::LLMConfig,
    api_key: &str,
    request: &LLMInvokeRequest,
) {
    use futures_util::StreamExt;

    let base_url = cfg.base_url.as_deref().unwrap_or("https://api.openai.com");
    let url = format!("{}/v1/chat/completions", base_url);

    // Build messages array, prepending system message if provided
    let mut messages: Vec<serde_json::Value> = Vec::new();
    if let Some(ref system) = request.system {
        messages.push(serde_json::json!({
            "role": "system",
            "content": system
        }));
    }
    for m in &request.messages {
        messages.push(serde_json::json!({
            "role": m.role,
            "content": m.content
        }));
    }

    let body = serde_json::json!({
        "model": cfg.model,
        "max_tokens": cfg.params.max_tokens,
        "temperature": cfg.params.temperature,
        "stream": true,
        "messages": messages
    });

    let client = reqwest::Client::new();
    let resp = match client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("content-type", "application/json")
        .header("accept", "text/event-stream")
        .json(&body)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            emit_delta(
                app,
                turn_id,
                UnifiedDelta::Error {
                    recoverable: true,
                    code: "network_error".to_string(),
                    message: e.to_string(),
                },
            );
            emit_done(app, turn_id);
            return;
        }
    };

    let status = resp.status().as_u16();
    if status >= 400 {
        let body_text = resp.text().await.unwrap_or_default();
        let recoverable = status >= 500 || status == 429;
        emit_delta(
            app,
            turn_id,
            UnifiedDelta::Error {
                recoverable,
                code: format!("http_{}", status),
                message: truncate(&body_text, 500),
            },
        );
        emit_done(app, turn_id);
        return;
    }

    // Parse SSE stream
    let mut stream = resp.bytes_stream();
    let mut buffer = String::new();

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(bytes) => {
                buffer.push_str(&String::from_utf8_lossy(&bytes));
                // Line-based SSE parsing
                while let Some(newline_pos) = buffer.find('\n') {
                    let line = buffer[..newline_pos].trim().to_string();
                    buffer = buffer[newline_pos + 1..].to_string();

                    if line.is_empty() || line.starts_with(':') {
                        continue;
                    }

                    if let Some(data) = line.strip_prefix("data: ") {
                        if data.trim() == "[DONE]" {
                            emit_delta(
                                app,
                                turn_id,
                                UnifiedDelta::Stop {
                                    reason: "end_turn".to_string(),
                                },
                            );
                            break;
                        }
                        if let Some(delta) = parse_openai_event(data) {
                            emit_delta(app, turn_id, delta);
                        }
                    }
                }
            }
            Err(e) => {
                emit_delta(
                    app,
                    turn_id,
                    UnifiedDelta::Error {
                        recoverable: true,
                        code: "stream_error".to_string(),
                        message: e.to_string(),
                    },
                );
                break;
            }
        }
    }

    emit_done(app, turn_id);
}

/// 解析 OpenAI 兼容 SSE 事件为 UnifiedDelta
///
/// OpenAI streaming format: each `data:` line contains a JSON object like:
/// ```json
/// {"id":"...","choices":[{"delta":{"content":"Hello"},"index":0}]}
/// ```
fn parse_openai_event(data: &str) -> Option<UnifiedDelta> {
    let json: serde_json::Value = serde_json::from_str(data).ok()?;
    let choices = json.get("choices")?.as_array()?;
    let first = choices.first()?;
    let delta = first.get("delta")?;

    // Check for tool_calls (function calling)
    if let Some(tool_calls) = delta.get("tool_calls").and_then(|v| v.as_array()) {
        for tc in tool_calls {
            if let Some(_index) = tc.get("index") {
                // If there's a function object with name, it's a new tool call
                if let Some(function) = tc.get("function") {
                    if let Some(name) = function.get("name").and_then(|v| v.as_str()) {
                        let id = tc
                            .get("id")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        return Some(UnifiedDelta::ToolUseStart {
                            id,
                            name: name.to_string(),
                        });
                    }
                    if let Some(args) = function.get("arguments").and_then(|v| v.as_str()) {
                        let index = tc
                            .get("index")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(0)
                            .to_string();
                        return Some(UnifiedDelta::ToolUseDelta {
                            id: index,
                            args_delta: args.to_string(),
                        });
                    }
                }
            }
        }
        return None;
    }

    // Text content delta
    if let Some(content) = delta.get("content").and_then(|v| v.as_str()) {
        if !content.is_empty() {
            return Some(UnifiedDelta::Text {
                delta: content.to_string(),
            });
        }
    }

    // Check for finish_reason
    if let Some(reason) = first.get("finish_reason").and_then(|v| v.as_str()) {
        return Some(UnifiedDelta::Stop {
            reason: reason.to_string(),
        });
    }

    // Usage info (if present in the last chunk with stream_options)
    if let Some(usage) = json.get("usage") {
        let prompt_tokens = usage.get("prompt_tokens").and_then(|v| v.as_u64());
        let completion_tokens = usage.get("completion_tokens").and_then(|v| v.as_u64());
        if prompt_tokens.is_some() || completion_tokens.is_some() {
            return Some(UnifiedDelta::Usage {
                input_tokens: prompt_tokens,
                output_tokens: completion_tokens,
                cached: None,
            });
        }
    }

    None
}

/// Emit delta 事件给 Renderer
fn emit_delta(app: &tauri::AppHandle, turn_id: &str, delta: UnifiedDelta) {
    let event = LLMDeltaEvent {
        turn_id: turn_id.to_string(),
        delta,
    };
    let _ = app.emit("llm_delta", event);
}

/// Emit done 事件给 Renderer
fn emit_done(app: &tauri::AppHandle, turn_id: &str) {
    let _ = app.emit("llm_done", serde_json::json!({ "turn_id": turn_id }));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试：llm_get_secret 缺失时返回 secret_not_found 错误
    #[test]
    fn test_llm_get_secret_not_found() {
        let result = llm_get_secret("non-existent-uuid".to_string());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.error, "secret_not_found");
    }

    /// 测试：parse_anthropic_event 解析 text_delta
    #[test]
    fn test_parse_anthropic_text_delta() {
        let data = r#"{
            "type": "content_block_delta",
            "index": 0,
            "delta": {
                "type": "text_delta",
                "text": "Hello"
            }
        }"#;
        let delta = parse_anthropic_event(data);
        match delta {
            Some(UnifiedDelta::Text { delta }) => assert_eq!(delta, "Hello"),
            other => panic!("Expected Text delta, got {:?}", other),
        }
    }

    /// 测试：parse_anthropic_event 解析 message_stop
    #[test]
    fn test_parse_anthropic_message_stop() {
        let data = r#"{"type": "message_stop"}"#;
        let delta = parse_anthropic_event(data);
        match delta {
            Some(UnifiedDelta::Stop { reason }) => assert_eq!(reason, "end_turn"),
            other => panic!("Expected Stop delta, got {:?}", other),
        }
    }

    /// 测试：parse_anthropic_event 解析 error
    #[test]
    fn test_parse_anthropic_error() {
        let data = r#"{
            "type": "error",
            "error": {
                "type": "overloaded_error",
                "message": "Service overloaded"
            }
        }"#;
        let delta = parse_anthropic_event(data);
        match delta {
            Some(UnifiedDelta::Error {
                recoverable,
                code,
                message,
            }) => {
                assert!(recoverable);
                assert_eq!(code, "overloaded_error");
                assert_eq!(message, "Service overloaded");
            }
            other => panic!("Expected Error delta, got {:?}", other),
        }
    }

    /// 测试：LLMConfigPayload 与 LLMConfig 互转
    #[test]
    fn test_config_payload_conversion() {
        let cfg = config::LLMConfig {
            role: "test".to_string(),
            provider: "anthropic".to_string(),
            model: "claude-fable-5".to_string(),
            base_url: None,
            secret_ref: "uuid-1".to_string(),
            params: config::LLMParams {
                temperature: 0.7,
                max_tokens: 4096,
            },
        };

        let payload = LLMConfigPayload::from(cfg.clone());
        assert_eq!(payload.role, "test");
        assert_eq!(payload.provider, "anthropic");
        assert_eq!(payload.model, "claude-fable-5");
        assert_eq!(payload.secret_ref, "uuid-1");
        assert!((payload.temperature - 0.7).abs() < f64::EPSILON);
        assert_eq!(payload.max_tokens, 4096);

        let back: config::LLMConfig = payload.into();
        assert_eq!(back.role, cfg.role);
        assert_eq!(back.provider, cfg.provider);
        assert_eq!(back.model, cfg.model);
        assert_eq!(back.secret_ref, cfg.secret_ref);
    }

    /// 测试：truncate 函数
    #[test]
    fn test_truncate() {
        assert_eq!(truncate("hello", 10), "hello");
        assert_eq!(truncate("hello world", 5), "hello...");
        assert_eq!(truncate("", 10), "");
    }

    /// 测试：parse_openai_event 解析 text delta
    #[test]
    fn test_parse_openai_text_delta() {
        let data = r#"{
            "id": "chatcmpl-123",
            "choices": [{
                "delta": {"content": "Hello"},
                "index": 0
            }]
        }"#;
        let delta = parse_openai_event(data);
        match delta {
            Some(UnifiedDelta::Text { delta }) => assert_eq!(delta, "Hello"),
            other => panic!("Expected Text delta, got {:?}", other),
        }
    }

    /// 测试：parse_openai_event 解析 finish_reason (stop)
    #[test]
    fn test_parse_openai_finish_stop() {
        let data = r#"{
            "id": "chatcmpl-123",
            "choices": [{
                "delta": {},
                "finish_reason": "stop",
                "index": 0
            }]
        }"#;
        let delta = parse_openai_event(data);
        match delta {
            Some(UnifiedDelta::Stop { reason }) => assert_eq!(reason, "stop"),
            other => panic!("Expected Stop delta, got {:?}", other),
        }
    }

    /// 测试：parse_openai_event 解析 tool call start
    #[test]
    fn test_parse_openai_tool_use_start() {
        let data = r#"{
            "id": "chatcmpl-123",
            "choices": [{
                "delta": {
                    "tool_calls": [{
                        "index": 0,
                        "id": "call_abc123",
                        "function": {"name": "get_weather", "arguments": ""}
                    }]
                },
                "index": 0
            }]
        }"#;
        let delta = parse_openai_event(data);
        match delta {
            Some(UnifiedDelta::ToolUseStart { id, name }) => {
                assert_eq!(id, "call_abc123");
                assert_eq!(name, "get_weather");
            }
            other => panic!("Expected ToolUseStart delta, got {:?}", other),
        }
    }

    /// 测试：parse_openai_event 解析 tool call arguments delta
    #[test]
    fn test_parse_openai_tool_use_delta() {
        let data = r#"{
            "id": "chatcmpl-123",
            "choices": [{
                "delta": {
                    "tool_calls": [{
                        "index": 0,
                        "function": {"arguments": "{\"location\":"}
                    }]
                },
                "index": 0
            }]
        }"#;
        let delta = parse_openai_event(data);
        match delta {
            Some(UnifiedDelta::ToolUseDelta { id, args_delta }) => {
                assert_eq!(id, "0");
                assert_eq!(args_delta, "{\"location\":");
            }
            other => panic!("Expected ToolUseDelta delta, got {:?}", other),
        }
    }

    /// 测试：parse_openai_event 解析 usage
    #[test]
    fn test_parse_openai_usage() {
        let data = r#"{
            "id": "chatcmpl-123",
            "choices": [{
                "delta": {},
                "index": 0
            }],
            "usage": {
                "prompt_tokens": 100,
                "completion_tokens": 50
            }
        }"#;
        let delta = parse_openai_event(data);
        match delta {
            Some(UnifiedDelta::Usage {
                input_tokens,
                output_tokens,
                cached,
            }) => {
                assert_eq!(input_tokens, Some(100));
                assert_eq!(output_tokens, Some(50));
                assert_eq!(cached, None);
            }
            other => panic!("Expected Usage delta, got {:?}", other),
        }
    }
}

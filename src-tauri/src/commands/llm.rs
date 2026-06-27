//! LLM 子系统 Tauri 命令
//!
//! 暴露给 Renderer 的 IPC 命令，负责：
//! - 配置读写（TOML `[llm.<role>]` 段）
//! - Secret 存取（OS Keyring + fallback）
//!
//! 详见 PRD §6.2 / ARCH §2.2 / design.md D1-D6。

use serde::{Deserialize, Serialize};
use tauri::command;

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
/// 内部流程：
/// 1. 加载 role 配置
/// 2. 取 API key
/// 3. 向 provider 发起极小请求（max_tokens=1）验证连通性 / 鉴权
#[command]
pub async fn llm_test_connection(role: String) -> Result<TestConnectionPayload, ErrorPayload> {
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

    // 3. 根据 provider 类型做 ping
    match cfg.provider.as_str() {
        "anthropic" => anthropic_ping(&cfg, &api_key).await,
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

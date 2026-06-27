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

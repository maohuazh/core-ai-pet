//! LLM Secret 存储模块
//!
//! 职责：
//! - 通过 OS Keyring 存取 API key 明文（明文不出 Rust 进程）
//! - Keyring 不可用时（例如无守护进程的 Linux 桌面）回落到
//!   `~/.core-ai-pet/.secrets/<secret_ref>` 文件 + DPAPI（Windows）
//!   或 mode 0600（Unix）
//!
//! 详见 PRD §6.3 / ARCH §5.3（ADR-006）。
//!
//! 当前为骨架，具体实现由 task 1.3 / 1.4 / 1.7 完成。

use serde::{Deserialize, Serialize};

/// Secret 存储抽象：统一 Keyring / DPAPI fallback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretRef {
    /// 全局唯一引用键（UUID v4），存入 config.toml 的 secret_ref 字段
    pub id: String,
    /// 所属 role（用于调试日志 / 审计）
    pub role: String,
}

/// Secret 存取错误
#[derive(Debug)]
pub enum SecretError {
    /// Keyring 调用失败（已自动降级到文件）
    KeyringFailed(String),
    /// 找不到对应 secret_ref
    NotFound,
    /// 文件 I/O 错误
    Io(std::io::Error),
}

impl std::fmt::Display for SecretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KeyringFailed(msg) => write!(f, "keyring failed: {}", msg),
            Self::NotFound => write!(f, "secret not found"),
            Self::Io(err) => write!(f, "io error: {}", err),
        }
    }
}

impl std::error::Error for SecretError {}

impl From<std::io::Error> for SecretError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

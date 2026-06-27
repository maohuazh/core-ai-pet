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
//! 本模块完成 task 1.3 的 Keyring 主体；task 1.4 完成 DPAPI fallback。

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Keyring service 名称（写入 OS Keychain 的标识）
const KEYRING_SERVICE: &str = "coreai-llm";

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
    /// Keyring 调用失败（可能已自动降级到文件）
    KeyringFailed(String),
    /// 找不到对应 secret_ref
    NotFound,
    /// 文件 I/O 错误
    Io(std::io::Error),
    /// 内部错误（例如 UUID 生成失败）
    Internal(String),
}

impl std::fmt::Display for SecretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KeyringFailed(msg) => write!(f, "keyring failed: {}", msg),
            Self::NotFound => write!(f, "secret not found"),
            Self::Io(err) => write!(f, "io error: {}", err),
            Self::Internal(msg) => write!(f, "internal error: {}", msg),
        }
    }
}

impl std::error::Error for SecretError {}

impl From<std::io::Error> for SecretError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

/// Secret 存储主入口
///
/// 当前实现直接走 Keyring（task 1.3）；
/// task 1.4 将引入 DPAPI fallback 逻辑（Keyring 失败时自动回落）。
pub struct SecretStore;

impl SecretStore {
    /// 保存明文 API key，返回 secret_ref（UUID v4）
    ///
    /// # Arguments
    /// * `role` - 所属 role（用于日志/审计，但不影响存储路径）
    /// * `plaintext` - 明文 API key
    ///
    /// # Returns
    /// 生成的 secret_ref ID（UUID v4）
    ///
    /// # Errors
    /// - `SecretError::KeyringFailed` - Keyring 写入失败（task 1.4 后会回落文件）
    /// - `SecretError::Internal` - UUID 生成失败（极小概率）
    pub fn save_secret(role: &str, plaintext: &str) -> Result<String, SecretError> {
        let secret_ref = Uuid::new_v4().to_string();

        let entry = keyring::Entry::new(KEYRING_SERVICE, &secret_ref)
            .map_err(|e| SecretError::KeyringFailed(e.to_string()))?;

        entry
            .set_password(plaintext)
            .map_err(|e| SecretError::KeyringFailed(e.to_string()))?;

        log::info!(
            "Saved LLM secret for role={} -> secret_ref={}",
            role,
            secret_ref
        );

        Ok(secret_ref)
    }

    /// 通过 secret_ref 取回明文 API key
    ///
    /// # Errors
    /// - `SecretError::NotFound` - secret_ref 在 Keyring 中不存在
    /// - `SecretError::KeyringFailed` - Keyring 读取失败
    pub fn get_secret(secret_ref: &str) -> Result<String, SecretError> {
        let entry = keyring::Entry::new(KEYRING_SERVICE, secret_ref)
            .map_err(|e| SecretError::KeyringFailed(e.to_string()))?;

        match entry.get_password() {
            Ok(plaintext) => Ok(plaintext),
            Err(keyring::Error::NoEntry) => Err(SecretError::NotFound),
            Err(e) => Err(SecretError::KeyringFailed(e.to_string())),
        }
    }

    /// 删除 secret_ref 对应的条目
    ///
    /// # Errors
    /// - `SecretError::NotFound` - secret_ref 不存在
    /// - `SecretError::KeyringFailed` - Keyring 删除失败
    pub fn delete_secret(secret_ref: &str) -> Result<(), SecretError> {
        let entry = keyring::Entry::new(KEYRING_SERVICE, secret_ref)
            .map_err(|e| SecretError::KeyringFailed(e.to_string()))?;

        match entry.delete_credential() {
            Ok(()) => {
                log::info!("Deleted LLM secret: secret_ref={}", secret_ref);
                Ok(())
            }
            Err(keyring::Error::NoEntry) => Err(SecretError::NotFound),
            Err(e) => Err(SecretError::KeyringFailed(e.to_string())),
        }
    }
}

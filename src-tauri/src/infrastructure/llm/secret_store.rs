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
//! 本模块完成：
//! - task 1.3：Keyring 主体
//! - task 1.4：DPAPI fallback（Unix: mode 0600；Windows: 当前仅文件权限，DPAPI 待后续增强）

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Keyring service 名称（写入 OS Keychain 的标识）
const KEYRING_SERVICE: &str = "coreai-llm";

/// Fallback 文件存放子目录（相对于 ~/.core-ai-pet/）
const SECRETS_SUBDIR: &str = ".secrets";

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
/// 优先走 Keyring；失败时自动回落到文件存储（`~/.core-ai-pet/.secrets/<secret_ref>`）：
/// - Unix: 文件 mode 0600
/// - Windows: 当前仅 std::fs（owner-only 权限需 ACL 配置，待后续增强）
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
    /// - `SecretError::KeyringFailed` + `SecretError::Io` - Keyring 与文件都失败
    /// - `SecretError::Internal` - UUID 生成失败（极小概率）
    pub fn save_secret(role: &str, plaintext: &str) -> Result<String, SecretError> {
        let secret_ref = Uuid::new_v4().to_string();

        // 优先 Keyring
        match Self::keyring_save(&secret_ref, plaintext) {
            Ok(()) => {
                log::info!(
                    "Saved LLM secret for role={} -> secret_ref={} (keyring)",
                    role,
                    secret_ref
                );
                return Ok(secret_ref);
            }
            Err(e) => {
                log::warn!(
                    "Keyring unavailable for role={}, falling back to file storage: {}",
                    role,
                    e
                );
            }
        }

        // Fallback 到文件
        Self::file_save(&secret_ref, plaintext)?;
        log::warn!(
            "Saved LLM secret for role={} -> secret_ref={} (file fallback)",
            role,
            secret_ref
        );

        Ok(secret_ref)
    }

    /// 通过 secret_ref 取回明文 API key
    ///
    /// # Errors
    /// - `SecretError::NotFound` - secret_ref 在 Keyring 与文件中都不存在
    /// - `SecretError::KeyringFailed` / `SecretError::Io` - 底层读取失败
    pub fn get_secret(secret_ref: &str) -> Result<String, SecretError> {
        // 优先 Keyring
        match Self::keyring_get(secret_ref) {
            Ok(plaintext) => Ok(plaintext),
            Err(SecretError::NotFound) => {
                // Keyring 中没有，尝试文件 fallback
                Self::file_get(secret_ref)
            }
            Err(e) => {
                // Keyring 故障，尝试文件 fallback
                log::warn!(
                    "Keyring get failed for secret_ref={}, trying file fallback: {}",
                    secret_ref,
                    e
                );
                match Self::file_get(secret_ref) {
                    Ok(plaintext) => Ok(plaintext),
                    Err(SecretError::NotFound) => Err(SecretError::NotFound),
                    Err(io_err) => Err(io_err),
                }
            }
        }
    }

    /// 删除 secret_ref 对应的条目（Keyring + 文件 任一存在即删除）
    ///
    /// # Errors
    /// - `SecretError::NotFound` - 两者都不存在
    pub fn delete_secret(secret_ref: &str) -> Result<(), SecretError> {
        let keyring_result = match Self::keyring_delete(secret_ref) {
            Ok(()) => {
                log::info!("Deleted LLM secret from keyring: secret_ref={}", secret_ref);
                true
            }
            Err(SecretError::NotFound) => false,
            Err(e) => {
                log::warn!(
                    "Keyring delete failed for secret_ref={}: {}",
                    secret_ref,
                    e
                );
                false
            }
        };

        let file_result = match Self::file_delete(secret_ref) {
            Ok(()) => {
                log::info!("Deleted LLM secret from file: secret_ref={}", secret_ref);
                true
            }
            Err(SecretError::NotFound) => false,
            Err(e) => {
                log::warn!(
                    "File delete failed for secret_ref={}: {}",
                    secret_ref,
                    e
                );
                false
            }
        };

        if keyring_result || file_result {
            Ok(())
        } else {
            Err(SecretError::NotFound)
        }
    }

    // ========================================================================
    // Keyring 内部方法
    // ========================================================================

    fn keyring_save(secret_ref: &str, plaintext: &str) -> Result<(), SecretError> {
        let entry = keyring::Entry::new(KEYRING_SERVICE, secret_ref)
            .map_err(|e| SecretError::KeyringFailed(e.to_string()))?;
        entry
            .set_password(plaintext)
            .map_err(|e| SecretError::KeyringFailed(e.to_string()))?;
        Ok(())
    }

    fn keyring_get(secret_ref: &str) -> Result<String, SecretError> {
        let entry = keyring::Entry::new(KEYRING_SERVICE, secret_ref)
            .map_err(|e| SecretError::KeyringFailed(e.to_string()))?;
        match entry.get_password() {
            Ok(plaintext) => Ok(plaintext),
            Err(keyring::Error::NoEntry) => Err(SecretError::NotFound),
            Err(e) => Err(SecretError::KeyringFailed(e.to_string())),
        }
    }

    fn keyring_delete(secret_ref: &str) -> Result<(), SecretError> {
        let entry = keyring::Entry::new(KEYRING_SERVICE, secret_ref)
            .map_err(|e| SecretError::KeyringFailed(e.to_string()))?;
        match entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Err(SecretError::NotFound),
            Err(e) => Err(SecretError::KeyringFailed(e.to_string())),
        }
    }

    // ========================================================================
    // Fallback 文件方法
    // ========================================================================

    /// `~/.core-ai-pet/.secrets/` 目录路径
    fn secrets_dir() -> Result<PathBuf, SecretError> {
        let home = std::env::var_os("USERPROFILE")
            .or_else(|| std::env::var_os("HOME"))
            .ok_or_else(|| {
                SecretError::Internal("cannot determine home directory".to_string())
            })?;
        let base = PathBuf::from(home).join(".core-ai-pet").join(SECRETS_SUBDIR);
        Ok(base)
    }

    fn secret_path(secret_ref: &str) -> Result<PathBuf, SecretError> {
        Ok(Self::secrets_dir()?.join(secret_ref))
    }

    fn file_save(secret_ref: &str, plaintext: &str) -> Result<(), SecretError> {
        let path = Self::secret_path(secret_ref)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // 写入文件
        std::fs::write(&path, plaintext.as_bytes())?;

        // 设置权限
        Self::apply_restricted_permissions(&path)?;

        Ok(())
    }

    fn file_get(secret_ref: &str) -> Result<String, SecretError> {
        let path = Self::secret_path(secret_ref)?;
        if !path.exists() {
            return Err(SecretError::NotFound);
        }
        let bytes = std::fs::read(&path)?;
        String::from_utf8(bytes)
            .map_err(|e| SecretError::Internal(format!("invalid UTF-8 in secret file: {}", e)))
    }

    fn file_delete(secret_ref: &str) -> Result<(), SecretError> {
        let path = Self::secret_path(secret_ref)?;
        if !path.exists() {
            return Err(SecretError::NotFound);
        }
        std::fs::remove_file(&path)?;
        Ok(())
    }

    /// 设置受限权限：
    /// - Unix: mode 0600（仅 owner 可读写）
    /// - Windows: 当前仅依赖默认权限（owner-only ACL 待后续增强）
    fn apply_restricted_permissions(path: &std::path::Path) -> Result<(), SecretError> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(0o600);
            std::fs::set_permissions(path, perms)?;
        }

        #[cfg(windows)]
        {
            // TODO(M2): 通过 windows-sys SetNamedSecurityInfo 设置 owner-only ACL
            // 当前 Windows 仅依赖文件系统默认 ACL（通常为 owner 独占）
            // 安全审计时需关注此 gap
            let _ = path;
        }

        Ok(())
    }
}

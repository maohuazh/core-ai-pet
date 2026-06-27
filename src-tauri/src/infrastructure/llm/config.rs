//! LLM 配置模块
//!
//! 职责：
//! - 读取 `~/.core-ai-pet/config.toml` 的 `[llm.<role>]` 段
//! - 将 TOML 解析为 `LLMConfig` 结构体
//! - 将 UI 修改回写 TOML（原子写，不破坏其他段）
//!
//! 详见 PRD §6.1 / ARCH §3.8（ADR-003）/ design.md D2。
//!
//! TOML 段结构：
//! ```toml
//! [llm.chat_assistant]
//! provider = "anthropic"
//! model = "claude-fable-5"
//! secret_ref = "k-uuid-1"
//! # base_url 可选
//!
//! [llm.chat_assistant.params]
//! temperature = 0.7
//! max_tokens = 4096
//! ```
//!
//! 注意：`role` 字段由 section 名称注入（`chat_assistant`），不在 TOML 中显式出现。

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

/// 配置文件名
const CONFIG_FILENAME: &str = "config.toml";

/// 配置错误类型
#[derive(Debug)]
pub enum ConfigError {
    /// 配置文件不存在（首次启动允许）
    FileNotFound,
    /// TOML 解析失败
    ParseError(String),
    /// 指定 role 在配置中不存在
    RoleNotFound(String),
    /// 文件 I/O 错误
    Io(std::io::Error),
    /// TOML 序列化错误（回写时）
    SerializeError(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileNotFound => write!(f, "config file not found"),
            Self::ParseError(msg) => write!(f, "parse error: {}", msg),
            Self::RoleNotFound(role) => write!(f, "role not found in config: {}", role),
            Self::Io(err) => write!(f, "io error: {}", err),
            Self::SerializeError(msg) => write!(f, "serialize error: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

/// LLM 槽位配置（一个 role 的全部设定）
///
/// 注意：`role` 字段由 TOML section 名注入，不在文件中显式出现。
/// `serde(skip)` 确保序列化时不包含 role。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    /// 槽位身份（role 名称）— 由 section 名注入，不序列化
    #[serde(skip)]
    pub role: String,
    /// Provider 厂商 id，如 `anthropic` / `openai` / `azure-openai`
    pub provider: String,
    /// Model 名称，如 `claude-fable-5` / `gpt-4o`
    pub model: String,
    /// 可选 base_url（自部署或代理网关）
    pub base_url: Option<String>,
    /// 指向 Keyring / fallback 文件的引用键（非明文）
    pub secret_ref: String,
    /// 超参数（可选）
    #[serde(default)]
    pub params: LLMParams,
}

/// 超参数
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LLMParams {
    /// 温度（默认 0.7）
    #[serde(default = "default_temperature")]
    pub temperature: f64,
    /// 最大输出 token（默认 4096）
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u64,
}

fn default_temperature() -> f64 {
    0.7
}

fn default_max_tokens() -> u64 {
    4096
}

/// TOML 顶层结构（仅解析 llm 段）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct ConfigFile {
    /// `[llm.<role>]` 嵌套段
    #[serde(default)]
    llm: BTreeMap<String, LLMConfigEntry>,
}

/// TOML 中 `[llm.<role>]` 段的原始形状（与 LLMConfig 几乎相同，但不含 role）
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LLMConfigEntry {
    provider: String,
    model: String,
    base_url: Option<String>,
    secret_ref: String,
    #[serde(default)]
    params: LLMParams,
}

impl From<&LLMConfig> for LLMConfigEntry {
    fn from(cfg: &LLMConfig) -> Self {
        Self {
            provider: cfg.provider.clone(),
            model: cfg.model.clone(),
            base_url: cfg.base_url.clone(),
            secret_ref: cfg.secret_ref.clone(),
            params: cfg.params.clone(),
        }
    }
}

/// 返回 `~/.core-ai-pet/config.toml` 的完整路径
pub fn config_path() -> Result<PathBuf, ConfigError> {
    let home = std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .ok_or_else(|| ConfigError::ParseError("cannot determine home directory".to_string()))?;
    Ok(PathBuf::from(home).join(".core-ai-pet").join(CONFIG_FILENAME))
}

/// 加载指定 role 的 LLM 配置
///
/// # Errors
/// - `ConfigError::FileNotFound` - 配置文件不存在（首次启动）
/// - `ConfigError::ParseError` - TOML 解析失败
/// - `ConfigError::RoleNotFound` - 指定 role 在配置中不存在
/// - `ConfigError::Io` - 文件 I/O 错误
pub fn load_llm_config(role: &str) -> Result<LLMConfig, ConfigError> {
    let path = config_path()?;

    if !path.exists() {
        return Err(ConfigError::FileNotFound);
    }

    let content = std::fs::read_to_string(&path)?;
    let cfg_file: ConfigFile = toml::from_str(&content)
        .map_err(|e| ConfigError::ParseError(e.to_string()))?;

    let entry = cfg_file
        .llm
        .get(role)
        .ok_or_else(|| ConfigError::RoleNotFound(role.to_string()))?;

    Ok(LLMConfig {
        role: role.to_string(),
        provider: entry.provider.clone(),
        model: entry.model.clone(),
        base_url: entry.base_url.clone(),
        secret_ref: entry.secret_ref.clone(),
        params: entry.params.clone(),
    })
}

/// 保存指定 role 的 LLM 配置（原子写）
///
/// # Errors
/// - `ConfigError::ParseError` - 现有配置文件解析失败
/// - `ConfigError::SerializeError` - TOML 序列化失败
/// - `ConfigError::Io` - 文件 I/O 错误
pub fn save_llm_config(role: &str, cfg: &LLMConfig) -> Result<(), ConfigError> {
    let path = config_path()?;

    // 读取现有配置（若不存在则创建空）
    let mut cfg_file: ConfigFile = if path.exists() {
        let content = std::fs::read_to_string(&path)?;
        toml::from_str(&content).map_err(|e| ConfigError::ParseError(e.to_string()))?
    } else {
        ConfigFile::default()
    };

    // 更新指定 role 的配置
    cfg_file.llm.insert(role.to_string(), LLMConfigEntry::from(cfg));

    // 序列化为 TOML
    let new_content = toml::to_string_pretty(&cfg_file)
        .map_err(|e| ConfigError::SerializeError(e.to_string()))?;

    // 原子写：先写临时文件，再 rename
    let tmp_path = path.with_extension("toml.tmp");
    std::fs::write(&tmp_path, &new_content)?;
    std::fs::rename(&tmp_path, &path)?;

    log::info!("Saved LLM config for role={}", role);
    Ok(())
}

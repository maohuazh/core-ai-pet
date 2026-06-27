//! LLM 配置模块
//!
//! 职责：
//! - 读取 `~/.core-ai-pet/config.toml` 的 `[llm.<role>]` 段
//! - 将 TOML 解析为 `LLMConfig` 结构体
//! - 将 UI 修改回写 TOML（原子写）
//!
//! 详见 PRD §6.1 / ARCH §3.8（ADR-003）/ design.md D2。
//!
//! 当前为骨架，具体实现由 task 1.5 / 1.6 / 1.7 完成。

use serde::{Deserialize, Serialize};

/// LLM 槽位配置（一个 role 的全部设定）
///
/// 对应 TOML 段 `[llm.<role>]`，例如：
///
/// ```toml
/// [llm.chat_assistant]
/// provider = "anthropic"
/// model = "claude-fable-5"
/// secret_ref = "k-uuid-1"
///
/// [llm.chat_assistant.params]
/// temperature = 0.7
/// max_tokens = 4096
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    /// 槽位身份（role 名称）
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

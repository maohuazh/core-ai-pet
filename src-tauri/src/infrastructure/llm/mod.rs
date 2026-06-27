//! LLM 子系统的 infrastructure 层
//!
//! 包含：
//! - `config`：TOML 配置加载 / 保存（`[llm.<role>]` 嵌套段）
//! - `secret_store`：OS Keyring 存取 + DPAPI fallback
//! - `provider`：LLM 提供商实现（Anthropic 等）

pub mod config;
pub mod provider;
pub mod secret_store;

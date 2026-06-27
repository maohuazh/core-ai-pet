//! LLM 子系统的 infrastructure 层
//!
//! 包含：
//! - `config`：TOML 配置加载 / 保存（`[llm.<role>]` 嵌套段）
//! - `secret_store`：OS Keyring 存取 + DPAPI fallback

pub mod config;
pub mod secret_store;

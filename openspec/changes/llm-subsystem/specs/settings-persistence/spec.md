## MODIFIED Requirements

### Requirement: Configuration persistence schema
系统 SHALL 提供统一的配置持久化机制。**M1 新增 `[llm.*]` 段（按 role 嵌套）的 TOML 读写路径，与现有 `[pet]` / `[window]` / `[model]` 平级。**

#### Scenario: M1 TOML structure
- **WHEN** 系统读取或写入 `~/.core-ai-pet/config.toml`
- **THEN** 配置 MUST 支持以下顶级段：
  - `[pet]` (existing)
  - `[window]` (existing)
  - `[model]` (existing)
  - `[llm.chat_assistant]` (new)
  - `[llm.chat_assistant.params]` (new, optional)

#### Scenario: Read LLM config
- **WHEN** Harness 启动需要读取配置
- **THEN** MUST 调用 `invoke('llm_load_config', { role })`，Rust 端解析 TOML 并返回 `LLMConfig` 对象

#### Scenario: Write LLM config
- **WHEN** 用户在设置面板保存
- **THEN** MUST 调用 `invoke('llm_save_config', { role, cfg })`，Rust 端原子写 TOML 对应段

#### Scenario: 不破坏其他段
- **WHEN** 写 LLM 配置
- **THEN** MUST 保留其他段（pet / window / model）的现有内容不变

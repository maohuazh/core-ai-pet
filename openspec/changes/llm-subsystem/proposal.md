## Why

CoreAIpet 的桌宠主程序目前只能执行预定义动作。PRD 规定桌宠需要接入 LLM 子系统（`message_processor` 角色），使桌面助手能真正"听懂"自然语言并触发表情/动作/通知。这是整个 LLM 集成路线图（PRD §10 Milestone M1）的第一块拼图 — **Harness 骨架 + 单个 Provider + 流式输出**。

不实现这一步，后续所有能力（多 Provider、Tool、MCP、Chat 窗口、Session、Memory、Egress 闸门、审计）都没有入口。M1 的目标是 **让 `llm.invoke(role, req)` 能真正发出请求并流式拿到响应**，哪怕只走 Anthropic 一条链路、哪怕还不持久化、还不接桌宠表情。

这是整个 LLM 子系统 12 个里程碑里的第一个，不是一次性做完所有 LLM 功能。

## What Changes

- **新增** `src/core/llm/` 目录：角色注册（`LLMRole`）、统一请求/响应类型（`UnifiedRequest` / `UnifiedDelta`）、Facade（`llm.invoke`）、Provider 接口（`LLMProvider`）、Registry（`LLMRegistry`）。
- **新增** Anthropic Provider 适配器：Anthropic Messages API，仅 streaming 模式，输出归一化为 `UnifiedDelta`。
- **新增** SSE 流式解析：基于 Web ReadableStream + ReadableStreamDefaultReader，按 Anthropic 事件协议（`message_start` / `content_block_delta` / `message_delta` / `message_stop`）归一化。
- **新增** 配置加载：从 `~/.core-ai-pet/config.toml` 读取 `[llm.message_processor]` 槽位配置（provider/model/base_url/secret_ref/params）。
- **新增** 密钥读取：通过 Tauri command `llm_get_secret(secret_ref)` 从 OS Keyring 取 API key（明文不出 Rust）。
- **新增** 设置面板 `💰 用量` 占位 + `🤖 AI 模型` 槽位 UI（chat_assistant 槽位可配置；message_processor 默认用 TOML 配置）。
- **新增** 测试连接按钮：用 `invoke('llm_test_connection', { role })` 调 `ping()` 返回可用状态。
- **新增** 触发链路：`triggerHandler` 在 `trigger_key = llm.message` 时调用 `llm.invoke(role='chat_assistant', req)`，结果通过 `llm.stream` 事件通知到 UI（Chat 窗口占位 div）。
- **不实现**（M1 明确不做）：多 Provider、OpenAI 适配、Tool / MCP、Context 压缩、Egress/PII/Quota 闸门、审计、Chat 窗口完整 UI、Workspace、Memory、Persona、Privacy、多会话、热键。

## Capabilities

### New Capabilities

- `llm-role-registry`: LLM 角色定义与运行时 Registry，业务通过 role 调用，Registry 解析为具体的 Provider 实例。
- `llm-unified-types`: 统一的 UnifiedRequest / UnifiedDelta / LLMProvider 接口，是 Provider 适配层的契约。
- `llm-harness-facade`: `llm.invoke(role, req)` Facade，负责解析 role → 取配置 → 调 Provider → 返回流。
- `llm-provider-anthropic`: 第一个 Provider 适配器，Anthropic Messages API + SSE streaming，输出归一化为 UnifiedDelta。
- `llm-config-toml`: TOML 配置加载器，解析 `[llm.<role>]` 槽位配置块，注入到 Harness。
- `llm-secret-keyring`: Tauri 命令 `llm_get_secret` / `llm_save_secret`，跨进程安全地取/存 API key。
- `llm-settings-ui`: AI 模型设置面板 UI，含槽位列表、provider 选择、模型配置、测试连接按钮。
- `llm-trigger-integration`: triggerHandler 在 `llm.message` / `llm.invoke` 触发时调用 Harness，流式结果 emit 到前端。

### Modified Capabilities

- `trigger-handler`: 新增 `llm.message` / `llm.invoke` / `llm.stream` 三个 trigger_key，让桌宠能感知 LLM 调用状态。
- `settings-persistence`: 新增 LLM 配置段（`[llm.*]`）的读写路径，与现有 `[pet]` / `[window]` / `[model]` 平级。

## Impact

- **新增代码**：约 2500 行 TS + 300 行 Rust（`src/core/llm/*`、`src/modules/settings/llm/*`、`src-tauri/src/infrastructure/llm/*`、`src-tauri/src/commands/llm.rs`）。
- **修改代码**：`triggerHandler.ts`（新增 3 个 trigger_key）、`App.vue`（占位 Chat 浮窗）、`PetStore.ts`（暴露当前 LLM 状态）、`src-tauri/src/main.rs`（注册新命令）。
- **新增依赖**：前端无新依赖（用原生 fetch + ReadableStream）；Rust 端 `toml` / `serde` 已有，`keyring` crate 新增。
- **API 影响**：新增 Tauri 命令 `llm_invoke` / `llm_get_secret` / `llm_save_secret` / `llm_test_connection`。
- **文件**：`~/.core-ai-pet/config.toml` 新增 `[llm.*]` 段。
- **风险**：API key 管理（首次启动无 key 时优雅降级）、网络请求阻塞（必须异步 + AbortController）、TOML 解析错误（必须 fail-closed 并 toast）。

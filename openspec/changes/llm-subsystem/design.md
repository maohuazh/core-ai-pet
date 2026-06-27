## Context

CoreAIpet 是一个 Tauri 2 + Vue 3 + Live2D 的桌面宠物应用。现有架构围绕 Live2D 渲染、动作/表情系统、trigger 事件总线、设置面板构成。PRD（`docs/prd/PRD-LLM-Integration.md` v1.5）和架构文档（`docs/arch/ARCH-LLM-Integration.md` v1.0）已经完成了整个 LLM 子系统的蓝图，但当前代码中 **没有任何一行 LLM 相关代码**。

本设计覆盖 **M1（PRD §10 的第一个 milestone）**，目标是：让 `llm.invoke(role, req)` 真正能发出一个 Anthropic streaming 请求并把 `UnifiedDelta` 流推送给 UI。

约束：
- 不引入新的运行时依赖（Node/CLI/SDK）。
- 不破坏现有触发链路（桌宠动作/表情）。
- API key 明文不出 Rust 进程（ARCH §5.3 ADR-006）。
- 配置用 TOML（ADR-003），不入 SQLite。

## Goals / Non-Goals

**Goals:**

- G1. 业务代码只通过 `llm.invoke(role, req)` 调用，不直接 import provider 细节。
- G2. 支持 Anthropic Messages API streaming 模式，归一化为 `UnifiedDelta`。
- G3. API key 明文不出 Rust 进程；Renderer 只持有 `secret_ref`。
- G4. 配置读取 `~/.core-ai-pet/config.toml` 的 `[llm.<role>]` 槽位。
- G5. 设置面板至少支持一个 role（chat_assistant）可配置 + 测试连接。
- G6. triggerHandler 能在 `llm.message` / `llm.invoke` / `llm.stream` 触发时调用 Harness，流式事件 emit 到前端。
- G7. 所有失败模式有明确降级路径（fail-closed for 安全、fail-open for 显示）。

**Non-Goals:**

- NG1. 多 Provider（OpenAI、Azure、Gateway）— M2 的事。
- NG2. Tool / MCP — M3-M4 的事。
- NG3. Chat 窗口完整 UI（三栏布局）— M12 的事；M1 只做一个浮层 placeholder 显示原始 delta。
- NG4. Session / Workspace / Memory / Knowledge — 后续 milestone。
- NG5. Egress / PII / Quota / Audit — M8-M9 的事。
- NG6. 多窗口 / 全局热键 — M12 的事。
- NG7. 桌宠表情联动（trigger_key = llm.thinking/streaming/...）— M6 的事；M1 只接触发链路，不做完整 8 trigger_key 表情矩阵。

## Decisions

### D1. 走 Rust SSE（ADR-006 方案 A），不走 Renderer 直连 fetch

**选择**：Provider 的 HTTPS + SSE 调用从 Rust 端发起，通过 `emit_to` 把 `UnifiedDelta` 推给 Renderer。

**替代方案**：Renderer 直连 fetch（PRD §2.2 方案 B），明文 key 通过 `invoke('inject_auth')` 临时注入请求头。

**理由**：
- 明文 key 永远不出 Rust 进程，满足 ARCH §5.3 硬约束。
- Tauri 的 `emit_to` 天然支持事件流到指定窗口，无额外 IPC 协议开销。
- Rust 端 `reqwest` 库对 SSE 的 ReadableStream 支持成熟。
- 性能：一次 IPC ~5-15ms，用户感知 RTT 主要在网络，不影响体验。

**代价**：
- Rust 端代码量增加 ~150 行（reqwest + tokio 流）。
- 需要定义一个 Rust → TS 的事件协议（`llm_delta` 事件，payload 为 `UnifiedDelta` 序列化）。

### D2. TOML 配置 schema：`[llm.<role>]` 嵌套

**选择**：
```toml
[llm.chat_assistant]
provider = "anthropic"
model = "claude-fable-5"
secret_ref = "k-uuid-1"
[llm.chat_assistant.params]
max_tokens = 4096
temperature = 0.7
```

**替代方案**：
- 入 SQLite — 违反 ADR-003。
- JSON — 不如 TOML 人可读。
- 顶层 `[llm]` 平铺 — 不支持多 role 扩展。

**理由**：
- 与 PRD §6.1 / ARCH §3.8 一致。
- TOML 的 section 嵌套天然支持 role 扩展（后续加 `message_processor` 不影响 chat_assistant 段）。
- `params` 子段允许不同 role 不同超参数。

### D3. UnifiedDelta 6 个 type

**选择**：`text | thinking | tool_use_start | tool_use_delta | tool_use_end | usage | stop | error`（共 8 个）。

M1 只实际 emit `text | usage | stop | error` 四种；thinking/tool_use_* 预留但不 emit。

**理由**：
- 与 ARCH §4.3 一致；M2 OpenAI 适配器要复用同一类型。
- M1 先做最小集，但类型定义一次写到位，避免后续 PR 重定义。

### D4. LLMProvider 接口的 invoke 返回 AsyncIterable

**选择**：`async invoke(cfg, req, ctx): AsyncIterable<UnifiedDelta>`

**替代方案**：
- `Promise<Response>` — 不 stream。
- 回调 `onDelta: (delta) => void` — 测试难做。

**理由**：
- AsyncIterable 天然支持 `for await (const delta of provider.invoke(...))`，业务代码简洁。
- 测试时可以用 `async function*` 模拟 provider。
- 与 ARCH §7.2.1 接口一致。

### D5. Registry 用内存 Map，不做 plugin 动态加载

**选择**：启动时硬编码注册 Anthropic（未来 M2 加 OpenAI），通过 `registry.resolve(cfg.provider)` 查找。

**替代方案**：
- 动态 import 第三方 plugin — 违反 ARCH §7 ADR-011。
- 配置文件声明 provider 列表 — 过度工程。

**理由**：
- M1 单一 provider，Map 最简。
- 测试可注入 mock provider。
- 扩展点保留在 Registry 接口，后续无破坏性。

### D6. Secret 用 OS Keyring，fallback DPAPI

**选择**：Rust `keyring` crate，失败时写入 `~/.core-ai-pet/.secrets/<secret_ref>` + DPAPI 加密（Windows）。

**替代方案**：
- 明文存 TOML — 违反 PRD §1.3 威胁模型。
- 环境变量 — 不支持多 secret。

**理由**：
- 与 PRD §6.3 / ARCH §5.3 一致。
- keyring crate 在 Win/Mac/Linux 都稳定。
- M1 先做 keyring + DPAPI fallback 两条路径。

### D7. triggerHandler 新增 3 个 trigger_key（不是完整的 8 个）

**选择**：M1 只加 `llm.message` / `llm.invoke` / `llm.stream` 三个。完整 8 个 trigger_key（PRD §4.10）是 M6 的事。

**理由**：
- M1 目标是"invoke 能跑通 + UI 能显示流"，不需要桌宠表情联动。
- `llm.stream` 是个复合 trigger（每次 delta emit 都触发），给 Chat 浮层刷新信号。
- 完整矩阵需要 Pet 资源（thinking/streaming/tool_running 等 8 个表情），资源没齐。

### D8. Chat 浮层用最小占位 div，不用完整三栏

**选择**：在 `App.vue` 右上角加一个 400×300 的浮层，显示原始 delta 文本流。

**替代方案**：
- 直接做三栏 — 违反 NG3。
- 不做 UI — 无法观察流式效果。

**理由**：
- 用户能立即看到"LLM 在说话"。
- 与 M12 完整 Chat 窗口不冲突（M12 时替换这个占位）。
- 代码量小（~50 行 Vue）。

## Risks / Trade-offs

### R1. Tauri emit_to 大 delta 时 IPC 拥塞
**风险**：流式响应每秒可能几十次 delta，每次 `emit_to` 都序列化 JSON。
**缓解**：`llm_delta` payload 最小化（只含 delta type + 文本片段，不含 metadata）；UI 端 `requestAnimationFrame` 批量 flush。若仍拥塞，考虑 batch 协议（每 50ms 一次 emit 含多个 delta）。
**监控**：M1 完成后压测 10 秒连续流，观察 UI 帧率。

### R2. TOML 解析错误时 UI 行为
**风险**：用户手改 TOML 写错字段。
**缓解**：Rust 端 fail-closed（拒绝启动该 role 的 invoke）+ toast 提示 + log 错误。不崩溃、不静默忽略。
**测试**：单测覆盖 5 种错误 TOML。

### R3. Keyring 在某些 Linux 桌面环境不可用
**风险**：无 keyring 守护进程时 keyring crate 抛错。
**缓解**：fallback 到 DPAPI 加密文件；首次启动时 toast 提示"使用文件加密存储，建议安装 keyring 守护进程"。
**测试**：CI 模拟无 keyring 环境。

### R4. Anthropic streaming 协议变更
**风险**：Anthropic 改事件名/字段。
**缓解**：适配层与协议绑定，业务层只看 UnifiedDelta；协议变更只需改 anthropic.ts。
**测试**：录制 5 个真实 SSE 样本，回归测试。

### R5. 单 role 配置扩展性
**风险**：M2 加 message_processor 时需要重新设计配置 schema。
**缓解**：D2 schema 已经用 `[llm.<role>]` 嵌套，扩展只是多加一段。
**风险**：M1 代码硬编码 `chat_assistant` role 名。
**缓解**：Registry 用 Map，不写 if 分支。

### R6. Chat 浮层与现有窗口冲突
**风险**：浮层位置挡住 Live2D 主体。
**缓解**：固定右上角，400×300，与 Pet 主窗口（左上角）无重叠。
**迁移**：M12 整体删除此占位，不迁移用户状态。

### R7. 设置面板 UI 与现有 LLM 相关 tab 冲突
**风险**：现有设置面板已有 `💰 用量` 等 tab，新增 `🤖 AI 模型` 可能视觉拥挤。
**缓解**：加一个 scrollable tab bar；tab 超过 6 个时启用横向滚动。
**测试**：UI 测试覆盖 6+ tab 场景。

## Migration Plan

**部署**：
1. 后端先上：config.toml 默认值、Rust 端 keyring / TOML / llm 命令。
2. 前端跟上：src/core/llm/ + 设置面板 + Chat 浮层。
3. 用户首次使用：设置面板配置 chat_assistant role → 测试连接 → OK 后触发 `llm.message`。

**回滚**：
- 完全删除 `src/core/llm/`、`src-tauri/src/infrastructure/llm/`、`src-tauri/src/commands/llm.rs`。
- TOML 保留 `[llm.*]` 段，不影响其他模块。
- keyring 中的 secret 不删除（用户可能重启用）。

**灰度**：
- M1 不做灰度开关 — 全量上线。若问题，用户可通过 TOML 禁用（删 `[llm.chat_assistant]` 段 = 该 role 不可用）。

## Open Questions

### Q1. Chat 浮层是否要"持久化"（重启后还在）？
M1 倾向：**不持久化**。浮层是调试/观察用，M12 才做真正的 Chat 窗口。

### Q2. `llm_get_secret` 是否要缓存？
M1 倾向：**每次 invoke 都重新取**。M2 再引入 secret 缓存（减少 keyring 调用频率）。

### Q3. Anthropic provider 是否要在 M1 实现 retry？
M1 倾向：**只做一次重试 + 指数退避**（250ms → 1000ms），不上断路器。断路器是 M9 的事。

### Q4. 测试连接用哪个 model？
M1 倾向：用配置中已填的 model 发极小请求（`max_tokens=1, prompt="hi"`）。

### Q5. trigger_key = llm.message 的触发源是什么？
M1 倾向：设置面板的"发送一条测试消息"按钮。M2 之后接 Chat 窗口输入框。

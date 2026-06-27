## 1. 后端基础：配置与密钥

- [x] 1.1 在 `src-tauri/src/infrastructure/` 下创建 `llm/` 目录骨架（`mod.rs` + `config.rs` + `secret_store.rs`）。
- [x] 1.2 在 `Cargo.toml` 新增 `keyring` / `toml` / `serde` 依赖（`toml` 与 `serde` 已存在则仅 `keyring`）。
- [x] 1.3 实现 `secret_store.rs`：`save_secret(role, plaintext) -> secret_ref` / `get_secret(secret_ref) -> plaintext` / `delete_secret(secret_ref)`，Keyring service=`coreai-llm`。
- [ ] 1.4 实现 DPAPI fallback：Keyring 失败时写 `~/.core-ai-pet/.secrets/<secret_ref>`，Windows 用 DPAPI，Unix 用 mode 0600。
- [ ] 1.5 实现 `config.rs`：`load_llm_config(role) -> LLMConfig` / `save_llm_config(role, cfg)`，TOML 段 `[llm.<role>]` 嵌套解析。
- [ ] 1.6 单测：5 种 TOML 错误（缺失必填字段 / 非法类型 / 空段 / 嵌套错 / 文件不存在）的行为。
- [ ] 1.7 单测：secret_store 的存/取/删 + fallback 路径 + 文件权限 0600（Unix only）。

## 2. 后端：Tauri 命令

- [ ] 2.1 在 `src-tauri/src/commands/` 下新增 `llm.rs`，声明 5 个命令：`llm_load_config` / `llm_save_config` / `llm_get_secret` / `llm_save_secret` / `llm_delete_secret`。
- [ ] 2.2 新增 `llm_test_connection` 命令：接收 `{ role }`，内部构造 ping 请求返回 `{ ok, reason? }`。
- [ ] 2.3 新增 `llm_invoke` 命令：接收 `{ role, request }`，在 Rust 端持有 plaintext key，发起 HTTPS+SSE 请求（详见 §3），通过 `emit_to` 推 `llm_delta` 事件给 Renderer。
- [ ] 2.4 在 `src-tauri/src/main.rs` 注册上述命令。
- [ ] 2.5 单测：`llm_get_secret` 缺失时返回 `secret_not_found` 错误。
- [ ] 2.6 单测：`llm_invoke` 在配置缺失时返回 `config_not_found`。

## 3. 后端：Anthropic Provider (Rust)

- [ ] 3.1 在 `src-tauri/src/infrastructure/llm/` 新增 `provider/anthropic.rs`。
- [ ] 3.2 实现 `invoke()`：`reqwest::Client::post(base_url + "/v1/messages")`，header 含 `x-api-key` / `anthropic-version` / `content-type`，body 含 `messages` / `model` / `stream=true` / `max_tokens`。
- [ ] 3.3 实现 SSE 流解析：按 `event:` 行 + `data:` 行解析，识别 `message_start` / `content_block_delta` / `message_delta` / `message_stop` / `error`。
- [ ] 3.4 实现 Anthropic 事件 → `UnifiedDelta` 归一化：text content → text delta；thinking content → thinking delta；usage → usage delta；message_stop → stop delta；error → error delta。
- [ ] 3.5 实现 `ping()`：发送极小请求（max_tokens=1, prompt="hi"），返回 ok/reason。
- [ ] 3.6 实现 `estimateCost()`：按 model 查表估算，未知 model 返回 0。
- [ ] 3.7 集成测试：用 mock server（wiremock 或类似）模拟 SSE，验证 5 种事件归一化。
- [ ] 3.8 集成测试：ping 的 401 / 网络错误场景。

## 4. 前端：核心类型与 Registry

- [ ] 4.1 在 `src/core/llm/` 下新增 `role.ts`（`LLMRole` 联合类型）/ `types.ts`（`LLMConfig` / `UnifiedRequest` / `UnifiedDelta` / `LLMProvider` 接口）/ `registry.ts`（`LLMRegistry` 单例）。
- [ ] 4.2 `LLMRegistry` 实现 `register` / `resolve` / `listRoles`，Map 存储，幂等覆盖 + WARN 日志。
- [ ] 4.3 单测：Registry 的 register / resolve / duplicate / unknown / list。
- [ ] 4.4 单测：UnifiedDelta 8 种 type 的 TypeScript 类型检查（`vitest typecheck`）。

## 5. 前端：Provider Adapter (Anthropic)

- [ ] 5.1 在 `src/core/llm/providers/` 下新增 `anthropic.ts`，实现 `LLMProvider` 接口。
- [ ] 5.2 `invoke()` 通过 `invoke('llm_invoke', ...)` 调 Rust，监听 `llm_delta` 事件，转换为本地 `AsyncIterable<UnifiedDelta>`（用 `AsyncGenerator` + yield）。
- [ ] 5.3 `ping()` / `estimateCost()` 通过 `invoke('llm_test_connection')` / 本地定价表实现。
- [ ] 5.4 启动时注册到 Registry：`registry.register('anthropic', anthropicProvider)`。
- [ ] 5.5 单测：mock Tauri invoke + emit_to，验证 Anthropic delta 归一化。

## 6. 前端：Facade

- [ ] 6.1 在 `src/core/llm/client.ts` 实现 `llm.invoke(role, req, opts?)`：解析 role → Registry → 取 LLMConfig → 调 Provider → 返回流。
- [ ] 6.2 错误处理：`RoleNotFoundError` / `ConfigNotFoundError` / `SecretNotFoundError`，不发起网络。
- [ ] 6.3 AbortController 支持：`opts.abort` 信号传入 ProviderCallContext。
- [ ] 6.4 单测：正常路径 / 三种错误 / abort / provider 5xx / provider 4xx（透传 error delta）。

## 7. 前端：设置面板 UI

- [ ] 7.1 在 `src/modules/settings/` 下新增 `LLMSettings.vue` + `LLMRoleForm.vue` 组件。
- [ ] 7.2 在现有设置面板的 tab 栏新增 `🤖 AI 模型` tab（若 tab 超过 6 个启用横向滚动）。
- [ ] 7.3 LLMRoleForm 渲染 provider 下拉（M1 仅 anthropic）+ model 输入 + base_url 输入 + secret 输入（type=password）+ params 表单。
- [ ] 7.4 实现"测试连接"按钮：点击 → loading → 成功绿勾 / 失败红叉 + reason。
- [ ] 7.5 实现"保存"按钮：校验 + `invoke('llm_save_config')` + 写失败回滚 UI。
- [ ] 7.6 单测：表单校验（必填项 / URL 格式）+ 保存流程（mock Tauri invoke）。

## 8. 前端：Trigger 集成 + Chat 浮层

- [ ] 8.1 在 `src/core/events/triggerHandler.ts` 新增 `llm.message` / `llm.invoke` 事件分支，调用 `llm.invoke()`。
- [ ] 8.2 Facade 流式过程中 emit `llm.stream` 事件到 eventBus（含 `turn_id` + delta）。
- [ ] 8.3 流结束 emit `llm.done`。
- [ ] 8.4 在 `App.vue` 新增 `ChatPlaceholder.vue` 浮层（400×300，右上角，固定位置）。
- [ ] 8.5 ChatPlaceholder 监听 `llm.stream`，追加 text delta 到显示；监听 `llm.done` 显示完成态。
- [ ] 8.6 浮层自动滚动到底部（除非用户手动上滚）。
- [ ] 8.7 浮层关闭按钮：隐藏不销毁，下次触发再显示。
- [ ] 8.8 单测：triggerHandler 三个 trigger_key 的分发逻辑。

## 9. 端到端验证

- [ ] 9.1 启动应用 → 设置面板配置 `chat_assistant`（provider=anthropic / model=claude-fable-5 / API key）→ 测试连接成功。
- [ ] 9.2 通过触发链路发送一条 "hello" → Chat 浮层显示流式响应。
- [ ] 9.3 中断 API key → 测试连接失败 + 红叉显示 "unauthorized"。
- [ ] 9.4 删除 config.toml `[llm.chat_assistant]` 段 → 重启 → toast 提示配置缺失。
- [ ] 9.5 模拟 Anthropic 500 → Facade yield error delta → 浮层显示错误消息。

## 10. 文档与发布

- [ ] 10.1 更新 `docs/arch/ARCH-LLM-Integration.md` 的 §9.1 Milestones，标记 M1 完成。
- [ ] 10.2 在 `openspec/specs/` 中新增 M1 相关能力的主规格文件（从 delta spec 合并到 main spec）。
- [ ] 10.3 撰写 `CHANGELOG.md` 条目：M1 Harness 骨架 + Anthropic Provider + 流式输出 + 设置面板 + Chat 浮层。
- [ ] 10.4 在 GitHub 创建 PR，描述 M1 范围 + 验证步骤 + 已知限制（不做 retry / 不做多 provider / 不做 Tool）。

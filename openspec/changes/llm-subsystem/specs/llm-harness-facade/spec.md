## ADDED Requirements

### Requirement: llm.invoke Facade 函数
系统 SHALL 导出 `llm.invoke(role, req, opts?)` 作为业务唯一入口。

#### Scenario: 正常调用
- **WHEN** 业务代码调用 `llm.invoke('chat_assistant', { messages: [...], stream: true })`
- **THEN** Facade MUST：
  1. 解析 role → Registry → Provider
  2. 读取 role 对应的 LLMConfig
  3. 通过 `invoke('llm_get_secret', { secret_ref })` 取 API key（明文不出 Rust）
  4. 调用 `provider.invoke(cfg, req, ctx)` 返回 AsyncIterable
  5. 透传 delta 流给调用方

#### Scenario: Unknown role
- **WHEN** role 不在 Registry 中
- **THEN** MUST 抛出 `RoleNotFoundError`，不发起网络请求

#### Scenario: 配置缺失
- **WHEN** role 配置段不存在或必填字段缺失
- **THEN** MUST 抛出 `ConfigNotFoundError`，并 toast 提示用户

#### Scenario: Secret 不存在
- **WHEN** secret_ref 在 Keyring 中找不到
- **THEN** MUST 抛出 `SecretNotFoundError`，并 toast 提示用户配置 API key

#### Scenario: AbortController 中断
- **WHEN** 调用方在 opts 中传入 `abort: AbortSignal` 并 abort
- **THEN** Facade MUST 终止 provider 请求，清理资源

### Requirement: Facade 不做重试（M1）
系统 SHALL 在 M1 不做自动重试（retry 是 M9 的事），provider 抛错直接透传 error delta。

#### Scenario: Provider 5xx 错误
- **WHEN** Anthropic 返回 500
- **THEN** Facade MUST yield `{ type: 'error', recoverable: true, code: 'provider_5xx', message: '...' }` 后立即结束流

#### Scenario: Provider 4xx 错误
- **WHEN** Anthropic 返回 401 / 400
- **THEN** Facade MUST yield `{ type: 'error', recoverable: false, code: 'provider_4xx', message: '...' }` 后立即结束流

## ADDED Requirements

### Requirement: LLMRole 类型定义
系统 SHALL 定义 `LLMRole` 类型为字符串字面量联合，v1 只包含 `'chat_assistant' | 'message_processor'`，未来可扩展。

#### Scenario: Role 类型可被编译器校验
- **WHEN** 业务代码使用 `LLMRole` 类型声明变量
- **THEN** TypeScript 编译器 MUST 拒绝任何不在联合中的字面量

#### Scenario: Role 类型可扩展
- **WHEN** 未来新增 role 字面量
- **THEN** 修改点 MUST 仅在 `src/core/llm/role.ts` 一个文件中

### Requirement: LLMRegistry 单例
系统 SHALL 提供一个 `LLMRegistry` 单例，维护 `role → Provider` 的映射。

#### Scenario: Register a provider
- **WHEN** 调用 `registry.register(role, provider)`
- **THEN** 之后调用 `registry.resolve(role)` MUST 返回该 provider

#### Scenario: Duplicate registration
- **WHEN** 对同一 role 重复 register 不同 provider
- **THEN** Registry MUST 用新 provider 覆盖旧的，并打 WARN 日志

#### Scenario: Resolve unknown role
- **WHEN** 调用 `registry.resolve('unknown_role')`
- **THEN** MUST 返回 `null`

#### Scenario: List roles
- **WHEN** 调用 `registry.listRoles()`
- **THEN** MUST 返回已注册的 role 数组（readonly，外部不能修改）

### Requirement: Provider 接口 LLMProvider
系统 SHALL 定义 `LLMProvider` 接口，所有 provider 适配 MUST 实现此接口。

#### Scenario: Provider 必须声明 id 与 capabilities
- **WHEN** 一个 provider 实例被创建
- **THEN** MUST 暴露 `id: string`、`capabilities: { streaming, thinking, tool_use, vision }`、`schemaVersion: '1'`

#### Scenario: Provider MUST 实现 ping
- **WHEN** 调用 `provider.ping(cfg, abort)`
- **THEN** MUST 返回 `{ ok: true }` 或 `{ ok: false, reason: string }`

#### Scenario: Provider MUST 实现 invoke 返回 AsyncIterable
- **WHEN** 调用 `provider.invoke(cfg, req, ctx)`
- **THEN** MUST 返回 `AsyncIterable<UnifiedDelta>`，且 MUST 支持 for-await-of 消费

#### Scenario: Provider MUST 实现 estimateCost
- **WHEN** 调用 `provider.estimateCost(req, cfg)`
- **THEN** MUST 返回 `{ input_usd, output_usd_max }` 估算，基于 cfg.params.max_tokens

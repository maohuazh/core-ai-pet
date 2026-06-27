## ADDED Requirements

### Requirement: UnifiedRequest 类型
系统 SHALL 定义 `UnifiedRequest` 为所有 provider 共用的请求格式。

#### Scenario: UnifiedRequest 字段完整
- **WHEN** 业务代码构造 UnifiedRequest
- **THEN** MUST 包含：`messages: Message[]`、`system?: string`、`tools?: Tool[]`、`stream: true`

#### Scenario: Message 类型完整
- **WHEN** 构造一条 message
- **THEN** MUST 支持 role ∈ `{'user' | 'assistant' | 'system'}`，content 为 string 或 ContentBlock[]

### Requirement: UnifiedDelta 类型
系统 SHALL 定义 `UnifiedDelta` 为流式响应的归一化事件类型。

#### Scenario: Delta 类型枚举
- **WHEN** Provider 发出 delta
- **THEN** type MUST 是以下之一：`text | thinking | tool_use_start | tool_use_delta | tool_use_end | usage | stop | error`

#### Scenario: text delta
- **WHEN** provider 发出文本片段
- **THEN** delta MUST 形如 `{ type: 'text', delta: string }`

#### Scenario: usage delta
- **WHEN** provider 报告 token 用量
- **THEN** delta MUST 形如 `{ type: 'usage', input_tokens?: number, output_tokens?: number, cached?: number }`

#### Scenario: stop delta
- **WHEN** provider 结束流
- **THEN** delta MUST 形如 `{ type: 'stop', reason: 'end_turn' | 'tool_use' | 'max_tokens' | 'stop_sequence' }`

#### Scenario: error delta
- **WHEN** 流中出错
- **THEN** delta MUST 形如 `{ type: 'error', recoverable: boolean, code: string, message: string }`

### Requirement: LLMConfig 类型
系统 SHALL 定义 `LLMConfig` 为一个槽位的全部配置。

#### Scenario: LLMConfig 字段
- **WHEN** 从 TOML 读取配置
- **THEN** MUST 解析为：`{ role, provider, model, base_url?, secret_ref, params: { temperature?, max_tokens? } }`

#### Scenario: secret_ref 必存在
- **WHEN** 配置段无 secret_ref
- **THEN** 解析 MUST 抛错并拒绝该 role 可用

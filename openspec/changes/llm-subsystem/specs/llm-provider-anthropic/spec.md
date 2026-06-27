## ADDED Requirements

### Requirement: Anthropic Messages API 适配
系统 SHALL 提供 `anthropicProvider`，适配 Anthropic Messages API v1 streaming。

#### Scenario: 支持 streaming=true
- **WHEN** invoke 时 cfg 含 model 且 req.stream=true
- **THEN** MUST 发送 `POST /v1/messages` 请求，`stream: true` 在 body 中，`anthropic-version` header 设为最新稳定版

#### Scenario: Authorization header 注入
- **WHEN** 发起请求
- **THEN** MUST 通过 ProviderCallContext 中注入的 auth（由 Facade 从 Rust 取来）作为 `x-api-key` header，**不能**直接持有明文 key

#### Scenario: base_url 可配置
- **WHEN** cfg.base_url 存在
- **THEN** MUST 使用该 URL 作为请求前缀（用于自部署/代理）

#### Scenario: cfg.base_url 缺失
- **WHEN** cfg.base_url 为空
- **THEN** MUST 使用 `https://api.anthropic.com` 作为默认

### Requirement: Anthropic SSE 事件归一化
系统 SHALL 将 Anthropic SSE 事件解析为 `UnifiedDelta`。

#### Scenario: message_start 事件
- **WHEN** 收到 `event: message_start` 数据
- **THEN** MUST 提取 message 元信息，不 yield delta（内部状态）

#### Scenario: content_block_start + delta (text)
- **WHEN** 收到 text 类型的 content_block_delta
- **THEN** MUST yield `{ type: 'text', delta: <text> }`

#### Scenario: content_block_start + delta (thinking)
- **WHEN** 收到 thinking 类型的 content_block_delta
- **THEN** MUST yield `{ type: 'thinking', delta: <text> }`

#### Scenario: message_delta + usage
- **WHEN** 收到 message_delta 事件
- **THEN** MUST 提取 usage 并 yield `{ type: 'usage', input_tokens, output_tokens, cached? }`

#### Scenario: message_stop
- **WHEN** 收到 message_stop 事件
- **THEN** MUST yield `{ type: 'stop', reason: 'end_turn' }` 并结束 AsyncIterable

#### Scenario: error 事件
- **WHEN** 收到 error 事件
- **THEN** MUST yield `{ type: 'error', recoverable: <根据 error type>, code: <error type>, message: <message> }`

### Requirement: Anthropic ping 实现
系统 SHALL 在 anthropicProvider 实现 `ping(cfg, abort)`。

#### Scenario: ping 成功
- **WHEN** 发送极小请求（max_tokens=1, prompt="hi"）并收到响应
- **THEN** MUST 返回 `{ ok: true }`

#### Scenario: ping 401
- **WHEN** API key 无效
- **THEN** MUST 返回 `{ ok: false, reason: 'unauthorized' }`

#### Scenario: ping 网络错误
- **WHEN** 网络不可达或超时
- **THEN** MUST 返回 `{ ok: false, reason: 'network_error: <detail>' }`

### Requirement: Anthropic estimateCost 实现
系统 SHALL 在 anthropicProvider 实现 `estimateCost(req, cfg)`。

#### Scenario: 估算基于 max_tokens 与 model 定价
- **WHEN** cfg.model='claude-fable-5' 且 req 包含消息
- **THEN** MUST 按 Anthropic 官方定价表估算 input + output，返回 `{ input_usd, output_usd_max }`

#### Scenario: 未知 model
- **WHEN** cfg.model 不在定价表
- **THEN** MUST 返回 `{ input_usd: 0, output_usd_max: 0 }` 并打 WARN 日志

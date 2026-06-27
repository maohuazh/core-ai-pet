## ADDED Requirements

### Requirement: llm.message trigger 触发 invoke
系统 SHALL 在 triggerHandler 中处理 `llm.message` trigger_key。

#### Scenario: 触发 llm.message
- **WHEN** 外部事件 `llm.message` 被发射到 eventBus，payload 含 `{ role, messages }`
- **THEN** triggerHandler MUST 调用 `llm.invoke(role, { messages, stream: true })`

#### Scenario: invoke 失败
- **WHEN** invoke 抛出异常
- **THEN** MUST emit `llm.error` 事件到 eventBus，并 toast 提示

### Requirement: llm.invoke trigger 同步调用
系统 SHALL 支持 `llm.invoke` trigger_key（同步返回非流式结果）。

#### Scenario: 触发 llm.invoke
- **WHEN** 外部事件 `llm.invoke` 被发射，payload 含 `{ role, req }`
- **THEN** MUST 调用 `llm.invoke(role, req)`，收集所有 delta 为完整响应，emit `llm.response`

#### Scenario: M1 暂不实现（占位）
- **WHEN** M1 阶段触发 llm.invoke
- **THEN** MUST 返回 `not_implemented` 错误，emit `llm.error`

### Requirement: llm.stream 事件透传
系统 SHALL 将 Provider 的 delta 流 emit 到 eventBus。

#### Scenario: 每个 delta emit
- **WHEN** Facade 收到一个 UnifiedDelta
- **THEN** MUST emit `llm.stream` 事件到 eventBus，payload 含 `{ turn_id, delta }`

#### Scenario: 流结束
- **WHEN** Facade 流结束
- **THEN** MUST emit `llm.done` 事件，payload 含 `{ turn_id, usage? }`

### Requirement: Chat 浮层占位
系统 SHALL 在 `App.vue` 渲染一个 400×300 的浮层，消费 `llm.stream` 事件。

#### Scenario: 浮层接收 delta
- **WHEN** eventBus 收到 llm.stream 且 delta.type='text'
- **THEN** 浮层 MUST 追加 delta.delta 到显示文本

#### Scenario: 浮层滚动
- **WHEN** 文本超过浮层高度
- **THEN** MUST 自动滚动到底部（除非用户手动上滚）

#### Scenario: 浮层关闭
- **WHEN** 用户点击浮层关闭按钮
- **THEN** MUST 隐藏浮层（不销毁，下次触发再显示）

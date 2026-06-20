## ADDED Requirements

### Requirement: Event type definition
系统 SHALL 定义统一的事件结构，包含事件类型、来源、时间戳和 payload。

#### Scenario: Event structure
- **WHEN** 创建事件对象
- **THEN** 事件 MUST 包含 type (string)、source (string)、timestamp (integer)、payload (any) 字段

### Requirement: Publish event
系统 SHALL 提供 API 发布事件到事件总线。

#### Scenario: Publish to backend event bus
- **WHEN** 调用 emit_event(event_type, payload) API
- **THEN** 事件 MUST 被发布到后端 EventBus
- **THEN** 所有订阅该事件类型的处理器 MUST 收到事件

#### Scenario: Publish from frontend
- **WHEN** 前端调用 publishEvent(event_type, payload)
- **THEN** 事件 MUST 通过 Tauri 事件系统传递到后端
- **THEN** 后端事件处理器 MUST 收到事件

### Requirement: Subscribe to event
系统 SHALL 提供 API 订阅特定类型的事件。

#### Scenario: Subscribe from backend
- **WHEN** 后端模块调用 event_bus.subscribe(event_type)
- **THEN** 该模块 MUST 收到所有匹配类型的事件

#### Scenario: Subscribe from frontend
- **WHEN** 前端调用 subscribeEvent(event_type, callback)
- **THEN** callback MUST 在收到匹配类型的事件时被调用

### Requirement: Unsubscribe from event
系统 SHALL 提供 API 取消事件订阅。

#### Scenario: Unsubscribe handler
- **WHEN** 调用 unsubscribe(handler_id) API
- **THEN** 该处理器 MUST 不再收到事件通知

### Requirement: Event type enumeration
系统 SHALL 预定义一组标准事件类型。

#### Scenario: Standard event types
- **WHEN** 编译系统代码
- **THEN** 事件类型枚举 MUST 包含：UserChat、LLMResponse、EmailReceived、SlackMessage、JiraUpdated、MeetingStarted、TaskCompleted、StateChanged

### Requirement: Event deduplication
系统 SHALL 对短时间内重复事件进行去重处理。

#### Scenario: Deduplicate identical events
- **WHEN** 100ms 内收到两个相同类型且 payload 相同的事件
- **THEN** 系统 MUST 仅处理第一个事件
- **THEN** 第二个事件 MUST 被丢弃并记录日志

### Requirement: Event logging
系统 SHALL 对所有发布的事件进行日志记录。

#### Scenario: Log published events
- **WHEN** 事件被发布到 EventBus
- **THEN** 系统 MUST 记录事件类型、来源、时间戳
- **THEN** 日志级别为 DEBUG

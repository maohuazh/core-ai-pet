## MODIFIED Requirements

### Requirement: External event listener
The system SHALL provide a `triggerHandler` module that listens for `external-event` events on the eventBus and maps them to trigger keys. **M1 新增 `llm.message` / `llm.invoke` / `llm.stream` 三个 trigger_key 的处理分支。**

#### Scenario: Receive external event
- **WHEN** an `external-event` is emitted on the eventBus with type 'chat-message-received'
- **THEN** the triggerHandler SHALL map it to trigger_key 'new_message'

#### Scenario: Receive unknown event type
- **WHEN** an `external-event` is emitted with an unrecognized type
- **THEN** the triggerHandler SHALL ignore the event and not trigger any action

#### Scenario: Receive llm.message event
- **WHEN** an `external-event` is emitted with type 'llm.message' and payload `{ role, messages }`
- **THEN** the triggerHandler SHALL map it to trigger_key 'llm.message' and invoke `llm.invoke(role, { messages, stream: true })`

#### Scenario: Receive llm.invoke event
- **WHEN** an `external-event` is emitted with type 'llm.invoke' and payload `{ role, req }`
- **THEN** the triggerHandler SHALL map it to trigger_key 'llm.invoke' and invoke `llm.invoke(role, req)` (M1 返回 not_implemented)

#### Scenario: llm.stream 内部 emit
- **WHEN** Facade 收到每个 UnifiedDelta
- **THEN** the triggerHandler SHALL emit `llm.stream` event 到 eventBus，供 Chat 浮层消费

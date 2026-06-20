## ADDED Requirements

### Requirement: State enumeration
系统 SHALL 定义一组角色状态枚举，包括：Idle、Walking、Thinking、Talking、Working、Meeting、Sleeping、Alert。

#### Scenario: State enum completeness
- **WHEN** 编译系统代码
- **THEN** 状态枚举 MUST 包含全部 8 个状态值
- **THEN** 每个状态值 MUST 有唯一标识符

### Requirement: State machine initialization
状态机 SHALL 在应用启动时初始化为 Idle 状态。

#### Scenario: Default state on startup
- **WHEN** 应用首次启动
- **THEN** 状态机当前状态 MUST 为 Idle

#### Scenario: Restore state from persistence
- **WHEN** 应用启动且存在持久化的状态数据
- **THEN** 状态机 MUST 恢复到上次保存的状态

### Requirement: State transition via event
状态机 SHALL 根据事件触发状态转换，遵循预定义的转换规则。

#### Scenario: Valid state transition
- **WHEN** 当前状态为 Idle 且收到 UserChat 事件
- **THEN** 状态 MUST 转换为 Thinking

#### Scenario: Thinking to Talking transition
- **WHEN** 当前状态为 Thinking 且收到 LLMResponse 事件
- **THEN** 状态 MUST 转换为 Talking

#### Scenario: Talking to Idle transition
- **WHEN** 当前状态为 Talking 且经过 3 秒无新事件
- **THEN** 状态 MUST 自动转换为 Idle

#### Scenario: Invalid state transition
- **WHEN** 当前状态为 Sleeping 且收到 UserChat 事件
- **THEN** 状态 MUST 保持 Sleeping 不变
- **THEN** 系统 SHALL 记录警告日志

### Requirement: Get current state
系统 SHALL 提供 API 获取当前状态。

#### Scenario: Get state returns current value
- **WHEN** 调用 get_state() API
- **THEN** 返回当前状态机所处的状态值

### Requirement: Force state transition
系统 SHALL 提供 API 强制设置状态（用于调试和特殊场景）。

#### Scenario: Force set state
- **WHEN** 调用 set_state(Alert) API
- **THEN** 状态机 MUST 立即切换到 Alert 状态
- **THEN** 不受转换规则约束

### Requirement: State change notification
状态变化时系统 SHALL 发布 StateChanged 事件。

#### Scenario: Notify on state change
- **WHEN** 状态从 Idle 转换为 Thinking
- **THEN** 系统 MUST 发布 StateChanged 事件
- **THEN** 事件 payload 包含 old_state 和 new_state

### Requirement: Minimum state duration
状态机 SHALL 支持为状态设置最小持续时间，在时间内不允许转换。

#### Scenario: Enforce minimum duration
- **WHEN** 进入 Thinking 状态且设置最小持续时间为 1 秒
- **THEN** 在 1 秒内收到转换事件时 MUST 忽略该事件
- **THEN** 1 秒后正常处理转换事件

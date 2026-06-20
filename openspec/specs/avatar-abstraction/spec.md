## ADDED Requirements

### Requirement: Avatar interface definition
系统 SHALL 定义 Avatar 接口，统一角色表现层的行为。

#### Scenario: Interface methods
- **WHEN** 定义 Avatar 接口
- **THEN** 接口 MUST 包含以下方法：
  - speak(text: string): Promise<void>
  - think(): Promise<void>
  - work(): Promise<void>
  - playMotion(group: string, index?: number): Promise<void>
  - playExpression(nameOrIndex: string | number): Promise<void>
  - getState(): PetState

#### Scenario: Interface type safety
- **WHEN** 实现 Avatar 接口
- **THEN** TypeScript 编译器 MUST 确保所有方法都被实现
- **THEN** 方法签名 MUST 与接口定义匹配

### Requirement: Live2DAvatar implementation
系统 SHALL 提供 Live2DAvatar 类实现 Avatar 接口。

#### Scenario: Live2DAvatar construction
- **WHEN** 创建 Live2DAvatar 实例
- **THEN** 构造函数 MUST 接收 Live2DRenderer 实例作为参数

#### Scenario: speak method
- **WHEN** 调用 avatar.speak(text)
- **THEN** Live2DRenderer MUST 播放 Talking 动画组
- **THEN** 返回 Promise 在动画开始时 resolve

#### Scenario: think method
- **WHEN** 调用 avatar.think()
- **THEN** Live2DRenderer MUST 播放 Thinking 动画组
- **THEN** 若模型无 Thinking 组，MUST 静默失败

#### Scenario: work method
- **WHEN** 调用 avatar.work()
- **THEN** Live2DRenderer MUST 播放 Working 动画组
- **THEN** 若模型无 Working 组，MUST 使用 Idle 组替代

### Requirement: Avatar state tracking
Avatar SHALL 跟踪当前状态。

#### Scenario: Get current state
- **WHEN** 调用 avatar.getState()
- **THEN** 返回当前 Avatar 所处的状态

#### Scenario: State update on action
- **WHEN** 调用 avatar.speak()
- **THEN** Avatar 内部状态 MUST 更新为 Talking

#### Scenario: State update on think
- **WHEN** 调用 avatar.think()
- **THEN** Avatar 内部状态 MUST 更新为 Thinking

### Requirement: Avatar factory pattern
系统 SHALL 提供工厂函数根据配置创建对应的 Avatar 实例。

#### Scenario: Create Live2D avatar
- **WHEN** 调用 createAvatar("live2d", renderer)
- **THEN** 返回 Live2DAvatar 实例

#### Scenario: Future Pixel avatar
- **WHEN** 调用 createAvatar("pixel", renderer)
- **THEN** 返回 PixelAvatar 实例（后续实现）

#### Scenario: Unknown avatar type
- **WHEN** 调用 createAvatar("unknown", renderer)
- **THEN** MUST 抛出错误 "Unknown avatar type: unknown"

### Requirement: Avatar integration with state machine
Avatar SHALL 与状态机集成，根据状态变化自动切换表现。

#### Scenario: State change triggers avatar update
- **WHEN** 状态机状态变为 Thinking
- **THEN** Avatar.think() MUST 被自动调用

#### Scenario: State change to Talking
- **WHEN** 状态机状态变为 Talking
- **THEN** Avatar.speak() MUST 被自动调用（传入空文本）

#### Scenario: State change to Idle
- **WHEN** 状态机状态变为 Idle
- **THEN** Avatar.playMotion("Idle", 0) MUST 被自动调用

### Requirement: Avatar destruction
Avatar SHALL 提供销毁方法释放资源。

#### Scenario: Destroy avatar
- **WHEN** 调用 avatar.destroy()
- **THEN** Avatar MUST 清理内部资源
- **THEN** 后续调用其他方法 MUST 抛出错误

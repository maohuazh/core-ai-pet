## ADDED Requirements

### Requirement: State-driven animation switching
Live2DRenderer SHALL 支持根据状态机状态自动切换动画。

#### Scenario: Switch to Thinking animation
- **WHEN** 状态机状态变为 Thinking
- **THEN** Live2DRenderer MUST 播放 Thinking 动画组
- **THEN** 若模型无 Thinking 组，MUST 使用 Idle 组替代

#### Scenario: Switch to Talking animation
- **WHEN** 状态机状态变为 Talking
- **THEN** Live2DRenderer MUST 播放 Talking 动画组

#### Scenario: Switch to Idle animation
- **WHEN** 状态机状态变为 Idle
- **THEN** Live2DRenderer MUST 播放 Idle 动画组

#### Scenario: State animation mapping
- **WHEN** 状态机状态变化
- **THEN** 系统 MUST 通过 Avatar 接口驱动动画切换
- **THEN** 不直接调用 Live2DRenderer 的 playMotion

### Requirement: Animation group fallback
系统 SHALL 在模型缺少特定动画组时提供降级策略。

#### Scenario: Missing animation group fallback
- **WHEN** 需要播放 Thinking 动画但模型无该组
- **THEN** 系统 MUST 尝试播放 Idle 组
- **THEN** 若 Idle 也不存在，MUST 静默失败

#### Scenario: Fallback chain
- **WHEN** 需要的动画组不存在
- **THEN** 降级顺序为：目标组 → Idle → 第一可用组 → 静默失败
- **THEN** 每次降级 MUST 记录警告日志

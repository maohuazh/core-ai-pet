## ADDED Requirements

### Requirement: Avatar 接口新增方向与缩放方法
`IAvatar` 接口 SHALL 新增 `setDirection(degree: number)` 和 `setScale(scale: number)` 方法。

#### Scenario: setDirection 方法
- **WHEN** 调用 `avatar.setDirection(180)`
- **THEN** Avatar 实现 MUST 将角度映射到 8 方向之一
- **THEN** 动画 MUST 切换到对应方向的帧

#### Scenario: setScale 方法
- **WHEN** 调用 `avatar.setScale(2.0)`
- **THEN** Avatar 实现 MUST 更新显示缩放
- **THEN** 渲染器 MUST 按新缩放重绘

#### Scenario: Live2DAvatar 兼容
- **WHEN** 调用 Live2DAvatar 的 `setDirection` 或 `setScale`
- **THEN** Live2DAvatar MUST 提供空实现或适配（Live2D 模型可能不支持方向切换）
- **THEN** 方法 MUST 不抛出异常

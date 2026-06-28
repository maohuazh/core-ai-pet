## ADDED Requirements

### Requirement: SpriteSheet 渲染后端支持
渲染管线 SHALL 支持 SpriteSheetRenderer 作为第二渲染后端，与 Live2DRenderer 并存。

#### Scenario: 渲染器类型选择
- **WHEN** 用户选择 Sprite 类型模型
- **THEN** 系统 SHALL 创建 `SpriteSheetRenderer` 实例
- **THEN** 系统 SHALL 不再创建 `Live2DRenderer`

#### Scenario: 渲染器切换
- **WHEN** 从 Live2D 模型切换到 Sprite 模型
- **THEN** 系统 SHALL 销毁 `Live2DRenderer` 并创建 `SpriteSheetRenderer`
- **THEN** Canvas 容器 MUST 保持相同的 DOM 挂载点

#### Scenario: 渲染器共存（不同实例）
- **WHEN** 应用中同时存在 Live2D 和 Sprite 模型配置
- **THEN** 系统 SHALL 按需创建对应的渲染器实例
- **THEN** 同一时刻 MUST 只有一个渲染器处于活跃状态（当前显示的模型）

## ADDED Requirements

### Requirement: Generic motion trigger interface
`Live2DRenderer` SHALL 提供 `playMotion(group: string, index?: number): Promise<void>` 方法，按动画组名和可选索引触发模型动画。

#### Scenario: Play specific motion
- **WHEN** 调用 `renderer.playMotion("TapBody", 0)`
- **THEN** 当前加载的模型播放 TapBody 组的第 0 个动画

#### Scenario: Play motion with random index
- **WHEN** 调用 `renderer.playMotion("TapBody")` 且不传 index
- **THEN** 从该动画组中随机选择一个动画播放

#### Scenario: Play motion on invalid group
- **WHEN** 调用 `renderer.playMotion("NonExistentGroup", 0)`
- **THEN** 方法 MUST 静默失败（打印警告日志），不抛出异常

### Requirement: Generic expression trigger interface
`Live2DRenderer` SHALL 提供 `playExpression(nameOrIndex: string | number): Promise<void>` 方法，按名称或索引触发模型表情。

#### Scenario: Play expression by name
- **WHEN** 调用 `renderer.playExpression("Smile")`
- **THEN** 当前模型切换到 Smile 表情

#### Scenario: Play expression by index
- **WHEN** 调用 `renderer.playExpression(0)`
- **THEN** 当前模型切换到第 0 个表情

#### Scenario: Play expression when model has no expressions
- **WHEN** 调用 `renderer.playExpression("Angry")` 且当前模型（如 Hiyori）无表情定义
- **THEN** 方法 MUST 静默失败（打印警告日志），不抛出异常

### Requirement: Model capability auto-discovery
`Live2DRenderer` SHALL 在模型加载完成后自动解析模型能力，提供 `getMotionGroups(): MotionGroup[]` 和 `getExpressions(): ExpressionInfo[]` 方法。

#### Scenario: Discover motion groups after loading
- **WHEN** 模型加载完成并调用 `renderer.getMotionGroups()`
- **THEN** 返回数组包含所有可用动画组，每项含 `name`(string) 和 `count`(number) 字段

#### Scenario: Discover expressions after loading
- **WHEN** Mao 模型加载完成并调用 `renderer.getExpressions()`
- **THEN** 返回包含 8 个表情的数组，每项含 `name`(string) 字段

#### Scenario: Discover expressions for model without expressions
- **WHEN** Hiyori 模型加载完成并调用 `renderer.getExpressions()`
- **THEN** 返回空数组

### Requirement: Hot model replacement
`Live2DRenderer.loadModel(url)` SHALL 在已有模型加载的情况下，先销毁旧模型再加载新模型，保持 PixiJS Application 实例不变。

#### Scenario: Switch model without recreating renderer
- **WHEN** 已加载 Haru 模型后调用 `loadModel(newUrl)` 加载 Hiyori
- **THEN** Haru 模型从 stage 移除并销毁，Hiyori 模型加载到同一个 stage，PixiJS Application 实例不变

### Requirement: Auto-play default motion after loading
模型加载完成后，Renderer SHALL 自动播放该模型的第一个 idle 动画，而非硬编码特定动画组名。

#### Scenario: Auto-play idle for different models
- **WHEN** 任意模型加载完成
- **THEN** Renderer MUST 自动查找并播放包含 "idle"（不区分大小写）的动画组的第一个动画

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

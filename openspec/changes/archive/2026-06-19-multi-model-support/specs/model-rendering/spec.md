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

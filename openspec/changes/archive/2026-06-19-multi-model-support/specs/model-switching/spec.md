## ADDED Requirements

### Requirement: PetStore reactive state
系统 SHALL 提供 `PetStore`（基于 Vue reactive），持有 `currentModel`(PetModelConfig) 和 `models`(PetModelConfig[]) 状态。

#### Scenario: Initial state
- **WHEN** 应用启动并初始化 PetStore
- **THEN** `currentModel` MUST 为注册表的默认模型，`models` MUST 包含所有已注册模型

#### Scenario: Update current model
- **WHEN** 调用 `petStore.setCurrentModel(newModel)`
- **THEN** `currentModel` 更新为新模型，响应式订阅者收到通知

### Requirement: Model switching UI entry
`PetHoverMenu` SHALL 新增"切换模型"菜单项，用户点击后切换到注册表中的下一个模型。

#### Scenario: Click switch model button
- **WHEN** 当前模型为 Hiyori，用户点击"切换模型"
- **THEN** 模型切换为 Mao（注册表中 Hiyori 的下一个）

#### Scenario: Cycle through all models
- **WHEN** 当前模型为注册表中最后一个，用户点击"切换模型"
- **THEN** 模型切换为注册表中的第一个模型（循环）

### Requirement: Model switch triggers renderer update
当 PetStore 中的 `currentModel` 发生变化时，Live2DCanvas 组件 SHALL 自动调用 `renderer.loadModel()` 加载新模型。

#### Scenario: Store change triggers model load
- **WHEN** `petStore.setCurrentModel(natoriModel)` 被调用
- **THEN** Live2DCanvas MUST 调用 `renderer.loadModel(natoriModel.modelUrl)` 加载 Natori 模型

#### Scenario: Model switch shows console feedback
- **WHEN** 模型切换完成
- **THEN** 控制台 MUST 输出包含新模型名称的日志信息

### Requirement: Model switch preserves window state
切换模型时，窗口位置、大小、拖拽功能 SHALL 保持不变，不重启不闪烁。

#### Scenario: Switch model while window is dragged
- **WHEN** 用户将窗口拖到屏幕边缘后切换模型
- **THEN** 窗口位置保持不变，新模型正常渲染

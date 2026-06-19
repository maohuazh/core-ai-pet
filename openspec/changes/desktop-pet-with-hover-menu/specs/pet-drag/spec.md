## ADDED Requirements

### Requirement: Drag pet window on model area
系统 SHALL 允许用户通过鼠标拖拽移动宠物窗口，但仅在模型不透明区域生效。

#### Scenario: Start dragging on model
- **WHEN** 用户在 Live2D 模型所在的像素区域按下鼠标左键
- **THEN** 系统 SHALL 进入拖拽模式
- **THEN** 鼠标移动时 SHALL 同步移动窗口位置

#### Scenario: Drag window
- **WHEN** 用户按住鼠标左键并移动
- **THEN** 宠物窗口 SHALL 跟随鼠标移动
- **THEN** 窗口位置 SHALL 实时更新

#### Scenario: Stop dragging
- **WHEN** 用户释放鼠标左键
- **THEN** 系统 SHALL 退出拖拽模式
- **THEN** 窗口 SHALL 保持在释放位置

### Requirement: No drag on transparent area
系统 SHALL 不在窗口透明区域触发拖拽，使点击事件传递到下层。

#### Scenario: Click on transparent area
- **WHEN** 用户在窗口中无模型像素的区域按下鼠标左键
- **THEN** 系统 SHALL 不进入拖拽模式
- **THEN** 点击事件 SHALL 传递到下层的其他应用程序

### Requirement: Drag implementation via Tauri
系统 SHALL 使用 Tauri 的 window.startDragging() API 实现拖拽。

#### Scenario: Invoke start_dragging
- **WHEN** 用户在模型区域按下鼠标左键
- **THEN** Vue 组件 SHALL 通过 invoke('start_dragging') 调用 Rust 后端
- **THEN** Rust 后端 SHALL 调用 window.start_dragging() 触发原生拖拽

#### Scenario: Detect model pixel area
- **WHEN** 需要判断点击是否在模型区域
- **THEN** 系统 SHALL 检查鼠标事件的目标元素
- **THEN** 仅当目标元素是 PixiJS 的 Canvas 时，才触发拖拽

### Requirement: Drag state management
系统 SHALL 管理拖拽状态，提供启用/禁用拖拽的 API。

#### Scenario: Enable/disable drag
- **WHEN** 调用 set_drag_enabled(true/false) API
- **THEN** 系统 SHALL 启用或禁用拖拽功能

#### Scenario: Check drag state
- **WHEN** 调用 is_dragging() API
- **THEN** 系统 SHALL 返回当前是否正在拖拽

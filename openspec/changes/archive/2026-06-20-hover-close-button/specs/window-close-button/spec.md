## ADDED Requirements

### Requirement: Close button position and appearance
系统 SHALL 在显示区域右上角固定位置渲染一个透明的 × 关闭按钮。

#### Scenario: Button default state
- **WHEN** 鼠标悬停在模型上
- **THEN** 右上角显示 × 按钮
- **THEN** 按钮默认 opacity 为 0.3，尺寸 28x28 像素

#### Scenario: Button hover state
- **WHEN** 鼠标悬停在 × 按钮上
- **THEN** 按钮 opacity 变为 1
- **THEN** 按钮背景变为红色半透明

### Requirement: Close button visibility tied to hover
关闭按钮 SHALL 与悬浮菜单同步显示和隐藏。

#### Scenario: Button appears on hover
- **WHEN** 鼠标进入 pet-container 区域
- **THEN** 关闭按钮与悬浮菜单同时显示

#### Scenario: Button disappears on leave
- **WHEN** 鼠标离开 pet-container 区域
- **THEN** 关闭按钮与悬浮菜单同时隐藏

### Requirement: Click closes application
点击关闭按钮 SHALL 退出桌面宠物程序。

#### Scenario: Click close button
- **WHEN** 用户点击 × 按钮
- **THEN** 系统调用 Tauri window close API 关闭主窗口
- **THEN** 应用程序退出

### Requirement: Close button does not interfere with model interaction
关闭按钮 SHALL 不阻止鼠标事件传递到模型区域（除按钮自身区域外）。

#### Scenario: Click outside button on model
- **WHEN** 用户在模型区域点击但不在 × 按钮上
- **THEN** 点击事件 SHALL 正常传递到 Live2D 模型（触发拖拽等）

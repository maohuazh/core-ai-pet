## ADDED Requirements

### Requirement: Transparent borderless window
系统 SHALL 创建一个无边框、透明背景的窗口，用于显示 Live2D 宠物。

#### Scenario: Window creation with transparency
- **WHEN** 应用启动
- **THEN** 系统创建一个无边框（decorations: false）、透明背景（transparent: true）的窗口
- **THEN** 窗口尺寸 SHALL 为 400x400 像素
- **THEN** 窗口初始位置 SHALL 居中显示在屏幕中央

### Requirement: Always on top
系统 SHALL 使宠物窗口始终置顶显示，不被其他窗口遮挡。

#### Scenario: Window stays on top
- **WHEN** 用户点击其他应用程序窗口
- **THEN** 宠物窗口 SHALL 始终保持在最上层
- **THEN** 宠物窗口 SHALL 不会获得键盘焦点

### Requirement: Skip taskbar
系统 SHALL 使宠物窗口不出现在任务栏中。

#### Scenario: Window not in taskbar
- **WHEN** 宠物窗口创建
- **THEN** 任务栏 SHALL 不显示宠物窗口的图标
- **THEN** Alt+Tab 切换时 SHALL 不显示宠物窗口

### Requirement: Mouse click-through on transparent areas
系统 SHALL 在窗口透明区域实现鼠标穿透，使点击事件传递到下层窗口。

#### Scenario: Click passes through transparent area
- **WHEN** 用户点击窗口中完全透明的区域（无 Live2D 模型像素）
- **THEN** 点击事件 SHALL 传递到下层的其他应用程序窗口
- **THEN** 宠物窗口 SHALL 不响应此点击

#### Scenario: Click on model area is captured
- **WHEN** 用户点击窗口中 Live2D 模型所在的区域
- **THEN** 宠物窗口 SHALL 捕获此点击事件
- **THEN** 下层窗口 SHALL 不接收此点击

### Requirement: Window position control
系统 SHALL 提供 API 控制宠物窗口的位置。

#### Scenario: Move window programmatically
- **WHEN** 调用 set_window_position(x, y) API
- **THEN** 宠物窗口 SHALL 移动到指定的屏幕坐标 (x, y)

#### Scenario: Get window position
- **WHEN** 调用 get_window_position() API
- **THEN** 系统 SHALL 返回当前窗口的屏幕坐标 (x, y)

## Purpose

设置面板窗口的创建、属性、关闭行为和自定义标题栏。

## Requirements

### Requirement: Settings window creation
系统 SHALL 提供 Tauri 命令创建设置面板窗口。窗口 MUST 为独立窗口，加载 `/settings` 路由。

#### Scenario: Create settings window
- **WHEN** 前端调用 invoke("open_settings_window")
- **THEN** 系统 MUST 创建名为 "settings" 的 Tauri 窗口
- **THEN** 窗口尺寸 MUST 为 680x720 像素
- **THEN** 窗口最小尺寸 MUST 为 560x480 像素
- **THEN** 窗口 MUST 无边框（decorations: false）
- **THEN** 窗口 MUST 支持透明背景

#### Scenario: Window already exists
- **WHEN** 设置窗口已存在且被调用打开
- **THEN** 系统 MUST 聚焦已有窗口而非创建新窗口

### Requirement: Settings window properties
设置窗口 SHALL 具备特定的视觉和行为属性。

#### Scenario: Window background
- **WHEN** 设置窗口显示
- **THEN** 窗口背景 MUST 为半透明毛玻璃效果
- **THEN** 背景色 MUST 为 rgba(255, 255, 255, 0.75)
- **THEN** MUST 应用 backdrop-filter: blur(20px)

#### Scenario: Window border and shadow
- **WHEN** 设置窗口显示
- **THEN** 窗口 MUST 有 16px 圆角
- **THEN** 窗口 MUST 有外阴影 0 8px 32px rgba(0, 0, 0, 0.12)

#### Scenario: Window always on top
- **WHEN** 设置窗口显示
- **THEN** 窗口默认 MUST NOT 置顶
- **THEN** 窗口 MUST 可调整大小

### Requirement: Settings window close behavior
设置窗口关闭时 SHALL 隐藏而非销毁。

#### Scenario: Close settings window via X button
- **WHEN** 用户点击设置窗口标题栏的 X 关闭按钮
- **THEN** 窗口 MUST 隐藏（hide）而非销毁
- **THEN** 窗口状态和数据 MUST 保留
- **THEN** 窗口 MUST 从屏幕消失（用户可见窗口消失）

#### Scenario: Reopen settings window
- **WHEN** 设置窗口已隐藏且用户再次打开
- **THEN** 窗口 MUST 显示（show）已有实例
- **THEN** 窗口 MUST 保留之前的状态和数据

### Requirement: Custom title bar
设置窗口 SHALL 提供自定义标题栏支持拖拽和关闭。

#### Scenario: Title bar drag
- **WHEN** 用户在标题栏区域按下鼠标并拖动
- **THEN** 窗口 MUST 跟随鼠标移动

#### Scenario: Title bar close button
- **WHEN** 用户点击标题栏的关闭按钮（X）
- **THEN** 窗口 MUST 立即隐藏
- **THEN** 应用 MUST 不退出

#### Scenario: Title bar minimize button
- **WHEN** 用户点击标题栏的最小化按钮
- **THEN** 窗口 MUST 最小化到任务栏

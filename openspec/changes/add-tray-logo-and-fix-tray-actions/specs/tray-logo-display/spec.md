## ADDED Requirements

### Requirement: Logo asset integration
系统 SHALL 将 AI 宠物 logo 图片集成到项目资源中。

#### Scenario: Logo file exists
- **WHEN** 项目构建
- **THEN** `src-tauri/icons/` 目录 SHALL 包含 logo PNG 文件
- **THEN** logo 文件 SHALL 支持透明度

#### Scenario: Multiple resolution assets
- **WHEN** logo 资源添加到项目
- **THEN** 系统 SHALL 提供多种分辨率版本（至少 32x32, 128x128, 256x256）
- **THEN** 高分辨率版本 SHALL 用于高 DPI 显示器

### Requirement: Tray icon uses logo
系统 SHALL 在系统托盘显示自定义 logo 图标。

#### Scenario: Application startup
- **WHEN** 应用启动完成
- **THEN** 系统托盘图标 SHALL 显示 AI 宠物 logo
- **THEN** 托盘图标 SHALL 保持清晰可辨识

#### Scenario: Minimize to tray
- **WHEN** 用户最小化应用窗口到托盘
- **THEN** 托盘图标 SHALL 持续显示 logo
- **THEN** 图标 SHALL 不消失或变为默认图标

### Requirement: Tray tooltip
系统 SHALL 为托盘 logo 图标设置悬停提示文字。

#### Scenario: Hover over tray logo
- **WHEN** 鼠标悬停在托盘图标上
- **THEN** 系统 SHALL 显示提示文字 "Core AI Pet"

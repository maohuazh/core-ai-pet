## ADDED Requirements

### Requirement: System tray icon
系统 SHALL 在系统托盘区域显示应用图标。

#### Scenario: Tray icon on startup
- **WHEN** 应用启动
- **THEN** 系统 SHALL 在系统托盘区域显示应用图标
- **THEN** 托盘图标 SHALL 使用应用 icon

### Requirement: Tray context menu
系统 SHALL 为托盘图标提供右键上下文菜单，菜单项功能 SHALL 正常工作。

#### Scenario: Right-click tray icon
- **WHEN** 用户右键点击系统托盘图标
- **THEN** 系统 SHALL 显示上下文菜单
- **THEN** 菜单 SHALL 包含"显示窗口"选项
- **THEN** 菜单 SHALL 包含"退出"选项

#### Scenario: Click show window
- **WHEN** 用户点击"显示窗口"菜单项
- **THEN** 宠物窗口 SHALL 显示在屏幕中央（若当前隐藏）
- **THEN** 窗口 SHALL 获得焦点并置于最前

#### Scenario: Click exit
- **WHEN** 用户点击"退出"菜单项
- **THEN** 应用 SHALL 完全退出
- **THEN** 托盘图标 SHALL 消失
- **THEN** 所有窗口 SHALL 关闭

### Requirement: Tray tooltip
系统 SHALL 为托盘图标设置悬停提示文字。

#### Scenario: Hover over tray icon
- **WHEN** 鼠标悬停在托盘图标上
- **THEN** 系统 SHALL 显示提示文字 "Core AI Pet"

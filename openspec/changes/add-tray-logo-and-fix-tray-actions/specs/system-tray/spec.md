## MODIFIED Requirements

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

## ADDED Requirements

### Requirement: AppTopBar components
顶部栏 SHALL 包含 Logo（左侧）和操作按钮（右侧）。

#### Scenario: TopBar left section
- **WHEN** 顶部栏渲染
- **THEN** 左侧 MUST 显示应用 Logo（爪印图标 + "CoreAIpet" 文字）

#### Scenario: TopBar right section - not logged in
- **WHEN** 用户未登录
- **THEN** 右侧 MUST 显示放大镜按钮、铃铛按钮、灰色默认头像
- **THEN** 不显示用户名
- **THEN** 点击灰色头像 MUST 弹出登录占位窗口

#### Scenario: TopBar right section - logged in
- **WHEN** 用户已登录
- **THEN** 右侧 MUST 显示放大镜按钮、铃铛按钮、用户头像和名称
- **THEN** 点击头像 MUST 弹出下拉菜单
- **THEN** 下拉菜单 MUST 包含：个人信息、设置、登出

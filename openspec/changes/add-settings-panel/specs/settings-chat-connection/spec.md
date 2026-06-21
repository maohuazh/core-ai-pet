## ADDED Requirements

### Requirement: Chat platform list display
系统 SHALL 显示已配置的聊天工具平台列表。

#### Scenario: Display chat platforms
- **WHEN** 用户打开设置面板并切换到聊天工具模块
- **THEN** 系统 MUST 显示所有已配置的聊天工具平台
- **THEN** 每个平台 MUST 显示为卡片形式
- **THEN** 卡片 MUST 包含：平台名称、图标、连接状态、启用开关

#### Scenario: Connected platform card
- **WHEN** 聊天工具状态为 "connected"
- **THEN** 卡片 MUST 显示绿色状态指示
- **THEN** 卡片 MUST 显示连接的账号名称
- **THEN** 卡片 MUST 显示连接时间
- **THEN** 卡片 MUST 显示启用开关和"断开连接"按钮

#### Scenario: Disconnected platform card
- **WHEN** 聊天工具状态为 "disconnected"
- **THEN** 卡片 MUST 显示灰色状态指示
- **THEN** 卡片 MUST 显示"未连接"
- **THEN** 卡片 MUST 显示启用开关（禁用状态）和"授权连接"按钮

### Requirement: Chat platform actions
用户 SHALL 能够对聊天工具平台执行操作。

#### Scenario: Toggle enable/disable
- **WHEN** 用户切换聊天工具平台的启用开关
- **THEN** 系统 MUST 更新 enabled 状态
- **THEN** 系统 MUST 将新状态持久化到 SQLite

#### Scenario: Disconnect platform
- **WHEN** 用户点击"断开连接"按钮
- **THEN** 系统 MUST 弹出确认对话框
- **THEN** 用户确认后 MUST 将状态更新为 "disconnected"
- **THEN** 卡片 MUST 更新为未连接状态

#### Scenario: Authorize new connection
- **WHEN** 用户点击"授权连接"按钮
- **THEN** 系统 MUST 显示"功能开发中"Toast 提示

#### Scenario: Add new platform
- **WHEN** 用户点击"+ 添加"按钮
- **THEN** 系统 MUST 显示"功能开发中"Toast 提示

#### Scenario: Edit platform name
- **WHEN** 用户点击卡片右上角 `⋮` 菜单并选择"编辑名称"
- **THEN** 系统 MUST 弹出编辑对话框
- **THEN** 用户输入新名称后 MUST 更新并持久化

#### Scenario: Delete platform from menu
- **WHEN** 用户点击卡片右上角 `⋮` 菜单并选择"删除"
- **THEN** 系统 MUST 弹出确认对话框
- **THEN** 用户确认后 MUST 删除该平台记录

### Requirement: Chat mock data
系统 SHALL 在首次启动时初始化 Mock 数据。

#### Scenario: Initialize mock data
- **WHEN** 应用首次启动且 chat_platforms 表为空
- **THEN** 系统 MUST 插入 4 条 Mock 数据
- **THEN** Mock 数据 MUST 包含：WeChat（connected, enabled）、Slack（connected, enabled）、Microsoft Teams（disconnected, disabled）、Discord（disconnected, disabled）

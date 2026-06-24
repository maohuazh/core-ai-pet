## ADDED Requirements

### Requirement: Jira connection list display
系统 SHALL 显示已连接的 Jira 实例列表。

#### Scenario: Display Jira connections
- **WHEN** 用户打开设置面板并切换到 Jira 模块
- **THEN** 系统 MUST 显示所有已配置的 Jira 连接
- **THEN** 每个连接 MUST 显示为卡片形式
- **THEN** 卡片 MUST 包含：名称、URL、邮箱、状态、启用开关

#### Scenario: Empty state
- **WHEN** 没有配置任何 Jira 连接
- **THEN** 系统 MUST 显示空状态提示

### Requirement: Jira connection card
每个 Jira 连接 SHALL 以卡片形式展示，包含操作按钮。

#### Scenario: Connected status card
- **WHEN** Jira 连接状态为 "connected"
- **THEN** 卡片 MUST 显示绿色状态指示
- **THEN** 卡片 MUST 显示 "上次同步: YYYY-MM-DD HH:mm"
- **THEN** 卡片 MUST 显示启用开关和"撤销授权"按钮

#### Scenario: Expired status card
- **WHEN** Jira 连接状态为 "expired"
- **THEN** 卡片 MUST 显示红色状态指示
- **THEN** 卡片 MUST 显示"授权已过期"
- **THEN** 卡片 MUST 显示启用开关和"重新授权"按钮

#### Scenario: Error status card
- **WHEN** Jira 连接状态为 "error"
- **THEN** 卡片 MUST 显示红色状态指示
- **THEN** 卡片 MUST 显示错误信息

### Requirement: Jira connection actions
用户 SHALL 能够对 Jira 连接执行操作。

#### Scenario: Toggle enable/disable
- **WHEN** 用户切换 Jira 连接的启用开关
- **THEN** 系统 MUST 更新 enabled 状态
- **THEN** 系统 MUST 将新状态持久化到 SQLite

#### Scenario: Revoke authorization
- **WHEN** 用户点击"撤销授权"按钮
- **THEN** 系统 MUST 弹出确认对话框
- **THEN** 用户确认后 MUST 删除该连接记录
- **THEN** 卡片 MUST 从列表中移除

#### Scenario: Add new connection
- **WHEN** 用户点击"+ 添加"按钮
- **THEN** 系统 MUST 显示"功能开发中"Toast 提示

#### Scenario: Edit connection name
- **WHEN** 用户点击卡片右上角 `⋮` 菜单并选择"编辑名称"
- **THEN** 系统 MUST 弹出编辑对话框
- **THEN** 用户输入新名称后 MUST 更新并持久化

#### Scenario: Delete connection from menu
- **WHEN** 用户点击卡片右上角 `⋮` 菜单并选择"删除"
- **THEN** 系统 MUST 弹出确认对话框
- **THEN** 用户确认后 MUST 删除该连接记录

### Requirement: Jira mock data
系统 SHALL 在首次启动时初始化 Mock 数据。

#### Scenario: Initialize mock data
- **WHEN** 应用首次启动且 jira_connections 表为空
- **THEN** 系统 MUST 插入 2 条 Mock 数据
- **THEN** Mock 数据 MUST 包含：公司项目管理（connected, enabled）、开源项目追踪（expired, disabled）

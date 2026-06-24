## Purpose

邮箱账户列表的显示、卡片 UI、操作（切换/删除/编辑/添加）及 Mock 数据。

## Requirements

### Requirement: Email account list display
系统 SHALL 显示已连接的邮箱账户列表。

#### Scenario: Display email accounts
- **WHEN** 用户打开设置面板并切换到邮箱模块
- **THEN** 系统 MUST 显示所有已配置的邮箱账户
- **THEN** 每个账户 MUST 显示为卡片形式
- **THEN** 卡片 MUST 包含：名称、邮箱地址、提供商图标、状态、启用开关

#### Scenario: Empty state
- **WHEN** 没有配置任何邮箱账户
- **THEN** 系统 MUST 显示空状态提示

### Requirement: Email account card
每个邮箱账户 SHALL 以卡片形式展示，结构与 Jira 连接一致。

#### Scenario: Connected status card
- **WHEN** 邮箱账户状态为 "connected"
- **THEN** 卡片 MUST 显示绿色状态指示
- **THEN** 卡片 MUST 显示提供商图标（Gmail/Outlook/IMAP）
- **THEN** 卡片 MUST 显示启用开关和"撤销授权"按钮

#### Scenario: Expired status card
- **WHEN** 邮箱账户状态为 "expired"
- **THEN** 卡片 MUST 显示红色状态指示
- **THEN** 卡片 MUST 显示"授权已过期"
- **THEN** 卡片 MUST 显示启用开关和"重新授权"按钮

### Requirement: Email account actions
用户 SHALL 能够对邮箱账户执行操作。

#### Scenario: Toggle enable/disable
- **WHEN** 用户切换邮箱账户的启用开关
- **THEN** 系统 MUST 更新 enabled 状态
- **THEN** 系统 MUST 将新状态持久化到 SQLite

#### Scenario: Revoke authorization
- **WHEN** 用户点击"撤销授权"按钮
- **THEN** 系统 MUST 弹出确认对话框
- **THEN** 用户确认后 MUST 删除该账户记录

#### Scenario: Add new account
- **WHEN** 用户点击"+ 添加"按钮
- **THEN** 系统 MUST 显示"功能开发中"Toast 提示

#### Scenario: Edit account name
- **WHEN** 用户点击卡片右上角 `⋮` 菜单并选择"编辑名称"
- **THEN** 系统 MUST 弹出编辑对话框
- **THEN** 用户输入新名称后 MUST 更新并持久化

### Requirement: Email mock data
系统 SHALL 在首次启动时初始化 Mock 数据。

#### Scenario: Initialize mock data
- **WHEN** 应用首次启动且 email_accounts 表为空
- **THEN** 系统 MUST 插入 2 条 Mock 数据
- **THEN** Mock 数据 MUST 包含：工作邮箱（outlook, connected, enabled）、个人邮箱（gmail, connected, enabled）

## Purpose

设置面板相关的数据库表结构（6 张新表）、索引和 Mock 数据初始化。

## Requirements

### Requirement: Jira connections table
系统 SHALL 提供 jira_connections 表存储 Jira 连接配置。

#### Scenario: Table schema
- **WHEN** 数据库初始化
- **THEN** jira_connections 表 MUST 包含以下字段：
  - id (TEXT PRIMARY KEY) - UUID
  - name (TEXT NOT NULL) - 用户自定义显示名
  - url (TEXT NOT NULL) - Jira 实例 URL
  - email (TEXT NOT NULL) - 登录邮箱
  - api_token (TEXT) - 加密存储的 API Token
  - status (TEXT NOT NULL DEFAULT 'connected') - CHECK IN ('connected', 'expired', 'error')
  - enabled (INTEGER NOT NULL DEFAULT 1) - 0=禁用, 1=启用
  - created_at (TEXT NOT NULL) - 创建时间
  - updated_at (TEXT NOT NULL) - 更新时间
  - last_sync_at (TEXT) - 最后成功同步时间

#### Scenario: CRUD operations
- **WHEN** 前端调用相应的 Tauri IPC 命令
- **THEN** 系统 MUST 支持：
  - get_jira_connections: 获取所有连接列表
  - toggle_jira_connection: 切换启用状态
  - delete_jira_connection: 删除连接
  - update_jira_connection: 更新连接信息

### Requirement: Email accounts table
系统 SHALL 提供 email_accounts 表存储邮箱账户配置。

#### Scenario: Table schema
- **WHEN** 数据库初始化
- **THEN** email_accounts 表 MUST 包含以下字段：
  - id (TEXT PRIMARY KEY) - UUID
  - name (TEXT NOT NULL) - 用户自定义显示名
  - email (TEXT NOT NULL) - 邮箱地址
  - provider (TEXT NOT NULL DEFAULT 'imap') - CHECK IN ('gmail', 'outlook', 'imap', 'other')
  - auth_type (TEXT NOT NULL DEFAULT 'oauth2') - CHECK IN ('oauth2', 'app_password', 'imap_password')
  - auth_token (TEXT) - 加密存储的认证凭据
  - imap_host (TEXT) - IMAP 服务器地址
  - imap_port (INTEGER) - IMAP 端口
  - smtp_host (TEXT) - SMTP 服务器地址
  - smtp_port (INTEGER) - SMTP 端口
  - status (TEXT NOT NULL DEFAULT 'connected') - CHECK IN ('connected', 'expired', 'error')
  - enabled (INTEGER NOT NULL DEFAULT 1)
  - created_at (TEXT NOT NULL)
  - updated_at (TEXT NOT NULL)
  - last_sync_at (TEXT)

#### Scenario: CRUD operations
- **WHEN** 前端调用相应的 Tauri IPC 命令
- **THEN** 系统 MUST 支持：
  - get_email_accounts: 获取所有账户列表
  - toggle_email_account: 切换启用状态
  - delete_email_account: 删除账户
  - update_email_account: 更新账户信息

### Requirement: Chat platforms table
系统 SHALL 提供 chat_platforms 表存储聊天工具连接配置。

#### Scenario: Table schema
- **WHEN** 数据库初始化
- **THEN** chat_platforms 表 MUST 包含以下字段：
  - id (TEXT PRIMARY KEY) - UUID
  - name (TEXT NOT NULL) - 平台名称（WeChat/Slack/Teams/Discord）
  - icon (TEXT) - 平台图标（emoji 或图片路径）
  - status (TEXT NOT NULL DEFAULT 'disconnected') - CHECK IN ('connected', 'disconnected', 'error')
  - enabled (INTEGER NOT NULL DEFAULT 0)
  - account_name (TEXT) - 连接的账号名称
  - auth_token (TEXT) - 加密存储的认证凭据
  - connected_at (TEXT) - 连接时间
  - created_at (TEXT NOT NULL)
  - updated_at (TEXT NOT NULL)

#### Scenario: CRUD operations
- **WHEN** 前端调用相应的 Tauri IPC 命令
- **THEN** 系统 MUST 支持：
  - get_chat_platforms: 获取所有平台列表
  - toggle_chat_platform: 切换启用状态
  - disconnect_chat_platform: 断开连接
  - delete_chat_platform: 删除平台

### Requirement: Models table
系统 SHALL 提供 models 表存储模型注册信息。

#### Scenario: Table schema
- **WHEN** 数据库初始化
- **THEN** models 表 MUST 包含以下字段：
  - id (TEXT PRIMARY KEY) - UUID
  - name (TEXT NOT NULL) - 模型名称
  - type (TEXT NOT NULL DEFAULT 'live2d') - CHECK IN ('live2d', 'sprite')
  - path (TEXT NOT NULL) - 模型目录路径
  - manifest_path (TEXT) - SpriteSheet: manifest.json 相对路径
  - model3_path (TEXT) - Live2D: .model3.json 相对路径
  - thumbnail (TEXT) - 缩略图路径
  - source (TEXT NOT NULL DEFAULT 'builtin') - CHECK IN ('builtin', 'cdn', 'custom')
  - status (TEXT NOT NULL DEFAULT 'active') - CHECK IN ('active', 'inactive')
  - author (TEXT) - 作者
  - version (TEXT) - 版本
  - description (TEXT) - 描述
  - license (TEXT) - 许可证
  - sort_order (INTEGER NOT NULL DEFAULT 0) - 排序
  - created_at (TEXT NOT NULL)
  - updated_at (TEXT NOT NULL)

#### Scenario: CRUD operations
- **WHEN** 前端调用相应的 Tauri IPC 命令
- **THEN** 系统 MUST 支持：
  - get_models: 获取所有模型列表
  - set_active_model: 设置活跃模型
  - delete_model: 删除模型
  - update_model: 更新模型信息

### Requirement: Model action mappings table
系统 SHALL 提供 model_action_mappings 表存储动作/表情映射配置。

#### Scenario: Table schema
- **WHEN** 数据库初始化
- **THEN** model_action_mappings 表 MUST 包含以下字段：
  - id (TEXT PRIMARY KEY) - UUID
  - model_id (TEXT NOT NULL REFERENCES models(id) ON DELETE CASCADE)
  - trigger_key (TEXT NOT NULL) - CHECK IN ('daily_1', 'daily_2', 'daily_3', 'new_message', 'new_task', 'new_email', 'task_in_progress', 'task_completed', 'task_approaching_deadline', 'task_overdue')
  - motion_group (TEXT) - 动作分组名
  - motion_name (TEXT) - 具体动作名
  - expression_name (TEXT) - 表情名
  - effect_name (TEXT) - 特效名
  - use_default (INTEGER NOT NULL DEFAULT 0) - 是否使用模型默认值
  - created_at (TEXT NOT NULL)
  - updated_at (TEXT NOT NULL)
  - UNIQUE(model_id, trigger_key) - 每个模型每个 trigger 只能有一条映射

#### Scenario: CRUD operations
- **WHEN** 前端调用相应的 Tauri IPC 命令
- **THEN** 系统 MUST 支持：
  - get_action_mappings: 获取指定模型的所有映射
  - save_action_mapping: 保存/更新映射配置
  - delete_action_mapping: 删除映射

### Requirement: App settings table
系统 SHALL 提供 app_settings 表存储应用全局设置（KV 存储）。

#### Scenario: Table schema
- **WHEN** 数据库初始化
- **THEN** app_settings 表 MUST 包含以下字段：
  - key (TEXT PRIMARY KEY) - 设置键名
  - value (TEXT NOT NULL) - JSON 序列化的值
  - updated_at (TEXT NOT NULL) - 更新时间

#### Scenario: CRUD operations
- **WHEN** 前端调用相应的 Tauri IPC 命令
- **THEN** 系统 MUST 支持：
  - get_app_setting: 获取指定设置
  - set_app_setting: 设置指定设置
  - delete_app_setting: 删除指定设置

### Requirement: Database migration
系统 SHALL 在首次启动时自动创建所有新表并初始化 Mock 数据。

#### Scenario: Create tables on first run
- **WHEN** 应用首次启动且新表不存在
- **THEN** 系统 MUST 自动创建 jira_connections、email_accounts、chat_platforms、models、model_action_mappings、app_settings 表
- **THEN** 系统 MUST 创建所有必要的索引

#### Scenario: Initialize mock data
- **WHEN** 应用首次启动且表为空
- **THEN** 系统 MUST 插入 Jira Mock 数据（2 条）
- **THEN** 系统 MUST 插入 Email Mock 数据（2 条）
- **THEN** 系统 MUST 插入 Chat Mock 数据（4 条）

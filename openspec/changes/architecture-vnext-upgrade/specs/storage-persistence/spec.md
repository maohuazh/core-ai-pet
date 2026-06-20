## ADDED Requirements

### Requirement: SQLite database initialization
系统 SHALL 在首次启动时自动创建 SQLite 数据库文件。

#### Scenario: Create database on first run
- **WHEN** 应用首次启动且数据库文件不存在
- **THEN** 系统 MUST 在 %APPDATA%/CoreAIpet/data.db 创建数据库文件
- **THEN** 系统 MUST 自动创建所有必要的表结构

#### Scenario: Database file location
- **WHEN** 应用启动
- **THEN** 数据库文件 MUST 位于 %APPDATA%/CoreAIpet/data.db（Windows）
- **THEN** 或 ~/.config/CoreAIpet/data.db（Linux/macOS）

### Requirement: Config table
系统 SHALL 提供 config 表用于存储键值对配置。

#### Scenario: Store config value
- **WHEN** 调用 storage_set(key, value) API
- **THEN** 配置值 MUST 存储到 config 表
- **THEN** updated_at 字段 MUST 更新为当前时间戳

#### Scenario: Retrieve config value
- **WHEN** 调用 storage_get(key) API
- **THEN** 返回对应 key 的配置值
- **THEN** 若 key 不存在返回 null

#### Scenario: Config table schema
- **WHEN** 数据库初始化
- **THEN** config 表 MUST 包含 key (TEXT PRIMARY KEY)、value (TEXT)、updated_at (INTEGER) 字段

### Requirement: Chat history table
系统 SHALL 提供 chat_history 表用于存储聊天记录。

#### Scenario: Store chat message
- **WHEN** 调用 chat_store(message) API
- **THEN** 消息 MUST 存储到 chat_history 表
- **THEN** 记录 MUST 包含 role、content、timestamp 字段

#### Scenario: Retrieve chat history
- **WHEN** 调用 chat_list(limit, offset) API
- **THEN** 返回最近的聊天记录列表
- **THEN** 按时间戳倒序排列

#### Scenario: Chat history schema
- **WHEN** 数据库初始化
- **THEN** chat_history 表 MUST 包含 id (INTEGER PRIMARY KEY)、role (TEXT)、content (TEXT)、timestamp (INTEGER)、metadata (TEXT) 字段

### Requirement: Window position table
系统 SHALL 提供 window_position 表用于存储窗口位置。

#### Scenario: Save window position
- **WHEN** 窗口位置发生变化
- **THEN** 新位置 MUST 存储到 window_position 表
- **THEN** x 和 y 字段 MUST 为整数

#### Scenario: Restore window position
- **WHEN** 应用启动
- **THEN** 系统 MUST 从 window_position 表读取上次保存的位置
- **THEN** 窗口 MUST 恢复到该位置

#### Scenario: Window position schema
- **WHEN** 数据库初始化
- **THEN** window_position 表 MUST 包含 id (INTEGER PRIMARY KEY CHECK id=1)、x (INTEGER)、y (INTEGER)、updated_at (INTEGER) 字段

### Requirement: Plugin state table
系统 SHALL 提供 plugin_state 表用于存储插件状态。

#### Scenario: Store plugin state
- **WHEN** 插件状态发生变化
- **THEN** 状态 MUST 存储到 plugin_state 表
- **THEN** 记录 MUST 包含 plugin_id、enabled、config 字段

#### Scenario: Retrieve plugin state
- **WHEN** 查询特定插件状态
- **THEN** 返回该插件的启用状态和配置

#### Scenario: Plugin state schema
- **WHEN** 数据库初始化
- **THEN** plugin_state 表 MUST 包含 plugin_id (TEXT PRIMARY KEY)、enabled (INTEGER)、config (TEXT)、last_active (INTEGER) 字段

### Requirement: Database connection management
系统 SHALL 管理数据库连接的生命周期。

#### Scenario: Open connection on startup
- **WHEN** 应用启动
- **THEN** 系统 MUST 打开数据库连接
- **THEN** 连接 MUST 在整个应用生命周期内保持

#### Scenario: Close connection on shutdown
- **WHEN** 应用关闭
- **THEN** 系统 MUST 正确关闭数据库连接
- **THEN** 所有未提交的事务 MUST 被提交或回滚

### Requirement: Storage API via Tauri commands
系统 SHALL 通过 Tauri 命令暴露存储 API 给前端。

#### Scenario: storage_get command
- **WHEN** 前端调用 invoke("storage_get", { key: "theme" })
- **THEN** 返回对应的配置值

#### Scenario: storage_set command
- **WHEN** 前端调用 invoke("storage_set", { key: "theme", value: "dark" })
- **THEN** 配置值被存储到数据库

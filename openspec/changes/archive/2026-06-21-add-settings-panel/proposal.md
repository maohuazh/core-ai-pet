## Why

应用目前缺少全局配置入口。用户无法连接外部工具（Jira、邮箱、聊天工具）、无法管理宠物模型（切换、导入、删除）、无法配置动作/表情映射。需要实现一个半透明毛玻璃风格的设置面板窗口，作为统一的配置管理中心，所有配置持久化到 SQLite 数据库。

## What Changes

- 新增 Tauri 独立窗口（680x720px，无边框，半透明毛玻璃背景）
- 新增左侧导航 + 右侧内容区的设置面板布局
- 新增 Jira 连接管理模块（授权、多连接、启用/禁用，Mock 数据）
- 新增邮箱连接管理模块（同 Jira 结构，Mock 数据）
- 新增聊天工具连接模块（WeChat/Slack/Teams/Discord，Mock 数据）
- 新增模型配置模块（列出所有 Live2D + SpriteSheet 模型，切换/导入/删除）
- 新增 SQLite 数据库表：jira_connections、email_accounts、chat_platforms、models、model_action_mappings、app_settings
- 修改 PetHoverMenu 添加设置按钮入口
- 从 PetHoverMenu 触发 `open_settings_window` 命令

## Capabilities

### New Capabilities

- `settings-window`: 设置面板窗口管理（创建、显示、隐藏、自定义标题栏拖拽）
- `settings-jira-connection`: Jira 连接模块 UI + 数据管理（Mock 数据、启用/禁用、删除）
- `settings-email-connection`: 邮箱连接模块 UI + 数据管理（Mock 数据、启用/禁用、删除）
- `settings-chat-connection`: 聊天工具连接模块 UI + 数据管理（Mock 数据、启用/禁用、断开）
- `settings-model-config`: 模型配置模块 UI + 数据管理（列出模型、切换、导入跳转、删除）
- `settings-persistence`: 设置数据 SQLite 持久化（6 张新表、Mock 数据初始化）

### Modified Capabilities

- `hover-menu-layout`: 在悬浮菜单中添加设置按钮入口
- `storage-persistence`: 扩展数据库 schema，新增 6 张配置表

## Impact

- **前端**: 新增 `src/components/settings/` 目录（SettingsPanel、SettingsSidebar、SettingsTitleBar、4 个模块组件、共享组件）
- **后端**: 新增 `src-tauri/src/commands/settings.rs`（设置窗口管理 + IPC 命令）
- **数据库**: 新增 6 张表 + 索引 + Mock 数据初始化逻辑
- **窗口配置**: `tauri.conf.json` 新增 settings 窗口定义
- **样式**: 新增毛玻璃效果 CSS tokens、色彩系统、动效规范
- **依赖**: 无新增依赖，使用现有 Tauri 多窗口 API + SQLite

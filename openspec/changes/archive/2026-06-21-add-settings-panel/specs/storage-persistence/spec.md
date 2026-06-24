## MODIFIED Requirements

### Requirement: SQLite database initialization
系统 SHALL 在首次启动时自动创建 SQLite 数据库文件，并初始化所有表结构（包括新增的设置面板相关表）。

#### Scenario: Create database on first run
- **WHEN** 应用首次启动且数据库文件不存在
- **THEN** 系统 MUST 在 %APPDATA%/CoreAIpet/data.db 创建数据库文件
- **THEN** 系统 MUST 自动创建所有必要的表结构
- **THEN** 系统 MUST 创建 jira_connections、email_accounts、chat_platforms、models、model_action_mappings、app_settings 表

#### Scenario: Database file location
- **WHEN** 应用启动
- **THEN** 数据库文件 MUST 位于 %APPDATA%/CoreAIpet/data.db（Windows）
- **THEN** 或 ~/.config/CoreAIpet/data.db（Linux/macOS）

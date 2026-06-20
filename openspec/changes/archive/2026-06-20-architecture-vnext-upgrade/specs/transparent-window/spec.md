## ADDED Requirements

### Requirement: Window position persistence
系统 SHALL 自动保存窗口位置并在重启时恢复。

#### Scenario: Save position on drag end
- **WHEN** 用户完成窗口拖拽操作
- **THEN** 系统 MUST 将当前窗口位置 (x, y) 保存到 storage
- **THEN** 位置数据写入 window_position 表

#### Scenario: Save position on move
- **WHEN** 调用 set_window_position(x, y) API
- **THEN** 系统 MUST 将新位置保存到 storage

#### Scenario: Restore position on startup
- **WHEN** 应用启动
- **THEN** 系统 MUST 从 storage 读取上次保存的窗口位置
- **THEN** 窗口 MUST 移动到该位置
- **THEN** 若无保存的位置，使用默认居中位置

#### Scenario: Position persistence via SQLite
- **WHEN** 窗口位置变化
- **THEN** 系统 MUST 调用 storage_set("window_position", { x, y })
- **THEN** 数据持久化到 SQLite 数据库

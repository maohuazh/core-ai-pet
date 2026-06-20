## ADDED Requirements

### Requirement: Plugin configuration loading
系统 SHALL 在启动时扫描 plugins/ 目录并加载所有 plugin.json 配置。

#### Scenario: Scan plugins directory
- **WHEN** 应用启动
- **THEN** 系统 MUST 扫描 plugins/ 目录下的所有子目录
- **THEN** 每个子目录 MUST 包含 plugin.json 文件

#### Scenario: Parse plugin manifest
- **WHEN** 读取 plugin.json 文件
- **THEN** 系统 MUST 解析 id、name、version、author、enabled 字段
- **THEN** 解析失败时 MUST 记录错误日志并跳过该插件

### Requirement: Plugin list API
系统 SHALL 提供 API 获取所有已加载的插件列表。

#### Scenario: List all plugins
- **WHEN** 调用 plugin_list() API
- **THEN** 返回所有已加载插件的信息列表
- **THEN** 每个插件信息包含 id、name、version、enabled 字段

#### Scenario: Frontend access plugin list
- **WHEN** 前端调用 invoke("plugin_list")
- **THEN** 返回所有插件的 JSON 数组

### Requirement: Enable plugin
系统 SHALL 提供 API 启用特定插件。

#### Scenario: Enable plugin via API
- **WHEN** 调用 plugin_enable(plugin_id) API
- **THEN** 该插件 MUST 标记为启用状态
- **THEN** 状态 MUST 持久化到 plugin_state 表

#### Scenario: Enable from frontend
- **WHEN** 前端调用 invoke("plugin_enable", { plugin_id: "jira" })
- **THEN** Jira 插件被启用
- **THEN** 返回成功状态

### Requirement: Disable plugin
系统 SHALL 提供 API 禁用特定插件。

#### Scenario: Disable plugin via API
- **WHEN** 调用 plugin_disable(plugin_id) API
- **THEN** 该插件 MUST 标记为禁用状态
- **THEN** 状态 MUST 持久化到 plugin_state 表

#### Scenario: Disable from frontend
- **WHEN** 前端调用 invoke("plugin_disable", { plugin_id: "jira" })
- **THEN** Jira 插件被禁用
- **THEN** 返回成功状态

### Requirement: Plugin manifest schema
插件配置文件 SHALL 遵循标准 schema。

#### Scenario: Required fields
- **WHEN** 解析 plugin.json
- **THEN** 文件 MUST 包含 id (string)、name (string)、version (string)、enabled (boolean) 字段

#### Scenario: Optional fields
- **WHEN** 解析 plugin.json
- **THEN** 文件 MAY 包含 description (string)、author (string)、config (object) 字段

#### Scenario: Example plugin.json
- **WHEN** 查看 plugin.json 示例
- **THEN** 格式 MUST 如下：
```json
{
  "id": "coreai.jira",
  "name": "Jira",
  "version": "1.0.0",
  "author": "CoreAIpet",
  "enabled": true,
  "description": "Jira 任务管理集成",
  "config": {
    "baseUrl": "",
    "username": "",
    "apiToken": ""
  }
}
```

### Requirement: Plugin state persistence
插件启用状态 SHALL 持久化存储。

#### Scenario: Persist enabled state
- **WHEN** 插件被启用
- **THEN** enabled=true MUST 写入 plugin_state 表

#### Scenario: Restore state on restart
- **WHEN** 应用重启
- **THEN** 系统 MUST 从 plugin_state 表恢复插件启用状态
- **THEN** 上次禁用的插件 MUST 保持禁用

### Requirement: Plugin error handling
系统 SHALL 优雅处理插件加载错误。

#### Scenario: Invalid plugin.json
- **WHEN** plugin.json 格式错误
- **THEN** 系统 MUST 记录错误日志
- **THEN** 该插件 MUST 被跳过，不影响其他插件加载

#### Scenario: Missing plugin directory
- **WHEN** plugins/ 目录不存在
- **THEN** 系统 MUST 创建空目录
- **THEN** 应用 MUST 正常启动，无插件加载

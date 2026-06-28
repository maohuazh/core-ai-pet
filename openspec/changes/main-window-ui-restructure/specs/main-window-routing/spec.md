## ADDED Requirements

### Requirement: Main window pathname routing
App.vue SHALL 支持 `/app` 路由及子路由的内容切换。

#### Scenario: Navigate to /app
- **WHEN** 窗口加载路径为 `/app`
- **THEN** `App.vue` MUST 渲染主应用窗口（`MainWindow.vue`）
- **THEN** 默认显示主页内容

#### Scenario: Sub-route switching
- **WHEN** pathname 为 `/app/home`、`/app/tasks`、`/app/chat`、`/app/jira`、`/app/email`、`/app/message`
- **THEN** 主窗口 MUST 切换右侧内容区为对应页面组件

#### Scenario: Route extraction
- **WHEN** pathname 为 `/app/tasks`
- **THEN** 路由提取逻辑 MUST 返回 `"tasks"` 作为当前子路由

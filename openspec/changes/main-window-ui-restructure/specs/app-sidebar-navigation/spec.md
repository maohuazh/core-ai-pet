## ADDED Requirements

### Requirement: Shared layout shell
系统 SHALL 提供 `AppLayout.vue` 作为主窗口和设置窗口的共享布局外壳。

#### Scenario: Layout renders correctly
- **WHEN** `AppLayout` 被挂载
- **THEN** MUST 渲染顶部栏 (`AppTopBar`)
- **THEN** MUST 渲染左侧栏 (`AppSidebar`) 和右侧内容区 (slot)
- **THEN** 布局 MUST 为 flex 行布局，左侧栏宽度可折叠

#### Scenario: Sidebar collapse
- **WHEN** 用户点击侧边栏折叠按钮
- **THEN** 侧边栏 MUST 收缩为仅图标模式（宽度约 56px）
- **THEN** 内容区 MUST 自动扩展填充剩余空间

---

## ADDED Requirements

### Requirement: AppSidebar navigation items
侧边栏 SHALL 展示固定导航项列表，包含图标和文字标签。

#### Scenario: Default navigation items
- **WHEN** 侧边栏渲染
- **THEN** MUST 显示以下导航项（按顺序）：主页、任务、聊天、任务、聊天、Jira、邮件、消息
- **THEN** 每个导航项 MUST 包含图标和文字标签
- **THEN** 默认选中第一项（主页）

#### Scenario: Active state
- **WHEN** 用户点击某个导航项
- **THEN** 该导航项 MUST 显示为激活状态（高亮背景）
- **THEN** 右侧内容区 MUST 切换为对应页面组件

#### Scenario: Theme toggle
- **WHEN** 侧边栏渲染
- **THEN** 底部 MUST 固定显示深色/浅色模式切换开关
- **THEN** 切换开关 MUST 位于侧边栏最底部

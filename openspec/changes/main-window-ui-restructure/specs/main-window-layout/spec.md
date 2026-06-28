## ADDED Requirements

### Requirement: Main window creation
系统 SHALL 提供 Tauri 命令创建主应用窗口。窗口 MUST 为独立窗口，加载 `/app` 路由。

#### Scenario: Create main window
- **WHEN** 前端调用 invoke("open_main_window")
- **THEN** 系统 MUST 创建名为 "main-app" 的 Tauri 窗口
- **THEN** 窗口尺寸 MUST 为 1200x800 像素
- **THEN** 窗口最小尺寸 MUST 为 900x600 像素
- **THEN** 窗口 MUST 有原生装饰（decorations: true）
- **THEN** 窗口 MUST 支持调整大小

#### Scenario: Window already exists
- **WHEN** 主窗口已存在且被调用打开
- **THEN** 系统 MUST 聚焦已有窗口而非创建新窗口

### Requirement: Main window close behavior
主窗口关闭时 SHALL 隐藏而非销毁。

#### Scenario: Close main window
- **WHEN** 用户点击主窗口的关闭按钮
- **THEN** 窗口 MUST 隐藏（hide）而非销毁
- **THEN** 窗口状态和数据 MUST 保留

---

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
- **THEN** MUST 显示以下导航项（按顺序）：主页、任务、聊天、Jira、邮件、消息
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

---

## ADDED Requirements

### Requirement: AppTopBar components
顶部栏 SHALL 包含 Logo（左侧）和操作按钮（右侧）。

#### Scenario: TopBar left section
- **WHEN** 顶部栏渲染
- **THEN** 左侧 MUST 显示应用 Logo（爪印图标 + "CoreAIpet" 文字）

#### Scenario: TopBar right section - not logged in
- **WHEN** 用户未登录
- **THEN** 右侧 MUST 显示放大镜按钮、铃铛按钮、灰色默认头像
- **THEN** 不显示用户名
- **THEN** 点击灰色头像 MUST 弹出登录占位窗口

#### Scenario: TopBar right section - logged in
- **WHEN** 用户已登录
- **THEN** 右侧 MUST 显示放大镜按钮、铃铛按钮、用户头像和名称
- **THEN** 点击头像 MUST 弹出下拉菜单
- **THEN** 下拉菜单 MUST 包含：个人信息、设置、登出

---

## ADDED Requirements

### Requirement: Placeholder pages
系统 SHALL 为每个导航项提供占位页面组件。

#### Scenario: HomePage
- **WHEN** 用户导航到主页
- **THEN** 显示欢迎文案 "欢迎使用 CoreAIpet，你的 AI 桌面助手 🐾"

#### Scenario: TasksPage
- **WHEN** 用户导航到任务
- **THEN** 显示 "功能正在完善，敬请期待..."

#### Scenario: SchedulePage
- **WHEN** 用户导航到日程
- **THEN** 显示 "功能正在完善，敬请期待..."

#### Scenario: JiraPage
- **WHEN** 用户导航到 Jira
- **THEN** 显示 "功能正在完善，敬请期待..."

#### Scenario: EmailPage
- **WHEN** 用户导航到邮件
- **THEN** 显示 "功能正在完善，敬请期待..."

#### Scenario: MessagePage
- **WHEN** 用户导航到消息
- **THEN** 显示 "功能正在完善，敬请期待..."

#### Scenario: ProfilePage
- **WHEN** 用户导航到个人信息
- **THEN** 显示 "功能正在完善，敬请期待..."

---

## ADDED Requirements

### Requirement: Settings sidebar navigation
设置窗口侧边栏 SHALL 展示配置项列表。

#### Scenario: Settings navigation items
- **WHEN** 设置窗口侧边栏渲染
- **THEN** MUST 显示以下配置项（按顺序）：Jira、Email、IM、宠物、AI模型、快捷键
- **THEN** 每个配置项 MUST 包含图标和文字标签

#### Scenario: Settings content switching
- **WHEN** 用户点击某个配置项
- **THEN** 右侧内容区 MUST 显示对应配置模块

#### Scenario: Jira module migrated
- **WHEN** 用户点击 Jira 配置项
- **THEN** 右侧 MUST 显示现有 `JiraModule.vue` 的完整功能（连接管理、添加、编辑、删除、连接状态）

#### Scenario: Email module migrated
- **WHEN** 用户点击 Email 配置项
- **THEN** 右侧 MUST 显示现有 `EmailModule.vue` 的完整功能

#### Scenario: IM module migrated
- **WHEN** 用户点击 IM 配置项
- **THEN** 右侧 MUST 显示现有 `ChatModule.vue` 的功能（重命名为 IM）

#### Scenario: Pet model module migrated
- **WHEN** 用户点击宠物配置项
- **THEN** 右侧 MUST 显示现有 `ModelConfigModule.vue` 的功能

#### Scenario: AI Model module migrated
- **WHEN** 用户点击 AI模型 配置项
- **THEN** 右侧 MUST 显示现有 `LLMSettings.vue` 的功能

#### Scenario: Shortcuts module
- **WHEN** 用户点击快捷键配置项
- **THEN** 右侧 MUST 显示快捷键列表：截图(win+F2)、打开聊天窗口(Ctrl+Alt+N)、打开主窗口(Ctrl+Alt+L)
- **THEN** 每个快捷键 MUST 显示功能名称和按键组合

---

## ADDED Requirements

### Requirement: Chat window new layout
聊天窗口 SHALL 采用新的左右布局，左侧为会话列表，右侧为消息区域。

#### Scenario: Chat window sidebar
- **WHEN** 聊天窗口渲染
- **THEN** 左侧 MUST 显示会话列表区域
- **THEN** 会话列表 MUST 支持搜索
- **THEN** 会话列表 MUST 支持新建对话
- **THEN** 会话列表 MUST 显示会话标题、最后消息摘要、时间戳

#### Scenario: Chat message area preserved
- **WHEN** 用户选择某个会话
- **THEN** 右侧 MUST 显示该会话的消息列表
- **THEN** 消息列表 MUST 保持现有所有功能：流式响应、思考过程折叠、工具调用显示、workspace 选择、git 分支显示
- **THEN** 底部输入框 MUST 保持 Enter 发送、Shift+Enter 换行

#### Scenario: Chat window existing sessions
- **WHEN** 聊天窗口打开
- **THEN** MUST 从 SQLite 加载已有会话列表
- **THEN** MUST 保持会话的创建、删除功能

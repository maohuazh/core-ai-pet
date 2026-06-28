## MODIFIED Requirements

### Requirement: Settings window creation
设置窗口 SHALL 使用共享布局系统。窗口 MUST 加载 `/settings` 路由，使用 `AppLayout` 外壳。

#### Scenario: Create settings window
- **WHEN** 前端调用 invoke("open_settings_window")
- **THEN** 系统 MUST 创建名为 "settings" 的 Tauri 窗口
- **THEN** 窗口尺寸 MUST 为 800x720 像素
- **THEN** 窗口最小尺寸 MUST 为 640x480 像素
- **THEN** 窗口 MUST 无边框（decorations: false）
- **THEN** 窗口 MUST 支持透明背景

### Requirement: Settings window sidebar navigation
设置窗口侧边栏 SHALL 使用新的导航项列表。

#### Scenario: Settings sidebar items
- **WHEN** 设置窗口渲染
- **THEN** 侧边栏 MUST 显示以下配置项：Jira、Email、IM、宠物、AI模型、快捷键
- **THEN** 默认选中第一项（Jira）

### Requirement: Settings window content modules
设置窗口右侧内容区 SHALL 根据侧边栏选择显示对应配置模块。

#### Scenario: Jira configuration module
- **WHEN** 用户选择 Jira 配置项
- **THEN** 右侧 MUST 显示 Jira 连接管理（迁移自现有 `JiraModule.vue`）

#### Scenario: Email configuration module
- **WHEN** 用户选择 Email 配置项
- **THEN** 右侧 MUST 显示 Email 账户管理（迁移自现有 `EmailModule.vue`）

#### Scenario: IM configuration module
- **WHEN** 用户选择 IM 配置项
- **THEN** 右侧 MUST 显示 IM 连接管理（迁移自现有 `ChatModule.vue`）

#### Scenario: Pet model configuration module
- **WHEN** 用户选择宠物配置项
- **THEN** 右侧 MUST 显示宠物模型管理（迁移自现有 `ModelConfigModule.vue`）

#### Scenario: AI Model configuration module
- **WHEN** 用户选择 AI模型 配置项
- **THEN** 右侧 MUST 显示 LLM 模型配置（迁移自现有 `LLMSettings.vue`）

#### Scenario: Shortcuts module
- **WHEN** 用户选择快捷键配置项
- **THEN** 右侧 MUST 显示快捷键列表：截图(win+F2)、打开聊天窗口(Ctrl+Alt+N)、打开主窗口(Ctrl+Alt+L)

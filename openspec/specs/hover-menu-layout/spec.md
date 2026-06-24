## ADDED Requirements

### Requirement: Six functional buttons
悬浮菜单 SHALL 包含 exactly 6 个功能按钮：任务、消息、Jira、邮件、Agent、设置。每个按钮 MUST 有独立的 emoji 图标和中文标签。

#### Scenario: Menu contains all 6 buttons
- **WHEN** 鼠标悬停在模型上
- **THEN** 显示 6 个按钮：📋任务、💬消息、🔗Jira、📧邮件、🤖Agent、⚙️设置

### Requirement: Circular layout
6 个按钮 SHALL 以环形布局均匀分布在模型外围，半径约 80px，间隔 60°。按钮 MUST 不遮挡模型主体区域。在智能穿透模式下，按钮仅在鼠标进入角色区域（取消穿透）后可见和可交互。

#### Scenario: Buttons arranged in circle
- **WHEN** 悬浮菜单显示
- **THEN** 6 个按钮均匀分布在以模型中心为圆心、半径约 80px 的圆环上

#### Scenario: Menu hidden in click-through mode
- **WHEN** 鼠标离开角色区域，系统进入穿透模式
- **THEN** 悬浮菜单 SHALL 隐藏
- **THEN** 按钮不可交互

### Requirement: Button click triggers action
每个按钮点击后 SHALL 触发对应的 action 事件，由 App.vue 处理。设置按钮 MUST 触发打开设置面板窗口。

#### Scenario: Click task button
- **WHEN** 用户点击"📋 任务"按钮
- **THEN** 触发 action "task"，显示"任务功能即将推出"提示

#### Scenario: Click settings button
- **WHEN** 用户点击"⚙️ 设置"按钮
- **THEN** 触发 action "settings"
- **THEN** 系统 MUST 调用 invoke("open_settings_window")
- **THEN** 设置面板窗口 MUST 打开或聚焦

### Requirement: Remove close button from menu
悬浮菜单 SHALL NOT 包含关闭按钮。关闭功能不在悬浮菜单中提供。

#### Scenario: No close button in menu
- **WHEN** 悬浮菜单显示
- **THEN** 菜单中不包含 ❌ 关闭按钮

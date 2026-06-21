## MODIFIED Requirements

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

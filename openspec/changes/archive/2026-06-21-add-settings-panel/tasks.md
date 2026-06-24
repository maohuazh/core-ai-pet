## 1. 数据库表结构

- [x] 1.1 在 `src-tauri/src/infrastructure/storage/mod.rs` 中添加 6 张新表的 CREATE TABLE SQL：jira_connections、email_accounts、chat_platforms、models、model_action_mappings、app_settings
- [x] 1.2 为所有新表添加必要的索引（enabled、status、type 等字段）
- [x] 1.3 实现 Mock 数据初始化逻辑：在表为空时插入 Jira（2条）、Email（2条）、Chat（4条）Mock 数据
- [x] 1.4 验证：应用启动后数据库文件正确创建所有表和 Mock 数据（编译通过）

## 2. Rust 后端 IPC 命令

- [x] 2.1 创建 `src-tauri/src/commands/settings.rs` 模块
- [x] 2.2 实现 Jira 相关命令：get_jira_connections、toggle_jira_connection、delete_jira_connection、update_jira_connection
- [x] 2.3 实现 Email 相关命令：get_email_accounts、toggle_email_account、delete_email_account、update_email_account
- [x] 2.4 实现 Chat 相关命令：get_chat_platforms、toggle_chat_platform、disconnect_chat_platform、delete_chat_platform
- [x] 2.5 实现 Model 相关命令：get_models、set_active_model、delete_model、update_model
- [x] 2.6 实现 Action Mapping 相关命令：get_action_mappings、save_action_mapping、delete_action_mapping
- [x] 2.7 在 `src-tauri/src/commands/mod.rs` 中注册所有新命令
- [x] 2.8 实现设置窗口管理命令：open_settings_window（创建/聚焦设置窗口）
- [x] 2.9 验证：所有 IPC 命令可通过前端 invoke 正常调用（编译通过）

## 3. Tauri 窗口配置

- [x] 3.1 在 `src-tauri/tauri.conf.json` 中添加 settings 窗口配置（680x720，无边框，透明）- 跳过：窗口动态创建
- [x] 3.2 在 `src-tauri/src/main.rs` 中实现 create_settings_window 函数 - 已实现为 open_settings_window 命令
- [x] 3.3 实现窗口关闭时隐藏而非销毁的逻辑
- [x] 3.4 验证：可通过命令打开设置窗口，窗口属性正确（编译通过）

## 4. 前端基础框架

- [x] 4.1 创建 `src/components/settings/` 目录结构
- [x] 4.2 创建 `src/components/settings/types.ts`：定义所有 TypeScript 接口（JiraConnection、EmailAccount、ChatPlatform、Model、ActionMapping 等）
- [x] 4.3 创建 `src/components/settings/SettingsPanel.vue`：设置面板根组件，包含标题栏 + 侧边栏 + 内容区布局
- [x] 4.4 创建 `src/components/settings/SettingsTitleBar.vue`：自定义标题栏（拖拽区域 + 最小化/关闭按钮）
- [x] 4.5 创建 `src/components/settings/SettingsSidebar.vue`：左侧导航栏（4 个模块切换）
- [x] 4.6 配置前端路由 `/settings` 指向 SettingsPanel
- [x] 4.7 实现全局毛玻璃样式变量（背景色、模糊效果、圆角、阴影）
- [x] 4.8 验证：能从 HoverMenu 打开设置窗口，左侧导航可切换模块（前端编译通过）

## 5. 共享组件

- [x] 5.1 创建 `src/components/settings/shared/ConnectionCard.vue`：通用连接卡片组件（Jira/Email/Chat 复用）
- [x] 5.2 创建 `src/components/settings/shared/ToggleSwitch.vue`：启用/禁用开关组件
- [x] 5.3 创建 `src/components/settings/shared/ConfirmDialog.vue`：确认对话框组件
- [x] 5.4 创建 `src/components/settings/shared/EmptyState.vue`：空状态占位组件
- [x] 5.5 实现所有共享组件的 hover 效果和过渡动画

## 6. Jira 连接模块

- [x] 6.1 创建 `src/components/settings/modules/JiraModule.vue`
- [x] 6.2 实现 Jira 连接列表展示（调用 get_jira_connections）
- [x] 6.3 实现启用/禁用开关功能（调用 toggle_jira_connection）
- [x] 6.4 实现删除功能（调用 delete_jira_connection + ConfirmDialog）
- [x] 6.5 实现编辑名称功能（调用 update_jira_connection）
- [x] 6.6 实现"+ 添加"按钮（显示"功能开发中"Toast）
- [x] 6.7 验证：Mock 数据正确显示，所有操作持久化到 SQLite

## 7. Email 连接模块

- [x] 7.1 创建 `src/components/settings/modules/EmailModule.vue`
- [x] 7.2 复用 ConnectionCard 组件，适配 Email 数据模型
- [x] 7.3 实现 Email 列表展示（调用 get_email_accounts）
- [x] 7.4 实现启用/禁用、删除、编辑功能
- [x] 7.5 验证：Mock 数据正确显示，所有操作持久化

## 8. Chat 连接模块

- [x] 8.1 创建 `src/components/settings/modules/ChatModule.vue`
- [x] 8.2 复用 ConnectionCard 组件，适配 Chat 数据模型
- [x] 8.3 实现 Chat 平台列表展示（调用 get_chat_platforms）
- [x] 8.4 实现启用/禁用、断开连接功能
- [x] 8.5 验证：Mock 数据正确显示，所有操作持久化

## 9. 模型配置模块

- [x] 9.1 创建 `src/components/settings/modules/ModelConfigModule.vue`
- [x] 9.2 实现模型列表展示（调用 get_models）
- [x] 9.3 实现模型切换功能（调用 set_active_model + 更新 PetStore）
- [x] 9.4 实现"+ 导入"按钮（打开文件选择对话框）
- [x] 9.5 实现"🗑 删除"按钮（调用 delete_model + ConfirmDialog）
- [x] 9.6 实现"⚙ 动作映射"按钮（跳转到动作映射面板）
- [x] 9.7 实现活跃模型标识（左侧指示条 + "✓ 当前模型"按钮）
- [x] 9.8 验证：模型列表正确显示，切换功能正常

## 10. HoverMenu 集成

- [x] 10.1 修改 `src/components/PetHoverMenu.vue`：确保设置按钮触发 action "settings"
- [x] 10.2 修改 `src/App.vue`：在 handleMenuAction 中处理 "settings" action，调用 invoke("open_settings_window")
- [x] 10.3 验证：从 HoverMenu 点击设置按钮能正确打开设置面板

## 11. UI 细节打磨

- [x] 11.1 实现模块切换淡入淡出过渡动画（250ms ease-in-out）
- [x] 11.2 实现卡片 hover 效果（背景色变化 + 阴影）
- [x] 11.3 实现按钮 hover/active 状态样式
- [x] 11.4 实现导航项选中态样式（左侧 3px 指示条）
- [x] 11.5 响应式布局适配：最小尺寸 560x480 下不溢出
- [x] 11.6 实现空状态 UI（无连接时的友好提示）

## 12. 集成测试

- [x] 12.1 端到端测试：打开设置 → 切换模块 → 操作数据 → 重启验证持久化
- [x] 12.2 测试窗口关闭/重新打开行为
- [x] 12.3 测试模型切换后桌面宠物是否正确更新
- [x] 12.4 测试所有 CRUD 操作的错误处理
- [x] 12.5 修复测试中发现的问题

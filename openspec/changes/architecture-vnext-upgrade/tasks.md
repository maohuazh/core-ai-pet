## 1. Rust 依赖与项目结构

- [x] 1.1 在 src-tauri/Cargo.toml 中添加 rusqlite 依赖（bundled feature）
- [x] 1.2 在 src-tauri/Cargo.toml 中添加 tokio 依赖（full features）
- [x] 1.3 创建 src-tauri/src/core/ 目录结构（state/、eventbus/、plugin/）
- [x] 1.4 创建 src-tauri/src/infrastructure/ 目录结构（storage/、config/）
- [x] 1.5 更新 src-tauri/src/main.rs 注册新模块

## 2. 状态机核心实现（Rust）

- [x] 2.1 创建 src-tauri/src/core/state/mod.rs，定义 PetState 枚举
- [x] 2.2 实现 StateMachine 结构体，包含 current_state 和 transitions
- [x] 2.3 实现 StateMachine::transition() 方法，处理状态转换逻辑
- [x] 2.4 实现 StateMachine::get_state() 方法
- [x] 2.5 实现 StateMachine::set_state() 强制状态设置
- [x] 2.6 定义 StateChanged 事件结构
- [x] 2.7 实现最小状态持续时间机制
- [x] 2.8 编写状态机单元测试

## 3. EventBus 核心实现（Rust）

- [x] 3.1 创建 src-tauri/src/core/eventbus/mod.rs，定义 Event 结构
- [x] 3.2 实现 EventBus 结构体，包含 subscribers HashMap
- [x] 3.3 实现 EventBus::subscribe() 方法
- [x] 3.4 实现 EventBus::publish() 方法
- [x] 3.5 实现 EventBus::unsubscribe() 方法
- [x] 3.6 实现事件去重逻辑（100ms 内相同事件）
- [x] 3.7 实现事件日志记录
- [x] 3.8 定义标准事件类型枚举

## 4. SQLite 存储层实现（Rust）

- [x] 4.1 创建 src-tauri/src/infrastructure/storage/mod.rs
- [x] 4.2 实现 Database 结构体和连接管理
- [x] 4.3 实现数据库初始化逻辑（自动创建表）
- [x] 4.4 创建 config 表（key、value、updated_at）
- [x] 4.5 创建 chat_history 表（id、role、content、timestamp、metadata）
- [x] 4.6 创建 window_position 表（id、x、y、updated_at）
- [x] 4.7 创建 plugin_state 表（plugin_id、enabled、config、last_active）
- [x] 4.8 实现 storage_get() 和 storage_set() 函数
- [x] 4.9 实现 chat_store() 和 chat_list() 函数
- [x] 4.10 实现窗口位置保存和恢复函数
- [x] 4.11 实现数据库连接生命周期管理（启动打开、关闭释放）

## 5. 插件运行时实现（Rust）

- [x] 5.1 创建 src-tauri/src/core/plugin/mod.rs
- [x] 5.2 定义 Plugin 结构体和 PluginConfig
- [x] 5.3 实现 PluginManager 结构体
- [x] 5.4 实现 plugins/ 目录扫描逻辑
- [x] 5.5 实现 plugin.json 解析逻辑
- [x] 5.6 实现 PluginManager::load_plugins() 方法
- [x] 5.7 实现 PluginManager::enable_plugin() 和 disable_plugin()
- [x] 5.8 实现 PluginManager::list_plugins() 方法
- [x] 5.9 实现插件状态持久化到 SQLite
- [x] 5.10 实现插件错误处理（跳过无效插件）

## 6. Tauri 命令暴露（Rust）

- [x] 6.1 创建 src-tauri/src/commands/state.rs，实现 get_state 和 set_state 命令
- [x] 6.2 创建 src-tauri/src/commands/event.rs，实现 emit_event 和 subscribe_event 命令
- [x] 6.3 创建 src-tauri/src/commands/storage.rs，实现 storage_get 和 storage_set 命令
- [x] 6.4 创建 src-tauri/src/commands/plugin.rs，实现 plugin_list、plugin_enable、plugin_disable 命令
- [x] 6.5 更新 src-tauri/src/commands/mod.rs 导出新命令模块
- [x] 6.6 更新 src-tauri/src/main.rs 注册新命令到 invoke_handler
- [x] 6.7 更新 src-tauri/capabilities/default.json 添加新命令权限

## 7. 前端目录结构重组

- [x] 7.1 创建 src/core/state/ 目录
- [x] 7.2 创建 src/core/events/ 目录
- [x] 7.3 创建 src/core/storage/ 目录
- [x] 7.4 创建 src/core/plugin/ 目录
- [x] 7.5 创建 src/core/avatar/ 目录

## 8. 前端状态机桥接（TypeScript）

- [x] 8.1 创建 src/core/state/types.ts，定义 PetState 类型
- [x] 8.2 创建 src/core/state/index.ts，实现 getState 和 setState 函数
- [x] 8.3 实现 StateChanged 事件订阅
- [x] 8.4 导出状态机 API 供组件使用

## 9. 前端 EventBus 桥接（TypeScript）

- [x] 9.1 创建 src/core/events/types.ts，定义 Event 接口
- [x] 9.2 创建 src/core/events/index.ts，实现 subscribeEvent 和 publishEvent
- [x] 9.3 使用 @tauri-apps/api/event 桥接后端事件
- [x] 9.4 实现事件取消订阅机制

## 10. 前端 Storage 封装（TypeScript）

- [x] 10.1 创建 src/core/storage/index.ts
- [x] 10.2 实现 storageGet 和 storageSet 函数
- [x] 10.3 实现 chatStore 和 chatList 函数（预留）
- [x] 10.4 封装 Tauri invoke 调用

## 11. 前端插件管理（TypeScript）

- [x] 11.1 创建 src/core/plugin/types.ts，定义 PluginInfo 接口
- [x] 11.2 创建 src/core/plugin/index.ts
- [x] 11.3 实现 pluginList、pluginEnable、pluginDisable 函数
- [x] 11.4 封装 Tauri invoke 调用

## 12. Avatar 抽象层（TypeScript）

- [x] 12.1 创建 src/core/avatar/types.ts，定义 Avatar 接口
- [x] 12.2 创建 src/core/avatar/Live2DAvatar.ts
- [x] 12.3 实现 Live2DAvatar 类的 speak、think、work 方法
- [x] 12.4 实现 Live2DAvatar 的 playMotion 和 playExpression 代理
- [x] 12.5 实现 Live2DAvatar 的状态跟踪
- [x] 12.6 创建 src/core/avatar/factory.ts，实现 createAvatar 工厂函数
- [x] 12.7 创建 src/core/avatar/index.ts 导出 Avatar API

## 13. 集成与测试

- [x] 13.1 在 Live2DCanvas.vue 中集成 Avatar 抽象层
- [x] 13.2 实现状态机驱动 Live2D 动画切换
- [x] 13.3 实现窗口位置持久化（启动恢复、拖拽保存）
- [x] 13.4 编写前端单元测试（状态机、EventBus）
- [x] 13.5 编写集成测试（端到端流程）
- [x] 13.6 验证现有功能无回归（Live2D 渲染、模型切换、拖拽、菜单）

## 14. 文档更新

- [x] 14.1 更新 docs/Architecture.md 反映新架构
- [x] 14.2 更新 README.md 添加新模块说明
- [x] 14.3 创建 API 文档（Tauri 命令列表）

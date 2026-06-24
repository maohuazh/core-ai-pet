## Context

当前应用是一个 Tauri 2 桌面宠物应用，前端 Vue 3 + PixiJS，后端 Rust + SQLite。主窗口为 200x200 透明无边框窗口，带有 6 按钮环形悬浮菜单。所有数据通过 `src/core/storage/index.ts` 和 Rust 端 `src-tauri/src/infrastructure/storage/mod.rs` 持久化到 SQLite。

现有数据库表：config、chat_history、window_position、plugin_state。

需要新增设置面板作为独立窗口，提供 Jira/Email/Chat/Model 四大配置模块。

## Goals / Non-Goals

**Goals:**
- 实现半透明毛玻璃风格的设置面板窗口
- 实现 4 个配置模块的 UI（Jira、Email、Chat、Model）
- 所有配置数据持久化到 SQLite（6 张新表）
- 从悬浮菜单设置按钮触发打开
- 支持窗口拖拽、最小化、关闭

**Non-Goals:**
- 不实现 Jira/Email/Chat 的实际连接逻辑（本阶段仅 Mock UI）
- 不实现模型的导入/删除功能（由其他 PRD 覆盖）
- 不实现 Action/Expression 映射配置 UI（由其他 PRD 覆盖）
- 不实现多语言国际化

## Decisions

### 决策 1：使用 Tauri 多窗口而非前端路由

**选择**: 创建独立 Tauri 窗口加载 `/settings` 路由

**理由**:
- 设置面板需要独立于主窗口的生命周期（关闭后隐藏，不销毁）
- 独立窗口可以使用不同的窗口属性（尺寸、装饰、置顶策略）
- 避免主窗口路由切换时的闪烁问题

**替代方案**: 前端路由切换 → 被否决，因为主窗口尺寸固定 200x200，无法容纳设置面板

### 决策 2：前端组件结构采用模块化设计

**选择**: 按模块拆分子目录
```
src/components/settings/
  ├── SettingsPanel.vue
  ├── SettingsSidebar.vue
  ├── SettingsTitleBar.vue
  ├── modules/
  │   ├── JiraModule.vue
  │   ├── EmailModule.vue
  │   ├── ChatModule.vue
  │   └── ModelConfigModule.vue
  ├── shared/
  │   ├── ConnectionCard.vue
  │   ├── ToggleSwitch.vue
  │   └── ConfirmDialog.vue
  └── types.ts
```

**理由**:
- 模块隔离，便于后续独立迭代
- ConnectionCard 在 Jira/Email/Chat 间复用
- 共享组件集中管理

### 决策 3：数据库表设计采用独立表 + KV 混合

**选择**:
- Jira/Email/Chat 使用独立关系表（结构化查询）
- 应用全局设置使用 KV 表 app_settings（灵活扩展）
- 模型配置使用独立表 models（支持复杂查询）

**理由**:
- 连接数据需要列表查询和状态过滤，关系表更合适
- 全局设置种类不确定，KV 模式更灵活
- 与现有 config 表分离，避免混淆

### 决策 4：Mock 数据通过 SQL 初始化脚本注入

**选择**: 在数据库初始化时通过 SQL INSERT 插入 Mock 数据

**理由**:
- Mock 数据结构与真实表结构一致，后续迁移零成本
- 初始化逻辑集中在 Rust 端，前端无需感知
- 可通过条件判断避免重复插入

## Risks / Trade-offs

- **毛玻璃效果跨平台兼容性** → 使用 `backdrop-filter` CSS，Linux 下 fallback 为纯色背景
- **多窗口内存开销** → 设置窗口关闭后隐藏而非销毁，额外 ~30MB RAM
- **SQLite 并发写入** → 使用 Mutex 写锁 + 前端操作防抖
- **Mock 数据与真实数据迁移** → Mock 数据结构与真实表结构保持完全一致

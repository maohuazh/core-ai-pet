## Why

当前架构为"渲染驱动"（Avatar First），仅支持 Live2D 模型展示和基础交互，缺乏 Agent Runtime、Memory 系统、EventBus、状态机、插件运行时等核心基础设施。产品规划从 V1（桌宠）向 V2-V4（AI 助手 → 工作助手 → 多 Agent 协作）演进，需要从底层重构为"Agent 驱动"架构。

本次升级目标：在保持现有功能稳定的前提下，引入核心基础设施层，为后续 Agent 能力铺平道路。

## 前端框架决策：Vue vs React

**结论：保持 Vue 3，不迁移到 React。**

决策依据：
| 维度 | Vue 3 | React | 结论 |
|------|-------|-------|------|
| 性能 | 编译器优化，响应式系统高效 | 虚拟 DOM，手动优化 | Vue 略优 |
| 包体积 | ~33KB (runtime) | ~42KB (runtime) | Vue 更小 |
| 学习曲线 | 模板语法，渐进式 | JSX，Hooks 心智模型 | Vue 更低 |
| Live2D 集成 | pixi-live2d-display 框架无关 | 同左 | 无差异 |
| 迁移成本 | 当前全部功能正常 | 需重写所有组件 | 高风险 |
| 生态 | Pinia, Vue Router | Redux, React Router | 均成熟 |
| Tauri 支持 | 官方模板 | 官方模板 | 无差异 |

迁移到 React 无技术收益，且引入不必要的迁移风险。Vue 3 的组合式 API 已足够支撑复杂状态管理。

## What Changes

### 新增核心模块（Rust 后端）
- **state-machine**：角色状态机（Idle/Walking/Thinking/Talking/Working/Meeting/Sleeping/Alert）
- **event-bus**：发布-订阅事件总线，统一事件来源（Slack/Jira/邮件/会议等）
- **storage**：SQLite 持久化层（配置、聊天记录、插件状态、窗口位置）
- **plugin-runtime**：插件生命周期管理（加载/卸载/权限控制）

### 新增核心模块（前端）
- **state/**：状态机前端绑定，驱动 Live2D 动画切换
- **events/**：EventBus 前端桥接层
- **storage/**：SQLite 前端 API 封装
- **plugin/**：插件管理器前端部分
- **avatar/**：Avatar 抽象层（Live2D/Pixel/3D 统一接口）

### 目录结构重组
```
src/core/
├── avatar/        # Avatar 抽象层（新增）
├── state/         # 状态机前端绑定（新增）
├── events/        # EventBus 桥接（新增）
├── storage/       # SQLite 封装（新增）
├── plugin/        # 插件管理前端（新增）
├── model/         # 模型注册（现有，保留）
├── renderer/      # 渲染引擎（现有，保留）
├── behavior/      # 行为系统（预留→实现）
└── interaction/   # 交互系统（预留→实现）

src/services/
├── storage/       # 存储服务协议（新增）
└── tauri/         # Tauri 桥接（现有，扩展）

src-tauri/src/
├── core/
│   ├── agent/       # Agent Runtime（预留）
│   ├── memory/      # Memory 系统（预留）
│   ├── scheduler/   # 调度器（预留）
│   ├── plugin/      # 插件运行时（新增）
│   ├── eventbus/    # 事件总线（新增）
│   ├── state/       # 状态机（新增）
│   └── workflow/    # 工作流（预留）
└── infrastructure/
    ├── storage/     # SQLite 存储（新增）
    └── config/      # 配置管理（新增）
```

### 新增 Tauri 命令
- `get_state()` / `set_state()` — 状态机操作
- `emit_event()` / `subscribe_event()` — 事件总线操作
- `storage_get()` / `storage_set()` — 持久化操作
- `plugin_list()` / `plugin_enable()` / `plugin_disable()` — 插件管理

### 新增配置存储
- SQLite 数据库：`%APPDATA%/CoreAIpet/data.db`
- 表结构：`config`, `chat_history`, `plugin_state`, `window_position`

### 现有功能保留
- ✅ Live2D 渲染（PixiJS + pixi-live2d-display）
- ✅ 多模型切换（ModelRegistry + PetStore）
- ✅ 窗口拖拽（invoke start_dragging）
- ✅ 悬浮菜单（PetHoverMenu）
- ✅ 透明无边框窗口

## Capabilities

### New Capabilities
- `state-machine`: 角色状态机，管理 Idle/Walking/Thinking/Talking 等状态转换，驱动 Live2D 动画
- `event-bus`: 发布-订阅事件总线，统一事件来源（来自后端事件或前端触发）
- `storage-persistence`: SQLite 持久化层，支持配置、聊天记录、窗口位置等数据存储
- `plugin-runtime`: 插件生命周期管理，支持加载/卸载/启用/禁用插件
- `avatar-abstraction`: Avatar 抽象层，统一 Live2D/Pixel/3D 接口（speak/think/work/playMotion）

### Modified Capabilities
- `transparent-window`: 新增窗口位置持久化能力（从 state-machine 或 storage 获取位置）
- `model-rendering`: 新增状态机驱动动画切换能力（从 state-machine 接收状态变化）

## Impact

### 代码影响
- **新增**：~15 个新模块文件（Rust + TypeScript）
- **修改**：Live2DCanvas.vue、Live2DRenderer.ts、main.rs（扩展命令注册）
- **保留**：ModelRegistry、PetStore、PetHoverMenu（无需修改）

### 依赖变更
**Rust 新增依赖：**
```toml
rusqlite = { version = "0.31", features = ["bundled"] }  # SQLite
serde = { version = "1", features = ["derive"] }          # 已有，扩展使用
tokio = { version = "1", features = ["full"] }             # 异步运行时
```

**前端新增依赖：**
```json
"@tauri-apps/plugin-sql": "^2.0.0"  # SQLite Tauri 插件
```

### API 变更
- 新增 ~10 个 Tauri 命令（state/event/storage/plugin 相关）
- 现有命令（start_dragging, set/get_window_position）保持不变

### 存储变更
- 新增 SQLite 数据库文件（首次运行自动创建）
- 配置文件仍可通过 JSON 兼容旧版本

### 迁移路径
- V1 功能 100% 保留，无破坏性变更
- 新模块渐进式集成，旧代码无需大规模重构
- 后续 V2-V4 功能可基于新基础设施逐步叠加

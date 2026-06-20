## Context

当前 CoreAIpet 为 V1 阶段产品，基于 Tauri 2 + Vue 3 + PixiJS + Live2D 实现桌面宠物功能。现有架构为"渲染驱动"（Avatar First），核心组件包括：
- ModelRegistry：模型注册中心
- PetStore：响应式状态管理
- Live2DRenderer：渲染引擎
- PetHoverMenu：悬浮菜单

产品规划从 V1（桌宠）向 V2-V4（AI 助手 → 工作助手 → 多 Agent 协作）演进，需要引入 Agent Runtime、Memory 系统、EventBus、状态机、插件运行时等基础设施。

**约束条件：**
- 现有功能必须 100% 保留，无破坏性变更
- 前端框架保持 Vue 3（决策依据见 proposal）
- 渐进式升级，支持分阶段交付

## Goals / Non-Goals

**Goals:**
1. 引入状态机（State Machine）管理角色状态，驱动 Live2D 动画切换
2. 引入事件总线（EventBus）统一事件来源，解耦组件通信
3. 引入 SQLite 持久化层，支持配置、聊天记录、窗口位置等存储
4. 引入插件运行时（Plugin Runtime），支持插件加载/卸载/启用/禁用
5. 引入 Avatar 抽象层，统一 Live2D/Pixel/3D 接口，为未来扩展预留
6. 重组目录结构，为后续 Agent/Memory/Workflow 模块预留空间

**Non-Goals:**
1. 不实现 Agent Runtime（本次仅预留目录和接口）
2. 不实现 Memory 系统（本次仅预留目录和接口）
3. 不实现具体插件（Jira/Slack/Gmail 等后续实现）
4. 不迁移前端框架（Vue → React）
5. 不引入向量数据库（LanceDB 等后续引入）

## Decisions

### Decision 1: 状态机设计 — 使用 Rust 后端状态机

**选择：** 状态机核心逻辑在 Rust 后端实现，前端通过 Tauri 命令调用。

**理由：**
- 状态机可能被多个前端组件和后端插件共享
- Rust 实现性能更高，适合高频状态转换
- 后端状态机可被插件系统直接访问

**替代方案：**
- Vue 前端实现：简单场景可行，但跨模块共享复杂
- 独立状态机库（如 xstate）：引入额外依赖，学习成本高

**实现方式：**
```rust
// src-tauri/src/core/state/mod.rs
pub enum PetState {
    Idle,
    Walking,
    Thinking,
    Talking,
    Working,
    Meeting,
    Sleeping,
    Alert,
}

pub struct StateMachine {
    current_state: PetState,
    transitions: HashMap<(PetState, Event), PetState>,
}

impl StateMachine {
    pub fn transition(&mut self, event: Event) -> Result<PetState, StateError>;
    pub fn get_state(&self) -> PetState;
}
```

### Decision 2: EventBus 设计 — 发布-订阅模式

**选择：** Rust 后端实现 EventBus，前端通过 Tauri 事件系统桥接。

**理由：**
- Tauri 自带事件系统（emit/listen），可复用
- 后端 EventBus 统一处理来自插件、系统、前端的事件
- 前端通过 `@tauri-apps/api/event` 订阅后端事件

**实现方式：**
```rust
// src-tauri/src/core/eventbus/mod.rs
pub struct EventBus {
    subscribers: HashMap<String, Vec<Sender<Event>>>,
}

impl EventBus {
    pub fn subscribe(&mut self, event_type: &str) -> Receiver<Event>;
    pub fn publish(&self, event: Event);
}
```

前端桥接：
```typescript
// src/core/events/index.ts
import { listen, emit } from '@tauri-apps/api/event';

export function subscribeEvent(eventType: string, callback: (event: Event) => void) {
  return listen(eventType, (e) => callback(e.payload));
}

export function publishEvent(eventType: string, payload: any) {
  emit(eventType, payload);
}
```

### Decision 3: 持久化存储 — SQLite + rusqlite

**选择：** 使用 SQLite 作为持久化存储，通过 rusqlite（bundled 模式）编译。

**理由：**
- SQLite 成熟稳定，适合嵌入式场景
- 支持复杂查询（聊天记录检索等）
- 单文件数据库，便于备份和迁移
- rusqlite bundled 模式无需系统安装 SQLite

**替代方案：**
- localStorage/IndexedDB：简单场景可行，但查询能力弱
- JSON 文件：适合配置，不适合大量结构化数据
- LevelDB/RocksDB：过于重量级

**表结构设计：**
```sql
-- 配置存储
CREATE TABLE config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at INTEGER NOT NULL
);

-- 聊天记录
CREATE TABLE chat_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    metadata TEXT
);

-- 插件状态
CREATE TABLE plugin_state (
    plugin_id TEXT PRIMARY KEY,
    enabled INTEGER NOT NULL,
    config TEXT,
    last_active INTEGER
);

-- 窗口位置
CREATE TABLE window_position (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    x INTEGER NOT NULL,
    y INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
```

### Decision 4: 插件运行时 — 配置驱动 + 权限控制

**选择：** 插件以 JSON 配置为主，运行时由 Rust 后端管理生命周期。

**理由：**
- 当前阶段插件以配置为主（API Key、启用状态等）
- 后续 V3 阶段可扩展为 WASM 插件或独立进程
- 权限控制确保插件不会越权访问系统资源

**实现方式：**
```rust
// src-tauri/src/core/plugin/mod.rs
pub struct PluginManager {
    plugins: HashMap<String, Plugin>,
}

pub struct Plugin {
    id: String,
    name: String,
    enabled: bool,
    config: PluginConfig,
}

impl PluginManager {
    pub fn load_plugins(&mut self) -> Result<(), PluginError>;
    pub fn enable_plugin(&mut self, plugin_id: &str) -> Result<(), PluginError>;
    pub fn disable_plugin(&mut self, plugin_id: &str) -> Result<(), PluginError>;
    pub fn list_plugins(&self) -> Vec<PluginInfo>;
}
```

### Decision 5: Avatar 抽象层 — TypeScript 接口定义

**选择：** 在 TypeScript 层定义 Avatar 接口，Live2DRenderer 实现该接口。

**理由：**
- 为未来 Pixel/3D Avatar 预留扩展点
- 统一上层调用接口，屏蔽底层渲染差异
- 状态机通过 Avatar 接口控制角色表现

**实现方式：**
```typescript
// src/core/avatar/types.ts
export interface Avatar {
  speak(text: string): Promise<void>;
  think(): Promise<void>;
  work(): Promise<void>;
  playMotion(group: string, index?: number): Promise<void>;
  playExpression(nameOrIndex: string | number): Promise<void>;
  getState(): PetState;
}

// src/core/avatar/Live2DAvatar.ts
export class Live2DAvatar implements Avatar {
  constructor(private renderer: Live2DRenderer) {}
  
  async speak(text: string): Promise<void> {
    await this.renderer.playMotion("Talking", 0);
  }
  
  async think(): Promise<void> {
    await this.renderer.playMotion("Thinking", 0);
  }
  // ...
}
```

### Decision 6: 目录结构重组 — 渐进式迁移

**选择：** 新增核心模块放入 `src/core/` 和 `src-tauri/src/core/`，现有模块保持不变。

**理由：**
- 现有代码（model/、renderer/）已稳定，无需移动
- 新模块按职责划分，清晰可扩展
- 避免大规模重构引入风险

**目录映射：**
```
新增前端模块:
  src/core/state/       → 状态机前端绑定
  src/core/events/      → EventBus 桥接
  src/core/storage/     → SQLite 封装
  src/core/plugin/      → 插件管理前端
  src/core/avatar/      → Avatar 抽象层

新增后端模块:
  src-tauri/src/core/state/       → 状态机实现
  src-tauri/src/core/eventbus/    → EventBus 实现
  src-tauri/src/core/plugin/      → 插件运行时
  src-tauri/src/infrastructure/storage/ → SQLite 存储
```

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| SQLite bundled 编译时间增加 | 使用 `rusqlite` bundled feature，首次编译约 30s，后续增量编译快 |
| 状态机复杂度增长 | 保持状态转换规则简单，避免过度设计；后续可引入状态机库 |
| EventBus 事件风暴 | 限制事件类型，添加事件去重和限流机制 |
| 插件权限控制不足 | 初期仅支持配置型插件，不支持代码执行；后续引入 WASM 沙箱 |
| Avatar 抽象层过度设计 | 仅定义接口，不实现 Pixel/3D；等有实际需求再扩展 |
| 目录重组导致合并冲突 | 仅新增目录，不移动现有文件；使用特性分支开发 |
| Tauri 事件系统性能瓶颈 | 高频事件使用 Rust 内部通道，不经过 Tauri 事件系统 |

## Migration Plan

### Phase 1: 基础设施层（本次变更）
1. 新增 Rust 依赖（rusqlite）
2. 实现状态机核心模块
3. 实现 EventBus 核心模块
4. 实现 SQLite 存储层
5. 实现插件管理器（配置型）
6. 定义 Avatar 接口
7. 前端桥接层实现
8. 目录结构重组

### Phase 2: 集成与测试（后续变更）
1. 状态机驱动 Live2D 动画切换
2. 窗口位置持久化
3. 单元测试覆盖

### Phase 3: V2 功能叠加（后续变更）
1. AI 对话集成（Chat 模块）
2. 聊天记录存储
3. 系统托盘

## Open Questions

1. **状态转换规则来源**：状态转换规则是硬编码还是从配置文件读取？（建议：初期硬编码，后续可扩展）
2. **EventBus 持久化**：事件是否需要持久化以支持离线回放？（建议：初期不持久化，仅内存）
3. **插件配置存储位置**：插件配置存储在 SQLite 还是独立 JSON 文件？（建议：SQLite 统一管理）
4. **Avatar 接口版本**：Avatar 接口是否需要版本控制以支持向后兼容？（建议：初期不需要，后续按需引入）

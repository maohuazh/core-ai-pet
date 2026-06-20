# CoreAIpet 架构设计文档

> Version: 2.0 | Date: 2026-06-20
> 基于 Tauri 2 + Vue 3 + PixiJS + Live2D 的桌面宠物架构

---

## 1. 技术栈总览

### 前端
| 分类 | 技术 | 版本 | 说明 |
|------|------|------|------|
| 框架 | Vue 3 | ^3.5.13 | 组合式 API + `<script setup>` |
| 构建工具 | Vite | ^6.0.3 | 开发服务器端口 1420 |
| 类型检查 | TypeScript + vue-tsc | ^5.7.2 / ^2.2.0 | 严格模式 |
| 2D 渲染 | PixiJS | ^7.4.2 | WebGL 渲染引擎 |
| Live2D 显示 | pixi-live2d-display | ^0.4.0 | Live2D Cubism 2/4 模型加载 |

### 后端/桌面端
| 分类 | 技术 | 版本 | 说明 |
|------|------|------|------|
| 桌面框架 | Tauri 2 | ^2.2.0 | 原生桌面应用 |
| 语言 | Rust | Edition 2021 | 后端逻辑 |
| 插件 | tauri-plugin-shell | 2 | Shell 命令支持 |
| 序列化 | serde + serde_json | 1 | JSON 序列化 |

---

## 2. 整体架构

```
┌─────────────────────────────────────────────────────────────┐
│                     Tauri 2 桌面窗口                          │
│  配置: 透明 / 无边框 / 置顶 / 400×400 / skipTaskbar          │
├──────────────────────────┬──────────────────────────────────┤
│      Rust 后端层          │          Vue 3 前端层              │
│                          │                                  │
│  ┌────────────────────┐  │  ┌────────────────────────────┐   │
│  │  commands/         │  │  │  App.vue (根组件)           │   │
│  │   ├ start_dragging │◄─┼──┤   ├ Live2DCanvas.vue       │   │
│  │   ├ set_window_pos │  │  │   │    └ Live2DRenderer.ts │   │
│  │   └ get_window_pos │  │  │   │        ├ PixiJS App    │   │
│  └────────────────────┘  │  │   │        └ pixi-live2d-  │   │
│                          │  │   │           display       │   │
│  ┌────────────────────┐  │  │   └ PetHoverMenu.vue       │   │
│  │  services/         │  │  │        (6个环形功能按钮)     │   │
│  │   └ window/        │  │  └────────────────────────────┘   │
│  │     (窗口服务)      │  │                                  │
│  └────────────────────┘  │  ┌────────────────────────────┐   │
│                          │  │  core/                      │   │
│  invoke() 命令调用 ──────┼──│   ├ model/ModelRegistry.ts  │   │
│                          │  │   ├ model/PetStore.ts       │   │
│                          │  │   └ renderer/live2d/        │   │
│                          │  │     └ Live2DRenderer.ts     │   │
│                          │  └────────────────────────────┘   │
├──────────────────────────┴──────────────────────────────────┤
│                 Live2D 模型资源 (public/models/)              │
│    Hiyori / Mao / Natori (本地) + Haru (CDN)                │
└─────────────────────────────────────────────────────────────┘
```

---

## 3. 前端架构

### 3.1 组件层

| 文件 | 职责 |
|------|------|
| `App.vue` | 根组件，管理悬浮菜单显隐、鼠标悬停交互、动作分发 |
| `Live2DCanvas.vue` | Live2D 画布组件，初始化渲染器，监听模型切换，处理拖拽 |
| `PetHoverMenu.vue` | 悬浮菜单，6个圆形按钮环形排列（任务/消息/Jira/邮件/Agent/设置） |

### 3.2 核心层（`src/core/`）

#### 模型管理 (`core/model/`)

| 模块 | 职责 |
|------|------|
| `ModelRegistry.ts` | 模型注册中心，管理所有可用 Live2D 模型配置 |
| `PetStore.ts` | 响应式状态管理（基于 Vue `ref`），跟踪当前模型，支持切换 |

#### 渲染引擎 (`core/renderer/live2d/`)

| 模块 | 职责 |
|------|------|
| `Live2DRenderer.ts` | 封装 PixiJS Application + pixi-live2d-display，负责模型加载/动画/表情 |

### 3.3 数据流

#### 模型切换流程
```
用户触发切换
    → PetStore.switchToNextModel()
    → Live2DCanvas watch 监听 currentModel 变化
    → Live2DRenderer.loadModel(newUrl)
    → 移除旧模型 → 加载新模型 → 自动播放 idle 动画
```

#### 窗口拖拽流程
```
用户在 canvas 上按下鼠标左键
    → Live2DCanvas.onMouseDown()
    → invoke("start_dragging")
    → Rust commands::window::start_dragging()
    → Tauri Window::start_dragging()
    → 窗口跟随鼠标移动
```

#### 悬浮菜单交互
```
鼠标进入 pet-container
    → App.vue onMouseEnter()
    → showMenu = true → PetHoverMenu 渲染
    → 6个按钮环形排列（半径100px，等分360°）

鼠标离开 pet-container
    → 200ms 延迟后 showMenu = false
    → PetHoverMenu 销毁

点击菜单按钮
    → PetHoverMenu.handleClick(action)
    → props.onAction(action)
    → App.vue handleMenuAction(action)
    → 执行对应功能（当前为 alert 占位）
```

### 3.4 路径别名

```typescript
// vite.config.ts
"@":         → "src/"
"@core":     → "src/core/"
"@components": "src/components/"
"@modules":  → "src/modules/"
"@services": → "src/services/"
```

---

## 4. 后端架构（Rust + Tauri 2）

### 4.1 模块结构

```
src-tauri/src/
├── main.rs                  # 应用入口，注册插件和命令
├── commands/
│   ├── mod.rs               # 命令模块导出
│   └── window.rs            # 窗口控制命令
│       ├ start_dragging()    # 开始拖拽窗口
│       ├ set_window_position() # 设置窗口位置
│       └ get_window_position() # 获取窗口位置
└── services/
    ├── mod.rs               # 服务模块导出
    └── window/
        └── mod.rs           # WindowService 结构体
```

### 4.2 Tauri 命令

前端通过 `invoke()` 调用 Rust 命令：

| 命令 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `start_dragging` | 无 | `Result<(), String>` | 开始窗口拖拽 |
| `set_window_position` | `x: f64, y: f64` | `Result<(), String>` | 设置窗口位置 |
| `get_window_position` | 无 | `Result<(f64, f64), String>` | 获取窗口位置 |

### 4.3 窗口配置

```json
// tauri.conf.json
{
  "windows": [{
    "title": "Core AI Pet",
    "width": 400,
    "height": 400,
    "resizable": false,
    "decorations": false,      // 无边框
    "transparent": true,       // 透明背景
    "alwaysOnTop": true,       // 桌面置顶
    "skipTaskbar": true,       // 不显示在任务栏
    "center": true
  }]
}
```

---

## 5. 目录结构

```
core-ai-pet/
├── index.html                         # HTML 入口
├── package.json                       # 前端依赖配置
├── vite.config.ts                     # Vite 构建配置 + 路径别名
├── tsconfig.json                      # TypeScript 配置
├── tsconfig.node.json                 # Node 端 TypeScript 配置
│
├── src/                               # ★ 前端源码
│   ├── main.ts                        # Vue 应用入口
│   ├── App.vue                        # 根组件
│   ├── vite-env.d.ts                  # Vite 类型声明
│   │
│   ├── components/                    # UI 组件
│   │   ├── Live2DCanvas.vue           # Live2D 画布组件
│   │   └── PetHoverMenu.vue           # 悬浮菜单组件
│   │
│   ├── core/                          # 核心逻辑
│   │   ├── model/                     # 模型管理
│   │   │   ├── ModelRegistry.ts       # 模型注册中心
│   │   │   └── PetStore.ts            # 响应式状态管理
│   │   │
│   │   ├── renderer/                  # 渲染引擎
│   │   │   └── live2d/
│   │   │       ├── Live2DRenderer.ts  # Live2D 渲染器
│   │   │       └── lib/               # Live2D Core 库
│   │   │
│   │   ├── behavior/                  # 行为系统（预留）
│   │   └── interaction/               # 交互系统（预留）
│   │
│   ├── modules/                       # 功能模块（预留）
│   │   ├── chat/                      # 聊天模块
│   │   └── settings/                  # 设置模块
│   │
│   ├── services/                      # 服务层（预留）
│   │   ├── storage/                   # 存储服务
│   │   └── tauri/                     # Tauri 桥接服务
│   │
│   ├── assets/                        # 静态资源
│   └── views/                         # 视图（预留）
│
├── src-tauri/                         # ★ Rust 后端
│   ├── Cargo.toml                     # Rust 依赖配置
│   ├── tauri.conf.json                # Tauri 窗口/构建配置
│   ├── build.rs                       # 构建脚本
│   ├── capabilities/
│   │   └── default.json               # 权限配置
│   ├── icons/                         # 应用图标
│   ├── gen/                           # 自动生成文件
│   │   └── schemas/                   # ACL/Capability schema
│   │
│   └── src/
│       ├── main.rs                    # 应用入口
│       ├── commands/                  # Tauri 命令
│       │   ├── mod.rs
│       │   └── window.rs              # 窗口控制命令
│       ├── services/                  # 服务层
│       │   ├── mod.rs
│       │   └── window/
│       │       └── mod.rs             # WindowService
│       ├── core/                      # 核心逻辑（预留）
│       └── infrastructure/            # 基础设施（预留）
│
├── public/models/                     # ★ Live2D 模型资源
│   ├── Hiyori/                        # Hiyori 模型（Cubism 4）
│   │   ├── Hiyori.model3.json
│   │   ├── Hiyori.cdi3.json
│   │   ├── Hiyori.physics3.json
│   │   ├── Hiyori.pose3.json
│   │   ├── Hiyori.userdata3.json
│   │   ├── Hiyori.2048/               # 纹理贴图
│   │   └── motions/                   # 动作文件 (10个)
│   │
│   ├── Mao/                           # Mao 模型（Cubism 4）
│   │   ├── Mao.model3.json
│   │   ├── expressions/               # 表情文件 (8个)
│   │   └── motions/                   # 动作文件 (8个)
│   │
│   ├── Natori/                        # Natori 模型（Cubism 4）
│   │   ├── Natori.model3.json
│   │   ├── exp/                       # 表情文件 (11个)
│   │   └── motions/                   # 动作文件 (8个)
│   │
│   └── shizuku/                       # Shizuku 模型（Cubism 2）
│       └── shizuku.model.json
│
├── plugins/                           # 插件配置（预留）
│   ├── CoreAIpet.Plugin.Email.Gmail/
│   │   └── plugin.json
│   ├── CoreAIpet.Plugin.Email.IMAP/
│   │   └── plugin.json
│   ├── CoreAIpet.Plugin.Email.Outlook/
│   │   └── plugin.json
│   ├── CoreAIpet.Plugin.Jira/
│   │   └── plugin.json
│   ├── CoreAIpet.Plugin.Message.DingTalk/
│   │   └── plugin.json
│   ├── CoreAIpet.Plugin.Message.Feishu/
│   │   └── plugin.json
│   ├── CoreAIpet.Plugin.Message.QQ/
│   │   └── plugin.json
│   ├── CoreAIpet.Plugin.Message.Slack/
│   │   └── plugin.json
│   ├── CoreAIpet.Plugin.Message.Teams/
│   │   └── plugin.json
│   └── CoreAIpet.Plugin.Message.WeChat/
│       └── plugin.json
│
├── docs/                              # 文档
│   ├── Architecture.md                # 本文档
│   ├── PRD - Windows AI桌面助手 V1.0.md
│   └── desktop-pet-design/            # 方案对比文档
│       ├── README.md
│       ├── 00-方案对比.md
│       ├── 01-方案一-Electron+Live2D.md
│       ├── 02-方案二-Tauri+精灵图.md
│       ├── 03-方案三-PySide6+Live2D.md
│       └── 04-方案四-Tauri+Live2D-推荐方案.md
│
├── openspec/                          # OpenSpec 变更管理
│   ├── specs/                         # 当前规格定义
│   │   ├── hover-menu-layout/
│   │   ├── live2d-renderer/
│   │   ├── model-registry/
│   │   ├── model-rendering/
│   │   ├── model-switching/
│   │   ├── pet-drag/
│   │   └── transparent-window/
│   └── changes/                       # 变更历史
│       ├── archive/                   # 已归档变更
│       └── remove-window-border/      # 进行中变更
│
├── dist/                              # 构建输出目录
│   ├── index.html
│   ├── assets/
│   └── models/
│
├── backup/dotnet-legacy/              # .NET 旧版本备份
│   ├── CoreAIpet.Core/
│   ├── CoreAIpet.Desktop/
│   ├── CoreAIpet.Live2DBridge/
│   └── Plugins/
│
├── vendor/live2d-models/              # Live2D 模型仓库（git submodule）
│   └── Live2d-model/
│
├── node_modules/                      # npm 依赖
└── tests/                             # 测试（遗留自 .NET 版本）
    ├── CoreAIpet.Core.Tests/
    └── CoreAIpet.Desktop.Tests/
```

---

## 6. 核心模块详解

### 6.1 ModelRegistry — 模型注册中心

```typescript
interface PetModelConfig {
  id: string;           // 唯一标识
  name: string;         // 显示名称
  description?: string; // 描述
  modelUrl: string;     // 模型文件 URL（本地或 CDN）
  cubismVersion: 2 | 4; // Live2D Cubism 版本
}
```

当前注册模型：

| ID | 名称 | 来源 | 说明 |
|----|------|------|------|
| `haru` | Haru | CDN (jsDelivr) | 动漫女孩，有 idle/happy 动画 |
| `hiyori` | Hiyori | 本地 (默认) | Live2D 官方 Cubism 4 示例，丰富 idle 动画 |
| `mao` | Mao | 本地 | 8个表情 + 6个点击动画 |
| `natori` | Natori | 本地 | 11个表情 + 5个点击动画 |

### 6.2 PetStore — 响应式状态管理

```typescript
class PetStore {
  currentModel: Ref<PetModelConfig>;  // 当前选中模型
  models: Ref<PetModelConfig[]>;      // 所有可用模型

  setCurrentModel(model: PetModelConfig): void;
  switchToNextModel(): void;          // 循环切换到下一个模型
}
```

### 6.3 Live2DRenderer — 渲染引擎

```typescript
interface IRenderer {
  init(): Promise<void>;                           // 初始化 PixiJS + Live2D
  loadModel(modelPath: string): Promise<void>;     // 加载模型
  playMotion(group: string, index?: number): Promise<void>;  // 播放动画
  playExpression(nameOrIndex: string | number): Promise<void>; // 播放表情
  getMotionGroups(): MotionGroup[];                // 获取动画组列表
  getExpressions(): ExpressionInfo[];              // 获取表情列表
  destroy(): void;                                 // 销毁
}
```

关键实现：
- 使用 `pixi-live2d-display` 加载 Live2D 模型
- 自动适配画布大小（缩放比例 0.9）
- 加载完成后自动播放 idle 动画
- 支持模型热切换（先销毁旧模型再加载新模型）

### 6.4 PetHoverMenu — 悬浮菜单

6个功能按钮环形排列，半径 100px：

| 按钮 | 图标 | 功能 |
|------|------|------|
| task | 📋 | 任务管理 |
| message | 💬 | 消息中心 |
| jira | 🔗 | Jira 集成 |
| email | 📧 | 邮件 |
| agent | 🤖 | AI Agent |
| settings | ⚙️ | 设置 |

交互：
- 鼠标悬停在宠物区域时显示
- 离开后 200ms 延迟隐藏
- 按钮有 popIn 动画和 hover 缩放效果
- 点击后通过 `onAction` 回调处理

---

## 7. 构建与开发

### 7.1 开发模式

```bash
npm run tauri dev
```

流程：
1. Vite 启动开发服务器 (http://localhost:1420)
2. Tauri 启动 Rust 后端
3. 加载前端页面到 WebView

### 7.2 生产构建

```bash
npm run build        # vue-tsc 类型检查 + vite build
npm run tauri build  # 打包桌面应用
```

### 7.3 构建配置

```typescript
// vite.config.ts
{
  server: {
    port: 1420,
    strictPort: true,
  },
  build: {
    target: "chrome105",  // Windows/Linux 使用 Chromium
    minify: "esbuild",
    sourcemap: true,      // debug 模式
    outDir: "dist",
  }
}
```

---

## 8. 关键设计特点

### 8.1 透明无边框窗口

Tauri 窗口配置实现桌面宠物效果：
- `transparent: true` — 透明背景
- `decorations: false` — 无边框
- `alwaysOnTop: true` — 桌面置顶
- `skipTaskbar: true` — 不显示在任务栏
- `resizable: false` — 固定大小 400×400

### 8.2 多模型切换

- `ModelRegistry` 注册多个 Live2D 模型
- `PetStore` 响应式管理当前选中模型
- `Live2DCanvas` 通过 `watch` 监听模型变化，自动重新加载

### 8.3 Live2D 渲染

- 使用 **PixiJS** 作为 WebGL 渲染引擎
- 使用 **pixi-live2d-display** 加载 Live2D 模型
- 支持 Cubism 2 和 Cubism 4 格式
- 自动播放 idle 动画

### 8.4 环形悬浮菜单

- 鼠标悬停显示 6 个功能按钮
- 半径 100px 环形排列
- 有 popIn 动画和 hover 缩放效果
- 200ms 延迟隐藏防止误操作

### 8.5 窗口拖拽

- 在 canvas 上按下鼠标左键
- 通过 `invoke("start_dragging")` 调用 Rust 命令
- Tauri 原生窗口拖拽实现

### 8.6 插件架构预留

- `plugins/` 目录已有消息/邮件/Jira 插件的 `plugin.json` 配置
- 为后续扩展预留了清晰的架构

---

## 9. 依赖关系图

```
┌─────────────────────────────────────────────────────────┐
│                      前端 (Vue 3)                        │
│                                                         │
│  App.vue ──► Live2DCanvas.vue ──► Live2DRenderer.ts     │
│     │              │                    │                │
│     │              ▼                    ▼                │
│     └────► PetHoverMenu.vue      PixiJS + pixi-live2d   │
│                                    -display              │
│                                   │                      │
│              PetStore.ts ◄────────┘                      │
│                  │                                       │
│                  ▼                                       │
│            ModelRegistry.ts                              │
└─────────────────────────────────────────────────────────┘
                           │
                           │ invoke()
                           ▼
┌─────────────────────────────────────────────────────────┐
│                    后端 (Tauri 2 + Rust)                  │
│                                                         │
│  main.rs ──► commands/window.rs                         │
│                 ├ start_dragging()                       │
│                 ├ set_window_position()                  │
│                 └ get_window_position()                  │
│                                                         │
│              services/window/WindowService               │
└─────────────────────────────────────────────────────────┘
```

---

## 10. 扩展指南

### 10.1 添加新 Live2D 模型

```
1. 将模型文件放入 public/models/{ModelName}/
2. 在 ModelRegistry.ts 中注册:

   modelRegistry.register({
     id: "new-model",
     name: "New Model",
     description: "描述",
     modelUrl: "./models/{ModelName}/{ModelName}.model3.json",
     cubismVersion: 4,
   });

3. 重启应用即可在模型列表中看到新模型
```

### 10.2 添加菜单功能

```
1. 在 PetHoverMenu.vue 的 menuItems 中添加:
   { action: "new-feature", icon: "🆕", label: "新功能" }

2. 在 App.vue 的 actionLabels 中添加:
   "new-feature": "新功能"

3. 在 handleMenuAction 中实现具体逻辑
```

### 10.3 添加 Tauri 命令

```rust
// src-tauri/src/commands/window.rs
#[tauri::command]
pub async fn new_command() -> Result<(), String> {
    // 实现逻辑
    Ok(())
}

// src-tauri/src/main.rs
// 注册命令
.invoke_handler(tauri::generate_handler![
    start_dragging,
    set_window_position,
    get_window_position,
    new_command,  // 添加新命令
])
```

---

## 11. 验证方案

| 验证项 | 方法 |
|--------|------|
| Live2D 渲染 | 启动应用，确认模型正常显示 |
| 模型切换 | 触发切换，确认新模型加载成功 |
| 窗口拖拽 | 按住宠物拖动，确认窗口跟随移动 |
| 悬浮菜单 | 鼠标悬停，确认菜单显示；离开，确认菜单隐藏 |
| 透明背景 | 确认窗口外区域透明 |
| 桌面置顶 | 打开其他窗口，确认宠物始终在最上层 |
| 动画播放 | 确认 idle 动画自动播放 |
| 表情切换 | 调用 playExpression，确认表情变化 |

---

## 12. 已知问题与待优化

### 待实现功能
- [ ] AI 对话集成（Chat 模块预留）
- [ ] 设置界面（Settings 模块预留）
- [ ] 插件系统加载逻辑
- [ ] 眼球追踪
- [ ] 角色状态机（Idle/Happy/Thinking/Talking）
- [ ] 位置持久化
- [ ] 系统托盘

### 技术优化
- [ ] 添加错误边界处理
- [ ] 优化模型加载性能
- [ ] 添加模型缓存机制
- [ ] 支持更多 Live2D 特效

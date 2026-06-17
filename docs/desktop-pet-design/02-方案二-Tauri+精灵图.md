# 方案二：Tauri + 精灵图 架构设计

## 技术选型

| 层次 | 技术 | 版本 |
|------|------|------|
| 桌面框架 | Tauri | 2.x |
| 后端语言 | Rust | 1.75+ |
| 前端框架 | Solid.js + TypeScript | 1.8+ |
| 渲染引擎 | Canvas 2D / WebGL | 原生 |
| 状态管理 | SolidJS 内置 | - |
| 构建工具 | Vite | 5.x |
| 样式 | UnoCSS | 0.58+ |
| 数据库 | SQLite (via rusqlite) | - |

---

## 为什么选 Solid.js 而非 React/Vue？

| 特性 | Solid.js | React | Vue |
|------|----------|-------|-----|
| 包体积 | ~7KB | ~40KB | ~30KB |
| 运行时 | 无虚拟DOM | 有 | 有 |
| 性能 | 接近原生 | 中 | 中 |
| 与 Tauri 配合 | 极佳 | 好 | 好 |

---

## 架构概览

```
┌─────────────────────────────────────────────────────────────┐
│                   Tauri Rust Backend                          │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────┐   │
│  │ Window   │ │ System   │ │ AI       │ │ Database     │   │
│  │ Manager  │ │ Tray     │ │ Service  │ │ (SQLite)     │   │
│  └──────────┘ └──────────┘ └──────────┘ └──────────────┘   │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────┐   │
│  │ Config   │ │ Auto     │ │ Global   │ │ Screen       │   │
│  │ Manager  │ │ Launch   │ │ Shortcut │ │ Capture      │   │
│  └──────────┘ └──────────┘ └──────────┘ └──────────────┘   │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐                    │
│  │ MCP      │ │ TTS      │ │ Updater  │                    │
│  │ Client   │ │ Service  │ │          │                    │
│  └──────────┘ └──────────┘ └──────────┘                    │
└─────────────────────────────────────────────────────────────┘
                          │ Tauri Commands / Events
┌─────────────────────────────────────────────────────────────┐
│                Solid.js Frontend                             │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                    View Layer                          │  │
│  │  ┌────────┐ ┌────────┐ ┌──────────┐ ┌─────────────┐  │  │
│  │  │ Pet    │ │ Chat   │ │ Settings │ │ Context Menu│  │  │
│  │  │ View   │ │ View   │ │ View     │ │             │  │  │
│  │  └────────┘ └────────┘ └──────────┘ └─────────────┘  │  │
│  └───────────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │              Rendering Engine                          │  │
│  │  ┌────────────────┐ ┌────────────────┐               │  │
│  │  │ Sprite Sheet   │ │ Canvas         │               │  │
│  │  │ Manager        │ │ Animation Loop │               │  │
│  │  └────────────────┘ └────────────────┘               │  │
│  └───────────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │             Behavior System                            │  │
│  │  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐  │  │
│  │  │ State Machine│ │ Event Bus    │ │ Scheduler    │  │  │
│  │  └──────────────┘ └──────────────┘ └──────────────┘  │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

---

## 文件结构设计

```
tauri-desktop-pet/
│
├── package.json                    # 前端依赖
├── Cargo.toml                      # Rust 工作区配置
├── vite.config.ts                  # Vite 配置
├── tsconfig.json                   # TypeScript 配置
├── uno.config.ts                   # UnoCSS 配置
├── .env.example                    # 环境变量
├── README.md                       # 项目说明
├── tauri.conf.json                 # Tauri 配置
│
├── src-tauri/                      # ===== Rust 后端 =====
│   ├── Cargo.toml                  # Rust 依赖
│   ├── tauri.conf.json             # Tauri 配置
│   ├── build.rs                    # 构建脚本
│   │
│   ├── src/
│   │   ├── main.rs                 # 入口
│   │   ├── lib.rs                  # 库入口
│   │   │
│   │   ├── commands/               # Tauri 命令（供前端调用）
│   │   │   ├── mod.rs
│   │   │   ├── pet.rs              # 宠物控制命令
│   │   │   ├── chat.rs             # 聊天命令
│   │   │   ├── settings.rs         # 设置命令
│   │   │   ├── window.rs           # 窗口命令
│   │   │   └── system.rs           # 系统命令
│   │   │
│   │   ├── services/               # 后端服务
│   │   │   ├── mod.rs
│   │   │   ├── window_manager.rs   # 窗口管理
│   │   │   ├── config_manager.rs   # 配置管理
│   │   │   ├── tray_manager.rs     # 系统托盘
│   │   │   ├── shortcut_manager.rs # 全局快捷键
│   │   │   ├── auto_launch.rs      # 开机自启
│   │   │   ├── screen_capture.rs   # 屏幕截图
│   │   │   ├── updater.rs          # 自动更新
│   │   │   └── store.rs            # 持久化存储
│   │   │
│   │   ├── ai/                     # AI 服务
│   │   │   ├── mod.rs
│   │   │   ├── provider.rs         # 提供商 trait
│   │   │   ├── openai.rs           # OpenAI 实现
│   │   │   ├── claude.rs           # Claude 实现
│   │   │   ├── gemini.rs           # Gemini 实现
│   │   │   ├── chat_service.rs     # 聊天服务
│   │   │   ├── streaming.rs        # 流式响应
│   │   │   └── tool_use.rs         # 工具调用
│   │   │
│   │   ├── mcp/                    # MCP 集成
│   │   │   ├── mod.rs
│   │   │   ├── client.rs           # MCP 客户端
│   │   │   ├── transport.rs        # 传输层（stdio）
│   │   │   ├── tool_registry.rs    # 工具注册
│   │   │   └── executor.rs         # 工具执行器
│   │   │
│   │   ├── tts/                    # 语音合成
│   │   │   ├── mod.rs
│   │   │   ├── service.rs          # TTS 服务
│   │   │   └── lip_sync.rs         # 口型同步
│   │   │
│   │   ├── db/                     # 数据库
│   │   │   ├── mod.rs
│   │   │   ├── migrations.rs       # 数据库迁移
│   │   │   ├── models/             # 数据模型
│   │   │   │   ├── chat_history.rs
│   │   │   │   ├── pet_state.rs
│   │   │   │   └── settings.rs
│   │   │   └── queries/            # 查询
│   │   │       ├── chat.rs
│   │   │       └── pet.rs
│   │   │
│   │   ├── events/                 # 事件系统
│   │   │   ├── mod.rs
│   │   │   └── types.rs            # 事件类型
│   │   │
│   │   └── error.rs                # 错误定义
│   │
│   ├── migrations/                 # SQLite 迁移文件
│   │   ├── 001_init.sql
│   │   └── 002_chat_history.sql
│   │
│   └── icons/                      # 应用图标
│       ├── 32x32.png
│       ├── 128x128.png
│       ├── icon.ico
│       └── icon.icns
│
├── src/                            # ===== Solid.js 前端 =====
│   ├── index.html                  # HTML 入口
│   ├── main.tsx                    # Solid 入口
│   ├── App.tsx                     # 根组件
│   │
│   ├── views/                      # 页面视图
│   │   ├── PetView.tsx             # 宠物视图
│   │   ├── ChatView.tsx            # 聊天视图
│   │   └── SettingsView.tsx        # 设置视图
│   │
│   ├── components/                 # 组件
│   │   ├── pet/                    # 宠物组件
│   │   │   ├── PetCanvas.tsx       # 画布组件
│   │   │   ├── PetBubble.tsx       # 对话气泡
│   │   │   ├── PetContextMenu.tsx  # 右键菜单
│   │   │   └── PetEffects.tsx      # 特效组件
│   │   ├── chat/                   # 聊天组件
│   │   │   ├── ChatMessage.tsx     # 消息组件
│   │   │   ├── ChatInput.tsx       # 输入组件
│   │   │   ├── ChatStream.tsx      # 流式输出
│   │   │   └── MessageBubble.tsx   # 消息气泡
│   │   ├── settings/               # 设置组件
│   │   │   ├── GeneralPanel.tsx    # 通用设置
│   │   │   ├── ApiPanel.tsx        # API 设置
│   │   │   ├── AppearancePanel.tsx # 外观设置
│   │   │   └── ShortcutPanel.tsx   # 快捷键设置
│   │   └── ui/                     # 基础 UI 组件
│   │       ├── Button.tsx
│   │       ├── Modal.tsx
│   │       ├── Input.tsx
│   │       └── Tooltip.tsx
│   │
│   ├── engine/                     # 渲染引擎（核心）
│   │   ├── SpriteRenderer.ts       # 精灵图渲染器
│   │   ├── SpriteSheet.ts          # 精灵图集管理
│   │   ├── AnimationController.ts  # 动画控制器
│   │   ├── FrameScheduler.ts       # 帧调度器
│   │   ├── HitDetector.ts          # 碰撞检测
│   │   └── types.ts                # 引擎类型定义
│   │
│   ├── behavior/                   # 行为系统
│   │   ├── StateMachine.ts         # 状态机
│   │   ├── states/                 # 状态定义
│   │   │   ├── BaseState.ts        # 基础状态
│   │   │   ├── IdleState.ts        # 空闲
│   │   │   ├── WalkState.ts        # 行走
│   │   │   ├── TalkState.ts        # 说话
│   │   │   ├── SleepState.ts       # 睡眠
│   │   │   └── ReactState.ts       # 反应
│   │   ├── transitions.ts          # 转换规则
│   │   ├── triggers/               # 触发器
│   │   │   ├── TimeTrigger.ts      # 时间触发
│   │   │   ├── InteractionTrigger.ts # 交互触发
│   │   │   └── SystemTrigger.ts    # 系统事件
│   │   └── BehaviorConfig.ts       # 行为配置
│   │
│   ├── stores/                     # 状态管理
│   │   ├── petStore.ts             # 宠物状态
│   │   ├── chatStore.ts            # 聊天状态
│   │   ├── settingsStore.ts        # 设置状态
│   │   └── uiStore.ts              # UI 状态
│   │
│   ├── services/                   # 前端服务
│   │   ├── tauriBridge.ts          # Tauri 通信桥
│   │   ├── eventBus.ts             # 事件总线
│   │   ├── animationService.ts    # 动画服务
│   │   └── storageService.ts      # 存储服务
│   │
│   ├── types/                      # TypeScript 类型
│   │   ├── pet.ts
│   │   ├── chat.ts
│   │   ├── settings.ts
│   │   └── tauri.ts
│   │
│   ├── assets/                     # 静态资源
│   │   ├── sprites/                # 精灵图资源
│   │   │   ├── idle/               # 空闲动画帧
│   │   │   │   └── sheet.webp
│   │   │   ├── walk/               # 行走动画帧
│   │   │   │   └── sheet.webp
│   │   │   ├── talk/               # 说话动画帧
│   │   │   │   └── sheet.webp
│   │   │   ├── sleep/              # 睡眠动画帧
│   │   │   │   └── sheet.webp
│   │   │   └── react/              # 反应动画帧
│   │   │       └── sheet.webp
│   │   ├── audio/                  # 音效
│   │   │   ├── click.wav
│   │   │   └── notification.wav
│   │   ├── images/                 # 图片
│   │   │   ├── icons/
│   │   │   └── backgrounds/
│   │   └── styles/                 # 全局样式
│   │       ├── global.css
│   │       └── animations.css
│   │
│   └── utils/                      # 工具函数
│       ├── math.ts                 # 数学计算
│       ├── platform.ts             # 平台检测
│       └── logger.ts               # 日志
│
├── public/                         # 公共资源
│   └── pet/                        # 宠物资源配置
│       └── manifest.json           # 宠物清单
│
├── scripts/                        # 构建脚本
│   ├── build-sprites.ts            # 精灵图打包
│   └── generate-manifest.ts        # 清单生成
│
├── tests/                          # 测试
│   ├── unit/
│   └── integration/
│
└── docs/
    ├── architecture.md
    └── sprite-format.md            # 精灵图格式说明
```

---

## 核心模块设计

### 1. 精灵图渲染器

```typescript
// src/engine/SpriteRenderer.ts

export interface SpriteFrame {
  x: number;      // 帧在图集中的 x 坐标
  y: number;      // 帧在图集中的 y 坐标
  width: number;
  height: number;
  duration: number; // 帧持续时间 (ms)
}

export interface SpriteAnimation {
  name: string;
  frames: SpriteFrame[];
  loop: boolean;
  priority: number;
}

export class SpriteRenderer {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private spriteSheet: SpriteSheet;
  private currentAnimation: SpriteAnimation | null = null;
  private frameIndex: number = 0;
  private lastFrameTime: number = 0;
  
  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    this.ctx = canvas.getContext('2d')!;
    this.spriteSheet = new SpriteSheet();
  }
  
  async loadSpriteSheet(imagePath: string, manifest: SpriteManifest): Promise<void> {
    await this.spriteSheet.load(imagePath, manifest);
  }
  
  playAnimation(name: string, priority: number = 0): void {
    const anim = this.spriteSheet.getAnimation(name);
    if (!anim) return;
    if (this.currentAnimation && this.currentAnimation.priority > priority) return;
    
    this.currentAnimation = anim;
    this.frameIndex = 0;
    this.lastFrameTime = performance.now();
  }
  
  update(timestamp: number): void {
    if (!this.currentAnimation) return;
    
    const frame = this.currentAnimation.frames[this.frameIndex];
    if (timestamp - this.lastFrameTime >= frame.duration) {
      this.frameIndex++;
      if (this.frameIndex >= this.currentAnimation.frames.length) {
        if (this.currentAnimation.loop) {
          this.frameIndex = 0;
        } else {
          this.currentAnimation = null;
          return;
        }
      }
      this.lastFrameTime = timestamp;
    }
    
    this.render();
  }
  
  private render(): void {
    if (!this.currentAnimation) return;
    
    const frame = this.currentAnimation.frames[this.frameIndex];
    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
    this.spriteSheet.drawFrame(this.ctx, frame, 0, 0);
  }
  
  hitTest(x: number, y: number): boolean {
    // 基于像素 alpha 的碰撞检测
    const pixel = this.ctx.getImageData(x, y, 1, 1).data;
    return pixel[3] > 0; // alpha > 0
  }
}
```

### 2. 行为状态机

```typescript
// src/behavior/StateMachine.ts

export interface State {
  name: string;
  onEnter(): void;
  onUpdate(dt: number): void;
  onExit(): void;
}

export interface Transition {
  from: string;
  to: string;
  condition: () => boolean;
  action?: () => void;
}

export class StateMachine {
  private states: Map<string, State> = new Map();
  private transitions: Transition[] = [];
  private currentState: State | null = null;
  
  addState(state: State): void {
    this.states.set(state.name, state);
  }
  
  addTransition(transition: Transition): void {
    this.transitions.push(transition);
  }
  
  setState(name: string): void {
    const newState = this.states.get(name);
    if (!newState || newState === this.currentState) return;
    
    if (this.currentState) {
      this.currentState.onExit();
    }
    
    this.currentState = newState;
    this.currentState.onEnter();
  }
  
  update(dt: number): void {
    if (!this.currentState) return;
    
    // 检查转换条件
    for (const transition of this.transitions) {
      if (transition.from === this.currentState.name && transition.condition()) {
        if (transition.action) transition.action();
        this.setState(transition.to);
        break;
      }
    }
    
    this.currentState.onUpdate(dt);
  }
  
  getCurrentState(): string | null {
    return this.currentState?.name ?? null;
  }
}
```

### 3. Tauri 命令层

```rust
// src-tauri/src/commands/chat.rs

use tauri::State;
use crate::ai::chat_service::ChatService;

#[tauri::command]
pub async fn send_message(
    message: String,
    chat_service: State<'_, ChatService>,
) -> Result<String, String> {
    chat_service.send_message(&message).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn send_message_stream(
    message: String,
    chat_service: State<'_, ChatService>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let mut stream = chat_service.send_message_stream(&message).await
        .map_err(|e| e.to_string())?;
    
    while let Some(chunk) = stream.next().await {
        app.emit("chat-chunk", &chunk).map_err(|e| e.to_string())?;
    }
    
    app.emit("chat-complete", ()).map_err(|e| e.to_string())?;
    Ok(())
}
```

---

## 精灵图格式规范

### 标准格式

```json
{
  "version": "1.0",
  "name": "default-pet",
  "cellSize": { "width": 192, "height": 208 },
  "animations": {
    "idle": {
      "row": 0,
      "frames": [
        { "col": 0, "duration": 280 },
        { "col": 1, "duration": 110 },
        { "col": 2, "duration": 110 },
        { "col": 3, "duration": 140 },
        { "col": 4, "duration": 140 },
        { "col": 5, "duration": 320 }
      ],
      "loop": true,
      "priority": 0
    },
    "walk_right": {
      "row": 1,
      "frames": [
        { "col": 0, "duration": 120 },
        { "col": 1, "duration": 120 },
        { "col": 2, "duration": 120 },
        { "col": 3, "duration": 120 },
        { "col": 4, "duration": 120 },
        { "col": 5, "duration": 120 },
        { "col": 6, "duration": 120 },
        { "col": 7, "duration": 220 }
      ],
      "loop": true,
      "priority": 1
    }
  }
}
```

### 动画状态定义

| 行号 | 状态名 | 帧数 | 循环 | 说明 |
|------|--------|------|------|------|
| 0 | idle | 6 | 是 | 空闲待机 |
| 1 | walk_right | 8 | 是 | 向右行走 |
| 2 | walk_left | 8 | 是 | 向左行走 |
| 3 | wave | 4 | 否 | 挥手 |
| 4 | jump | 5 | 否 | 跳跃 |
| 5 | sad | 8 | 否 | 难过 |
| 6 | wait | 6 | 是 | 等待 |
| 7 | run | 6 | 是 | 奔跑 |
| 8 | sleep | 6 | 是 | 睡眠 |

---

## 扩展性设计

### 1. 自定义宠物包

用户可以创建自定义宠物包：

```
my-pet/
├── manifest.json         # 清单文件
├── spritesheet.webp      # 精灵图集
├── config.json           # 宠物配置（性格、行为）
└── audio/                # 音效
    ├── click.wav
    └── greeting.wav
```

### 2. 行为脚本配置

```json
{
  "behavior": {
    "states": {
      "idle": {
        "minDuration": 5000,
        "maxDuration": 30000,
        "nextStates": ["walk_right", "walk_left", "sleep"]
      },
      "walk_right": {
        "speed": 20,
        "maxDistance": 300,
        "nextStates": ["idle"]
      }
    },
    "triggers": {
      "time_based": [
        { "time": "08:00", "state": "greeting", "message": "早上好！" },
        { "time": "23:00", "state": "sleep" }
      ],
      "idle_triggers": [
        { "idleMinutes": 30, "state": "sad", "message": "陪我玩一会儿吧" }
      ]
    }
  }
}
```

### 3. 插件系统

```typescript
// 宠物插件接口
export interface PetPlugin {
  name: string;
  version: string;
  
  // 生命周期
  onInit(): void;
  onDestroy(): void;
  
  // 事件钩子
  onStateChange?(from: string, to: string): void;
  onInteraction?(type: string, x: number, y: number): void;
  onChat?(message: string): string | void;
}

// 注册插件
registerPlugin({
  name: 'weather-plugin',
  version: '1.0.0',
  onInit() {
    // 初始化天气插件
  },
  onStateChange(from, to) {
    if (to === 'sleep') {
      // 天气变冷时提醒盖被子
    }
  }
});
```

---

## 性能数据预期

| 指标 | 目标值 |
|------|--------|
| 安装包大小 | ~8-15 MB |
| 内存占用 (idle) | ~30-50 MB |
| 内存占用 (active) | ~50-80 MB |
| CPU 占用 (idle) | < 1% |
| 启动时间 | < 1s |
| 渲染帧率 | 60 FPS |

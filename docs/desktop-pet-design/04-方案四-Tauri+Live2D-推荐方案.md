# 方案四：Tauri + Live2D 架构设计（推荐方案）

## 技术选型

| 层次 | 技术 | 版本 |
|------|------|------|
| 桌面框架 | Tauri | 2.x |
| 后端语言 | Rust | 1.75+ |
| 前端框架 | Vue 3 + TypeScript | 3.4+ |
| 渲染引擎 | PixiJS | 7.x |
| Live2D | pixi-live2d-display | 0.4+ |
| 状态管理 | Pinia | 2.x |
| 构建工具 | Vite | 5.x |
| AI SDK | Vercel AI SDK (Rust 移植) | - |
| 样式 | UnoCSS | 0.58+ |
| 数据库 | SQLite (via rusqlite) | - |
| 序列化 | serde + serde_json | - |

---

## 为什么这是推荐方案

| 特性 | 优势 |
|------|------|
| **包体积** | ~15MB（Electron 的 1/10） |
| **内存占用** | ~80MB（Electron 的 1/3） |
| **性能** | Rust 后端 + 原生渲染，接近 C++ |
| **安全性** | Rust 内存安全 + Tauri 权限模型 |
| **Live2D** | 通过 Web 端 pixi-live2d-display 支持 |
| **开发效率** | Vue 3 生态 + TypeScript |
| **跨平台** | Windows / macOS / Linux |

---

## 架构概览

```
┌──────────────────────────────────────────────────────────────┐
│                    Tauri Rust Backend                          │
│                                                               │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │              Core Services                               │ │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐  │ │
│  │  │ Window   │ │ Config   │ │ Database │ │ AI       │  │ │
│  │  │ Manager  │ │ Manager  │ │ (SQLite) │ │ Service  │  │ │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘  │ │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐  │ │
│  │  │ Tray     │ │ Shortcut │ │ Auto     │ │ MCP      │  │ │
│  │  │ Manager  │ │ Manager  │ │ Launch   │ │ Client   │  │ │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘  │ │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐               │ │
│  │  │ TTS      │ │ Screen   │ │ Updater  │               │ │
│  │  │ Service  │ │ Capture  │ │          │               │ │
│  │  └──────────┘ └──────────┘ └──────────┘               │ │
│  └─────────────────────────────────────────────────────────┘ │
│                                                               │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │              Event System (tokio-based)                  │ │
│  └─────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────┘
                          │ Tauri Commands / Events
┌──────────────────────────────────────────────────────────────┐
│                  Vue 3 Frontend                               │
│                                                               │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │              View Layer (Vue Components)                 │ │
│  │  ┌────────┐ ┌────────┐ ┌──────────┐ ┌──────────────┐  │ │
│  │  │ Pet    │ │ Chat   │ │ Settings │ │ Context Menu │  │ │
│  │  │ View   │ │ View   │ │ View     │ │              │  │ │
│  │  └────────┘ └────────┘ └──────────┘ └──────────────┘  │ │
│  └─────────────────────────────────────────────────────────┘ │
│                                                               │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │              Rendering Engine (PixiJS + Live2D)          │ │
│  │  ┌──────────────────┐  ┌──────────────────┐            │ │
│  │  │ Live2D Renderer  │  │ Sprite Renderer  │            │ │
│  │  │ (pixi-live2d)    │  │ (fallback)       │            │ │
│  │  └──────────────────┘  └──────────────────┘            │ │
│  │  ┌──────────────────┐  ┌──────────────────┐            │ │
│  │  │ Animation        │  │ Behavior         │            │ │
│  │  │ Controller       │  │ State Machine    │            │ │
│  │  └──────────────────┘  └──────────────────┘            │ │
│  └─────────────────────────────────────────────────────────┘ │
│                                                               │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │              State Management (Pinia)                    │ │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐  │ │
│  │  │ pet      │ │ chat     │ │ settings │ │ ui       │  │ │
│  │  │ store    │ │ store    │ │ store    │ │ store    │  │ │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘  │ │
│  └─────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────┘
```

---

## 文件结构设计

```
tauri-live2d-pet/
│
├── package.json                    # 前端依赖
├── Cargo.toml                      # Rust 工作区
├── vite.config.ts                  # Vite 配置
├── tsconfig.json                   # TypeScript 配置
├── uno.config.ts                   # UnoCSS 配置
├── .env.example                    # 环境变量
├── README.md                       # 项目说明
├── LICENSE                         # 许可证
│
├── src-tauri/                      # ===== Rust 后端 =====
│   ├── Cargo.toml                  # Rust 依赖
│   ├── build.rs                    # 构建脚本
│   │
│   ├── src/
│   │   ├── main.rs                 # 入口
│   │   ├── lib.rs                  # 库入口
│   │   │
│   │   ├── commands/               # Tauri 命令
│   │   │   ├── mod.rs
│   │   │   ├── pet.rs              # 宠物控制
│   │   │   ├── chat.rs             # 聊天
│   │   │   ├── settings.rs         # 设置
│   │   │   ├── window.rs           # 窗口
│   │   │   └── system.rs           # 系统
│   │   │
│   │   ├── services/               # 核心服务
│   │   │   ├── mod.rs
│   │   │   ├── window_manager.rs   # 窗口管理
│   │   │   ├── config_manager.rs   # 配置管理
│   │   │   ├── tray_manager.rs     # 系统托盘
│   │   │   ├── shortcut_manager.rs # 全局快捷键
│   │   │   ├── auto_launch.rs      # 开机自启
│   │   │   ├── screen_capture.rs   # 屏幕截图
│   │   │   └── updater.rs          # 自动更新
│   │   │
│   │   ├── ai/                     # AI 服务
│   │   │   ├── mod.rs
│   │   │   ├── provider.rs         # 提供商 trait
│   │   │   ├── openai.rs           # OpenAI
│   │   │   ├── claude.rs           # Claude
│   │   │   ├── gemini.rs           # Gemini
│   │   │   ├── ollama.rs           # Ollama (本地)
│   │   │   ├── chat_service.rs     # 聊天服务
│   │   │   ├── streaming.rs        # 流式响应
│   │   │   └── tool_use.rs         # 工具调用
│   │   │
│   │   ├── mcp/                    # MCP 集成
│   │   │   ├── mod.rs
│   │   │   ├── client.rs           # MCP 客户端
│   │   │   ├── transport.rs        # stdio 传输
│   │   │   ├── registry.rs         # 工具注册
│   │   │   └── executor.rs         # 执行器
│   │   │
│   │   ├── tts/                    # 语音合成
│   │   │   ├── mod.rs
│   │   │   ├── service.rs          # TTS 服务
│   │   │   ├── edge_tts.rs         # Edge TTS
│   │   │   └── lip_sync.rs         # 口型同步
│   │   │
│   │   ├── db/                     # 数据库
│   │   │   ├── mod.rs
│   │   │   ├── manager.rs          # 数据库管理器
│   │   │   ├── migrations.rs       # 迁移
│   │   │   ├── models/             # 数据模型
│   │   │   │   ├── mod.rs
│   │   │   │   ├── chat_message.rs
│   │   │   │   ├── pet_state.rs
│   │   │   │   ├── user_prefs.rs
│   │   │   │   └── memory.rs
│   │   │   └── queries/            # 查询
│   │   │       ├── mod.rs
│   │   │       ├── chat.rs
│   │   │       └── pet.rs
│   │   │
│   │   ├── events/                 # 事件系统
│   │   │   ├── mod.rs
│   │   │   ├── types.rs            # 事件类型
│   │   │   └── handler.rs          # 事件处理器
│   │   │
│   │   ├── models/                 # 共享模型
│   │   │   ├── mod.rs
│   │   │   ├── pet.rs              # 宠物模型
│   │   │   ├── chat.rs             # 聊天模型
│   │   │   ├── config.rs           # 配置模型
│   │   │   └── error.rs            # 错误模型
│   │   │
│   │   └── utils/                  # 工具函数
│   │       ├── mod.rs
│   │       ├── platform.rs         # 平台检测
│   │       └── logger.rs           # 日志
│   │
│   ├── migrations/                 # SQLite 迁移
│   │   ├── 001_init.sql
│   │   └── 002_chat_history.sql
│   │
│   └── icons/                      # 应用图标
│       ├── 32x32.png
│       ├── 128x128.png
│       ├── icon.ico
│       └── icon.icns
│
├── src/                            # ===== Vue 3 前端 =====
│   ├── index.html                  # HTML 入口
│   ├── main.ts                     # Vue 入口
│   ├── App.vue                     # 根组件
│   │
│   ├── views/                      # 页面
│   │   ├── PetView.vue             # 宠物视图
│   │   ├── ChatView.vue            # 聊天视图
│   │   └── SettingsView.vue        # 设置视图
│   │
│   ├── components/                 # 组件
│   │   ├── pet/                    # 宠物组件
│   │   │   ├── PetCanvas.vue       # Live2D 画布
│   │   │   ├── PetBubble.vue       # 对话气泡
│   │   │   ├── PetContextMenu.vue  # 右键菜单
│   │   │   └── PetStatus.vue       # 状态指示
│   │   ├── chat/                   # 聊天组件
│   │   │   ├── ChatMessage.vue     # 消息
│   │   │   ├── ChatInput.vue       # 输入
│   │   │   ├── ChatStream.vue      # 流式输出
│   │   │   └── MessageBubble.vue   # 气泡
│   │   ├── settings/               # 设置组件
│   │   │   ├── GeneralPanel.vue    # 通用
│   │   │   ├── ApiPanel.vue        # API
│   │   │   ├── AppearancePanel.vue # 外观
│   │   │   └── ShortcutPanel.vue   # 快捷键
│   │   └── ui/                     # 基础 UI
│   │       ├── BaseButton.vue
│   │       ├── BaseModal.vue
│   │       ├── BaseInput.vue
│   │       └── BaseTooltip.vue
│   │
│   ├── composables/                # 组合式函数
│   │   ├── usePetEngine.ts         # 宠物引擎
│   │   ├── useLive2D.ts            # Live2D 控制
│   │   ├── useAnimation.ts         # 动画控制
│   │   ├── useBehavior.ts          # 行为状态机
│   │   ├── useChatService.ts       # 聊天服务
│   │   ├── useAiProvider.ts        # AI 提供商
│   │   ├── useSpeechService.ts     # 语音服务
│   │   ├── useMcpClient.ts         # MCP 客户端
│   │   ├── useTauriBridge.ts       # Tauri 桥接
│   │   └── useTheme.ts             # 主题
│   │
│   ├── engine/                     # 渲染引擎
│   │   ├── types.ts                # 引擎类型
│   │   ├── Live2DRenderer.ts       # Live2D 渲染器
│   │   ├── SpriteRenderer.ts       # 精灵图渲染器
│   │   ├── AnimationController.ts  # 动画控制器
│   │   ├── HitDetector.ts          # 碰撞检测
│   │   └── LipSyncController.ts    # 口型同步
│   │
│   ├── behavior/                   # 行为系统
│   │   ├── StateMachine.ts         # 状态机
│   │   ├── states/                 # 状态
│   │   │   ├── BaseState.ts
│   │   │   ├── IdleState.ts
│   │   │   ├── WalkState.ts
│   │   │   ├── TalkState.ts
│   │   │   ├── SleepState.ts
│   │   │   └── ReactState.ts
│   │   ├── transitions.ts          # 转换规则
│   │   ├── triggers/               # 触发器
│   │   │   ├── TimeTrigger.ts
│   │   │   ├── InteractionTrigger.ts
│   │   │   └── SystemTrigger.ts
│   │   └── config.ts               # 行为配置
│   │
│   ├── stores/                     # Pinia 状态
│   │   ├── pet.ts                  # 宠物状态
│   │   ├── chat.ts                 # 聊天状态
│   │   ├── settings.ts             # 设置状态
│   │   └── ui.ts                   # UI 状态
│   │
│   ├── services/                   # 前端服务
│   │   ├── tauri.ts                # Tauri 通信
│   │   ├── events.ts               # 事件总线
│   │   └── storage.ts              # 本地存储
│   │
│   ├── types/                      # TypeScript 类型
│   │   ├── pet.ts
│   │   ├── chat.ts
│   │   ├── settings.ts
│   │   ├── tauri.ts
│   │   └── live2d.ts
│   │
│   ├── assets/                     # 静态资源
│   │   ├── models/                 # Live2D 模型
│   │   │   └── .gitkeep
│   │   ├── sprites/                # 精灵图
│   │   │   └── .gitkeep
│   │   ├── audio/                  # 音频
│   │   │   └── .gitkeep
│   │   ├── images/                 # 图片
│   │   │   ├── icons/
│   │   │   └── backgrounds/
│   │   └── styles/                 # 样式
│   │       ├── global.css
│   │       └── animations.css
│   │
│   └── utils/                      # 工具
│       ├── math.ts
│       ├── platform.ts
│       └── logger.ts
│
├── public/                         # 公共资源
│   └── live2d/                     # Live2D SDK
│       ├── live2dcubismcore.min.js
│       └── live2d.min.js
│
├── scripts/                        # 工具脚本
│   ├── build.ts                    # 构建
│   ├── dev.ts                      # 开发
│   └── package-model.ts            # 模型打包
│
├── tests/                          # 测试
│   ├── unit/
│   └── e2e/
│
└── docs/
    ├── architecture.md
    ├── development.md
    └── live2d-model.md             # Live2D 模型说明
```

---

## 核心模块设计

### 1. Live2D 渲染器

```typescript
// src/engine/Live2DRenderer.ts

import * as PIXI from 'pixi.js'
import { Live2DModel } from 'pixi-live2d-display'

export interface Live2DRendererOptions {
  width: number
  height: number
  transparent: boolean
}

export class Live2DRenderer {
  private app: PIXI.Application
  private model: Live2DModel | null = null
  private container: HTMLElement
  
  constructor(container: HTMLElement, options: Live2DRendererOptions) {
    this.container = container
    
    this.app = new PIXI.Application({
      width: options.width,
      height: options.height,
      transparent: options.transparent,
      backgroundAlpha: 0,
      antialias: true,
      resolution: window.devicePixelRatio || 1,
      autoDensity: true,
    })
    
    container.appendChild(this.app.view as HTMLCanvasElement)
  }
  
  async loadModel(modelPath: string): Promise<void> {
    // 销毁旧模型
    if (this.model) {
      this.app.stage.removeChild(this.model)
      this.model.destroy()
    }
    
    // 加载新模型
    this.model = await Live2DModel.from(modelPath)
    this.app.stage.addChild(this.model)
    
    // 自适应大小
    const scaleX = this.app.screen.width / this.model.width
    const scaleY = this.app.screen.height / this.model.height
    const scale = Math.min(scaleX, scaleY)
    this.model.scale.set(scale)
    
    // 居中
    this.model.x = (this.app.screen.width - this.model.width) / 2
    this.model.y = (this.app.screen.height - this.model.height) / 2
  }
  
  playMotion(group: string, index: number = 0, priority: number = 2): void {
    if (!this.model) return
    this.model.motion(group, index, priority)
  }
  
  setExpression(name: string): void {
    if (!this.model) return
    this.model.expression(name)
  }
  
  hitTest(x: number, y: number): string | null {
    if (!this.model) return null
    
    // pixi-live2d-display 的碰撞检测
    const hitAreas = this.model.hitTest(x, y)
    return hitAreas.length > 0 ? hitAreas[0] : null
  }
  
  // 口型同步
  setMouthOpenY(value: number): void {
    if (!this.model) return
    const internalModel = this.model.internalModel
    const coreModel = internalModel.coreModel
    
    // 设置嘴部参数
    const paramId = internalModel.settings.parameters.find(
      p => p.id === 'ParamMouthOpenY'
    )
    if (paramId) {
      coreModel.setParameterValueById('ParamMouthOpenY', value)
    }
  }
  
  // 头部追踪
  lookAt(x: number, y: number): void {
    if (!this.model) return
    // pixi-live2d-display 内置 lookAt 支持
    this.model.focus(x, y)
  }
  
  // 获取模型信息
  getModelInfo() {
    if (!this.model) return null
    return {
      width: this.model.width,
      height: this.model.height,
      motionGroups: this.model.internalModel.settings.motions,
    }
  }
  
  destroy(): void {
    if (this.model) {
      this.model.destroy()
    }
    this.app.destroy(true)
  }
}
```

### 2. 行为状态机

```typescript
// src/behavior/StateMachine.ts

export enum PetState {
  IDLE = 'idle',
  WALKING = 'walking',
  TALKING = 'talking',
  SLEEPING = 'sleeping',
  PLAYING = 'playing',
  REACTING = 'reacting',
}

export interface StateContext {
  pet: {
    x: number
    y: number
    direction: 'left' | 'right'
  }
  time: {
    hour: number
    minute: number
  }
  user: {
    lastInteraction: number
    isIdle: boolean
  }
}

export interface IState {
  name: PetState
  enter(ctx: StateContext): void
  update(dt: number, ctx: StateContext): void
  exit(ctx: StateContext): void
}

export interface Transition {
  from: PetState
  to: PetState
  condition: (ctx: StateContext) => boolean
  action?: (ctx: StateContext) => void
}

export class StateMachine {
  private states: Map<PetState, IState> = new Map()
  private transitions: Transition[] = []
  private currentState: IState | null = null
  private context: StateContext
  
  constructor(initialContext: StateContext) {
    this.context = initialContext
  }
  
  addState(state: IState): void {
    this.states.set(state.name, state)
  }
  
  addTransition(transition: Transition): void {
    this.transitions.push(transition)
  }
  
  setState(name: PetState): void {
    const newState = this.states.get(name)
    if (!newState || newState === this.currentState) return
    
    if (this.currentState) {
      this.currentState.exit(this.context)
    }
    
    this.currentState = newState
    this.currentState.enter(this.context)
  }
  
  update(dt: number): void {
    if (!this.currentState) return
    
    // 检查转换
    for (const transition of this.transitions) {
      if (transition.from === this.currentState.name) {
        if (transition.condition(this.context)) {
          if (transition.action) {
            transition.action(this.context)
          }
          this.setState(transition.to)
          break
        }
      }
    }
    
    this.currentState.update(dt, this.context)
  }
  
  getCurrentState(): PetState | null {
    return this.currentState?.name ?? null
  }
  
  updateContext(updater: (ctx: StateContext) => void): void {
    updater(this.context)
  }
}
```

### 3. Tauri 通信桥

```typescript
// src/composables/useTauriBridge.ts

import { invoke } from '@tauri-apps/api/core'
import { listen, emit } from '@tauri-apps/api/event'

export function useTauriBridge() {
  // ===== 宠物控制 =====
  const petCommands = {
    setWindowPosition: (x: number, y: number) =>
      invoke('set_pet_position', { x, y }),
      
    startWalking: () => invoke('start_walking'),
    stopWalking: () => invoke('stop_walking'),
    
    setAlwaysOnTop: (value: boolean) =>
      invoke('set_always_on_top', { value }),
      
    setClickThrough: (value: boolean) =>
      invoke('set_click_through', { value }),
  }
  
  // ===== 聊天服务 =====
  const chatCommands = {
    sendMessage: (message: string) =>
      invoke<string>('send_chat_message', { message }),
      
    sendMessageStream: async (message: string, onChunk: (chunk: string) => void) => {
      // 监听流式响应
      const unlisten = await listen<string>('chat-chunk', (event) => {
        onChunk(event.payload)
      })
      
      try {
        await invoke('send_chat_message_stream', { message })
      } finally {
        unlisten()
      }
    },
    
    getHistory: () => invoke<ChatMessage[]>('get_chat_history'),
    clearHistory: () => invoke('clear_chat_history'),
  }
  
  // ===== 设置 =====
  const settingsCommands = {
    getSettings: () => invoke<Settings>('get_settings'),
    updateSettings: (settings: Partial<Settings>) =>
      invoke('update_settings', { settings }),
      
    getApiConfig: () => invoke<ApiConfig>('get_api_config'),
    updateApiConfig: (config: Partial<ApiConfig>) =>
      invoke('update_api_config', { config }),
  }
  
  // ===== 窗口控制 =====
  const windowCommands = {
    openChat: () => invoke('open_chat_window'),
    openSettings: () => invoke('open_settings_window'),
    closeWindow: (name: string) => invoke('close_window', { name }),
    minimizeToTray: () => invoke('minimize_to_tray'),
  }
  
  // ===== 事件监听 =====
  const events = {
    onStateChange: (callback: (state: PetState) => void) =>
      listen<PetState>('pet-state-changed', (e) => callback(e.payload)),
      
    onTrayClick: (callback: () => void) =>
      listen('tray-icon-click', callback),
      
    onShortcut: (callback: (shortcut: string) => void) =>
      listen<string>('shortcut-triggered', (e) => callback(e.payload)),
  }
  
  return {
    pet: petCommands,
    chat: chatCommands,
    settings: settingsCommands,
    window: windowCommands,
    events,
  }
}
```

### 4. Rust AI 服务

```rust
// src-tauri/src/ai/chat_service.rs

use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;

use crate::models::{ChatMessage, ChatConfig};
use super::provider::AiProvider;

pub struct ChatService {
    provider: Arc<dyn AiProvider + Send + Sync>,
    history: Arc<RwLock<Vec<ChatMessage>>>,
    config: ChatConfig,
}

impl ChatService {
    pub fn new(provider: Arc<dyn AiProvider + Send + Sync>, config: ChatConfig) -> Self {
        Self {
            provider,
            history: Arc::new(RwLock::new(Vec::new())),
            config,
        }
    }
    
    /// 发送普通消息
    pub async fn send_message(&self, content: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 添加用户消息到历史
        let user_msg = ChatMessage {
            role: "user".to_string(),
            content: content.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        };
        
        self.history.write().await.push(user_msg);
        
        // 调用 AI
        let messages = self.history.read().await.clone();
        let response = self.provider.chat(&messages, &self.config).await?;
        
        // 添加助手消息到历史
        let assistant_msg = ChatMessage {
            role: "assistant".to_string(),
            content: response.clone(),
            timestamp: chrono::Utc::now().timestamp(),
        };
        
        self.history.write().await.push(assistant_msg);
        
        Ok(response)
    }
    
    /// 流式消息
    pub async fn send_message_stream<F>(&self, content: &str, on_chunk: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(String) + Send + 'static,
    {
        let user_msg = ChatMessage {
            role: "user".to_string(),
            content: content.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        };
        
        self.history.write().await.push(user_msg);
        
        let messages = self.history.read().await.clone();
        let mut full_response = String::new();
        
        let mut stream = self.provider.chat_stream(&messages, &self.config).await?;
        
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            full_response.push_str(&chunk);
            on_chunk(chunk);
        }
        
        let assistant_msg = ChatMessage {
            role: "assistant".to_string(),
            content: full_response,
            timestamp: chrono::Utc::now().timestamp(),
        };
        
        self.history.write().await.push(assistant_msg);
        
        Ok(())
    }
    
    /// 清空历史
    pub async fn clear_history(&self) {
        self.history.write().await.clear();
    }
    
    /// 获取历史
    pub async fn get_history(&self) -> Vec<ChatMessage> {
        self.history.read().await.clone()
    }
}
```

---

## Tauri 配置

```json
// src-tauri/tauri.conf.json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:1420",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "label": "main",
        "title": "Desktop Pet",
        "width": 300,
        "height": 400,
        "transparent": true,
        "decorations": false,
        "alwaysOnTop": true,
        "skipTaskbar": true,
        "resizable": false
      }
    ],
    "security": {
      "csp": null
    },
    "trayIcon": {
      "iconPath": "icons/icon.png",
      "iconAsTemplate": true
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "identifier": "com.desktoppet.app",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/icon.ico",
      "icons/icon.icns"
    ]
  }
}
```

---

## 扩展性设计

### 1. 渲染器抽象

```typescript
// src/engine/types.ts

export interface IRenderer {
  init(container: HTMLElement): Promise<void>
  loadModel(path: string): Promise<void>
  playAnimation(name: string, priority?: number): void
  hitTest(x: number, y: number): string | null
  setParameter(name: string, value: number): void
  destroy(): void
}

// 渲染器工厂
export class RendererFactory {
  static create(type: 'live2d' | 'sprite'): IRenderer {
    switch (type) {
      case 'live2d':
        return new Live2DRenderer()
      case 'sprite':
        return new SpriteRenderer()
    }
  }
}
```

### 2. AI 提供商注册

```typescript
// src/composables/useAiProvider.ts

interface AiProviderConfig {
  name: string
  apiKey: string
  baseUrl?: string
  model: string
}

class AiProviderRegistry {
  private providers: Map<string, AiProvider> = new Map()
  
  register(name: string, provider: AiProvider): void {
    this.providers.set(name, provider)
  }
  
  get(name: string): AiProvider | undefined {
    return this.providers.get(name)
  }
  
  list(): string[] {
    return Array.from(this.providers.keys())
  }
}

// 内置提供商
export const builtinProviders = {
  openai: { name: 'OpenAI', models: ['gpt-4o', 'gpt-4', 'gpt-3.5-turbo'] },
  claude: { name: 'Claude', models: ['claude-3-opus', 'claude-3-sonnet'] },
  gemini: { name: 'Gemini', models: ['gemini-pro', 'gemini-pro-vision'] },
  ollama: { name: 'Ollama', models: ['llama2', 'mistral', 'codellama'] },
}
```

### 3. 行为配置化

```json
// public/pet/behavior.json
{
  "states": {
    "idle": {
      "animations": ["idle_01", "idle_02", "idle_03"],
      "minDuration": 5000,
      "maxDuration": 30000,
      "nextStates": ["walk", "sleep", "play"]
    },
    "walk": {
      "animations": ["walk_left", "walk_right"],
      "speed": 1.5,
      "maxDistance": 200,
      "nextStates": ["idle"]
    }
  },
  "schedules": [
    { "time": "08:00", "action": "greet", "message": "早上好！" },
    { "time": "12:00", "action": "remind", "message": "该吃午饭了！" },
    { "time": "23:00", "action": "sleep" }
  ],
  "reactions": {
    "click_head": { "motion": "tap_head", "message": "不要摸头！" },
    "click_body": { "motion": "tap_body", "message": "嘿嘿~" },
    "idle_5min": { "motion": "wave", "message": "陪我玩一会儿吧" }
  }
}
```

---

## 构建与发布

### 开发环境

```bash
# 安装依赖
npm install

# 启动开发
npm run tauri dev

# 运行测试
npm run test
```

### 构建发布

```bash
# 构建应用
npm run tauri build

# 输出目录
# Windows: src-tauri/target/release/bundle/nsis/
# macOS: src-tauri/target/release/bundle/dmg/
# Linux: src-tauri/target/release/bundle/appimage/
```

### CI/CD 配置

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    strategy:
      matrix:
        platform: [windows-latest, macos-latest, ubuntu-latest]
    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      - uses: dtolnay/rust-toolchain@stable
      - run: npm install
      - run: npm run tauri build
      - uses: softprops/action-gh-release@v1
        with:
          files: |
            src-tauri/target/release/bundle/**/*.exe
            src-tauri/target/release/bundle/**/*.dmg
            src-tauri/target/release/bundle/**/*.AppImage
```

---

## 性能指标

| 指标 | 目标值 | 说明 |
|------|--------|------|
| 安装包大小 | ~15 MB | Windows NSIS |
| 内存占用 (idle) | ~60-80 MB | 含 Live2D 渲染 |
| 内存占用 (active) | ~100-150 MB | 含聊天窗口 |
| CPU 占用 (idle) | < 1% | 静态待机 |
| 启动时间 | < 1.5s | 冷启动 |
| 渲染帧率 | 60 FPS | Live2D 动画 |

---

## 总结

本方案结合了 Tauri 的轻量高效和 Live2D 的专业动画效果：

1. **轻量高效**：安装包仅 ~15MB，内存占用低
2. **专业动画**：Live2D 提供流畅的角色动画
3. **完整功能**：AI 聊天、MCP、TTS、行为系统
4. **扩展性强**：模块化设计，支持插件扩展
5. **跨平台**：Windows / macOS / Linux 全支持

这是功能、性能、开发效率的最佳平衡方案。

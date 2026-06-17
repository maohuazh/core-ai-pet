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

## 交互设计（增强）

> **决策记录**：基于"悬浮按钮"和"自定义快捷聊天"需求，确认 Tauri + Live2D 方案为首选。
> Vue 3 组件化开发 + CSS 动画可实现流畅的悬浮交互，同时保持轻量高效。

### 1. 悬浮菜单设计

鼠标移入宠物区域时，多个功能按钮从宠物周围弹出：

```
                    ┌─────────────────────┐
                    │   💬 聊天           │
        ┌───────────┤                     │
        │  🏠 首页   └─────────────────────┘
        │
   ┌────┴────┐
   │         │  ← Live2D 宠物
   │  Pet    │
   │         │
   └────┬────┘
        │
        │           ┌─────────────────────┐
        └───────────┤   ⚙️ 设置           │
                    └─────────────────────┘
                    ┌─────────────────────┐
                    │   🎨 主题           │
                    └─────────────────────┘
```

**交互逻辑：**
- 鼠标移入宠物区域 → 按钮从宠物周围弹出（环形/侧边栏）
- 鼠标移出 → 按钮淡出收回
- 点击按钮 → 触发对应功能
- 透明区域 → 点击穿透到桌面

### 2. 悬浮菜单组件

```vue
<!-- src/components/pet/PetHoverMenu.vue -->
<template>
  <Transition name="fade">
    <div v-if="visible" class="hover-menu" :style="menuStyle">
      <button
        v-for="(item, index) in items"
        :key="item.id"
        class="menu-btn"
        :style="getButtonStyle(index)"
        @click="handleClick(item)"
        @mouseenter="hoveredBtn = item.id"
        @mouseleave="hoveredBtn = null"
      >
        <span class="icon">{{ item.icon }}</span>
        <Transition name="tooltip">
          <span class="label" v-if="hoveredBtn === item.id">
            {{ item.label }}
          </span>
        </Transition>
      </button>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

export interface MenuItem {
  id: string
  icon: string
  label: string
  action: string
}

const props = withDefaults(defineProps<{
  items: MenuItem[]
  layout?: 'radial' | 'sidebar-left' | 'sidebar-right' | 'topbar'
  radius?: number
}>(), {
  layout: 'radial',
  radius: 60
})

const emit = defineEmits<{
  action: [action: string]
}>()

const visible = ref(false)
const hoveredBtn = ref<string | null>(null)

// 计算每个按钮的位置（环形布局）
function getButtonStyle(index: number) {
  if (props.layout === 'radial') {
    const totalItems = props.items.length
    const startAngle = -90 // 从顶部开始
    const angleStep = 180 / (totalItems - 1) // 半圆分布
    const angle = startAngle + angleStep * index
    const radian = (angle * Math.PI) / 180
    
    return {
      '--offset-x': `${Math.cos(radian) * props.radius}px`,
      '--offset-y': `${Math.sin(radian) * props.radius}px`,
    }
  }
  return {}
}

const menuStyle = computed(() => ({
  '--layout': props.layout,
}))

function show() { visible.value = true }
function hide() { visible.value = false }

function handleClick(item: MenuItem) {
  emit('action', item.action)
}

defineExpose({ show, hide })
</script>

<style scoped>
.hover-menu {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  pointer-events: none;
}

.menu-btn {
  position: absolute;
  width: 44px;
  height: 44px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.95);
  border: 2px solid rgba(255, 255, 255, 0.3);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  pointer-events: auto;
  
  /* 从中心弹出动画 */
  transform: translate(
    calc(var(--offset-x, 0) * 0.3),
    calc(var(--offset-y, 0) * 0.3)
  );
  opacity: 0;
  animation: popIn 0.3s ease-out forwards;
  animation-delay: calc(var(--index) * 50ms);
  
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  transition: all 0.2s ease;
}

.menu-btn:hover {
  transform: translate(var(--offset-x, 0), var(--offset-y, 0)) scale(1.15);
  background: white;
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.2);
}

.menu-btn .icon {
  font-size: 20px;
}

.menu-btn .label {
  position: absolute;
  bottom: -28px;
  left: 50%;
  transform: translateX(-50%);
  background: rgba(0, 0, 0, 0.8);
  color: white;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 12px;
  white-space: nowrap;
}

@keyframes popIn {
  from {
    opacity: 0;
    transform: translate(0, 0) scale(0.5);
  }
  to {
    opacity: 1;
    transform: translate(var(--offset-x), var(--offset-y)) scale(1);
  }
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
```

### 3. 快捷聊天窗口设计

```
┌────────────────────────────────────────┐
│  💬 快捷聊天                      [×]   │
├────────────────────────────────────────┤
│  快捷指令                               │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐  │
│  │ 🌐 翻译  │ │ 📝 总结  │ │ 💡 解释  │  │
│  └─────────┘ └─────────┘ └─────────┘  │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐  │
│  │ ✉️ 邮件  │ │ 🧠 风暴  │ │ + 自定义 │  │
│  └─────────┘ └─────────┘ └─────────┘  │
├────────────────────────────────────────┤
│  对话区域                               │
│  ┌──────────────────────────────────┐  │
│  │ 🤖 你好！有什么可以帮你的？       │  │
│  └──────────────────────────────────┘  │
│  ┌──────────────────────────────────┐  │
│  │ 👤 帮我翻译这段代码              │  │
│  └──────────────────────────────────┘  │
├────────────────────────────────────────┤
│  [输入消息...]                    [➤]   │
└────────────────────────────────────────┘
```

### 4. 快捷聊天组件

```vue
<!-- src/components/chat/QuickChat.vue -->
<template>
  <div class="quick-chat">
    <!-- 快捷指令区 -->
    <div class="quick-commands">
      <div
        v-for="cmd in commands"
        :key="cmd.id"
        class="command-chip"
        @click="executeCommand(cmd)"
      >
        <span class="cmd-icon">{{ cmd.icon }}</span>
        <span class="cmd-label">{{ cmd.label }}</span>
      </div>
      <div class="command-chip add" @click="openCommandEditor">
        <span class="cmd-icon">+</span>
        <span class="cmd-label">自定义</span>
      </div>
    </div>
    
    <!-- 消息列表 -->
    <div class="messages" ref="messagesRef">
      <div
        v-for="msg in messages"
        :key="msg.id"
        class="message"
        :class="msg.role"
      >
        <div class="avatar">{{ msg.role === 'user' ? '👤' : '🤖' }}</div>
        <div class="content">{{ msg.content }}</div>
      </div>
    </div>
    
    <!-- 输入区 -->
    <div class="input-area">
      <input
        v-model="inputText"
        @keyup.enter="sendMessage"
        placeholder="输入消息..."
      />
      <button @click="sendMessage" :disabled="!inputText.trim()">
        ➤
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick } from 'vue'

interface Command {
  id: string
  icon: string
  label: string
  prompt: string
}

interface Message {
  id: string
  role: 'user' | 'assistant'
  content: string
  timestamp: number
}

// 从 store 加载指令
const commands = ref<Command[]>([
  { id: '1', icon: '🌐', label: '翻译', prompt: '请翻译以下内容：' },
  { id: '2', icon: '📝', label: '总结', prompt: '请总结以下内容的要点：' },
  { id: '3', icon: '💡', label: '解释', prompt: '请解释以下代码的含义：' },
  { id: '4', icon: '✉️', label: '邮件', prompt: '帮我写一封邮件，主题是：' },
  { id: '5', icon: '🧠', label: '风暴', prompt: '让我们一起头脑风暴：' },
])

const messages = ref<Message[]>([])
const inputText = ref('')
const messagesRef = ref<HTMLElement>()

function executeCommand(cmd: Command) {
  inputText.value = cmd.prompt
  // 自动聚焦输入框
  nextTick(() => {
    const input = document.querySelector('.input-area input') as HTMLInputElement
    input?.focus()
  })
}

async function sendMessage() {
  const content = inputText.value.trim()
  if (!content) return
  
  // 添加用户消息
  messages.value.push({
    id: Date.now().toString(),
    role: 'user',
    content,
    timestamp: Date.now(),
  })
  
  inputText.value = ''
  
  // 调用 AI 服务（通过 Tauri）
  // const response = await invoke('send_chat_message', { message: content })
  
  // 滚动到底部
  scrollToBottom()
}

function scrollToBottom() {
  nextTick(() => {
    if (messagesRef.value) {
      messagesRef.value.scrollTop = messagesRef.value.scrollHeight
    }
  })
}

function openCommandEditor() {
  // 打开指令编辑器弹窗
}
</script>

<style scoped>
.quick-chat {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: white;
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
}

.quick-commands {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  padding: 12px;
  border-bottom: 1px solid #eee;
}

.command-chip {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 12px;
  background: #f5f5f5;
  border-radius: 20px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.command-chip:hover {
  background: #e8e8e8;
  transform: translateY(-1px);
}

.command-chip.add {
  background: #f0f7ff;
  color: #1976d2;
}

.messages {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.message {
  display: flex;
  gap: 8px;
  max-width: 80%;
}

.message.user {
  align-self: flex-end;
  flex-direction: row-reverse;
}

.message .avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: #f0f0f0;
  display: flex;
  align-items: center;
  justify-content: center;
}

.message .content {
  padding: 10px 14px;
  border-radius: 16px;
  background: #f5f5f5;
}

.message.user .content {
  background: #1976d2;
  color: white;
}

.input-area {
  display: flex;
  gap: 8px;
  padding: 12px;
  border-top: 1px solid #eee;
}

.input-area input {
  flex: 1;
  padding: 10px 16px;
  border: 1px solid #ddd;
  border-radius: 24px;
  outline: none;
  transition: border-color 0.2s;
}

.input-area input:focus {
  border-color: #1976d2;
}

.input-area button {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: #1976d2;
  color: white;
  border: none;
  cursor: pointer;
  transition: all 0.2s;
}

.input-area button:hover:not(:disabled) {
  background: #1565c0;
  transform: scale(1.05);
}

.input-area button:disabled {
  background: #ccc;
  cursor: not-allowed;
}
</style>
```

### 5. 指令管理器

```typescript
// src/stores/commands.ts

import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

export interface QuickCommand {
  id: string
  icon: string
  label: string
  prompt: string
  category: string
  createdAt: number
}

export const useCommandsStore = defineStore('commands', () => {
  const commands = ref<QuickCommand[]>([])
  
  // 从本地存储加载
  function loadFromStorage() {
    const saved = localStorage.getItem('quick-commands')
    if (saved) {
      commands.value = JSON.parse(saved)
    } else {
      // 默认指令
      commands.value = [
        { id: '1', icon: '🌐', label: '翻译', prompt: '请翻译：', category: '常用', createdAt: Date.now() },
        { id: '2', icon: '📝', label: '总结', prompt: '请总结：', category: '常用', createdAt: Date.now() },
        { id: '3', icon: '💡', label: '解释', prompt: '请解释：', category: '常用', createdAt: Date.now() },
      ]
    }
  }
  
  // 保存到本地存储
  function saveToStorage() {
    localStorage.setItem('quick-commands', JSON.stringify(commands.value))
  }
  
  // 监听变化自动保存
  watch(commands, saveToStorage, { deep: true })
  
  // 添加指令
  function addCommand(cmd: Omit<QuickCommand, 'id' | 'createdAt'>) {
    commands.value.push({
      ...cmd,
      id: Date.now().toString(),
      createdAt: Date.now(),
    })
  }
  
  // 更新指令
  function updateCommand(id: string, updates: Partial<QuickCommand>) {
    const index = commands.value.findIndex(c => c.id === id)
    if (index !== -1) {
      commands.value[index] = { ...commands.value[index], ...updates }
    }
  }
  
  // 删除指令
  function removeCommand(id: string) {
    commands.value = commands.value.filter(c => c.id !== id)
  }
  
  // 按类别分组
  function getByCategory() {
    const grouped: Record<string, QuickCommand[]> = {}
    commands.value.forEach(cmd => {
      if (!grouped[cmd.category]) {
        grouped[cmd.category] = []
      }
      grouped[cmd.category].push(cmd)
    })
    return grouped
  }
  
  return {
    commands,
    loadFromStorage,
    addCommand,
    updateCommand,
    removeCommand,
    getByCategory,
  }
})
```

### 6. 更新后的文件结构

```
src/
├── components/
│   ├── pet/
│   │   ├── PetCanvas.vue           # Live2D 画布
│   │   ├── PetHoverMenu.vue        # [新增] 悬浮菜单
│   │   ├── PetContextMenu.vue      # 右键菜单
│   │   └── PetEffects.vue          # 特效组件
│   ├── chat/
│   │   ├── ChatView.vue            # 聊天主视图
│   │   ├── QuickChat.vue           # [新增] 快捷聊天
│   │   ├── CommandEditor.vue       # [新增] 指令编辑器
│   │   └── ChatMessage.vue         # 消息组件
│   └── settings/
│       ├── GeneralPanel.vue
│       ├── ApiPanel.vue
│       ├── AppearancePanel.vue
│       ├── ShortcutPanel.vue
│       └── CommandPanel.vue        # [新增] 指令管理
│
├── stores/
│   ├── pet.ts
│   ├── chat.ts
│   ├── settings.ts
│   ├── ui.ts
│   └── commands.ts                 # [新增] 指令状态
│
├── composables/
│   ├── usePetEngine.ts
│   ├── useLive2D.ts
│   ├── useAnimation.ts
│   ├── useBehavior.ts
│   ├── useChatService.ts
│   ├── useAiProvider.ts
│   ├── useSpeechService.ts
│   ├── useMcpClient.ts
│   ├── useTauriBridge.ts
│   ├── useTheme.ts
│   ├── useHoverMenu.ts             # [新增] 悬浮菜单逻辑
│   └── useQuickChat.ts             # [新增] 快捷聊天逻辑
│
└── types/
    ├── pet.ts
    ├── chat.ts
    ├── settings.ts
    ├── tauri.ts
    ├── live2d.ts
    └── commands.ts                 # [新增] 指令类型
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
4. **交互体验**：悬浮菜单 + 快捷聊天，操作流畅自然
5. **扩展性强**：模块化设计，支持插件扩展
6. **跨平台**：Windows / macOS / Linux 全支持

### 方案选择决策

| 需求 | 选择理由 |
|------|---------|
| **悬浮多按钮** | Vue 3 组件 + CSS 动画可实现流畅的环形/侧边栏弹出效果 |
| **自定义快捷聊天** | Pinia 状态管理 + localStorage 持久化，支持指令的增删改查 |
| **交互体验** | Tauri 窗口透明 + 点击穿透，鼠标事件处理精确 |
| **性能平衡** | 相比 Electron 包体积减少 90%，内存减少 60% |

这是功能、性能、交互体验的最佳平衡方案。

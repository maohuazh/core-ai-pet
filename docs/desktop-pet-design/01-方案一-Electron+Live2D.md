# 方案一：Electron + Live2D 架构设计

## 技术选型

| 层次 | 技术 | 版本 |
|------|------|------|
| 桌面框架 | Electron | 30+ |
| 渲染引擎 | PixiJS | 7.x |
| Live2D | pixi-live2d-display | 0.4+ |
| 前端框架 | Vue 3 + TypeScript | 3.4+ |
| 状态管理 | Pinia | 2.x |
| 构建工具 | Vite | 5.x |
| AI SDK | Vercel AI SDK | 3.x |
| 样式 | Tailwind CSS | 3.x |

---

## 架构概览

```
┌─────────────────────────────────────────────────────────────┐
│                     Electron Main Process                    │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌───────────────┐  │
│  │ Window   │ │ Tray     │ │ IPC      │ │ Auto-Launch   │  │
│  │ Manager  │ │ Manager  │ │ Hub      │ │ Manager       │  │
│  └──────────┘ └──────────┘ └──────────┘ └───────────────┘  │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌───────────────┐  │
│  │ Store    │ │ Updater  │ │ Shortcut │ │ Screen        │  │
│  │ Manager  │ │          │ │ Manager  │ │ Capture       │  │
│  └──────────┘ └──────────┘ └──────────┘ └───────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │ IPC
┌─────────────────────────────────────────────────────────────┐
│                   Renderer Process (Vue 3)                   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │                   App Shell (Vue)                     │   │
│  │  ┌────────┐ ┌────────┐ ┌────────┐ ┌──────────────┐  │   │
│  │  │ Pet    │ │ Chat   │ │ Settings│ │ Menu Overlay │  │   │
│  │  │ View   │ │ View   │ │ View   │ │              │  │   │
│  │  └────────┘ └────────┘ └────────┘ └──────────────┘  │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │               Core Services (Composables)            │   │
│  │  ┌──────────────┐ ┌──────────────┐ ┌─────────────┐  │   │
│  │  │ Animation    │ │ Behavior     │ │ Interaction │  │   │
│  │  │ Engine       │ │ State Machine│ │ Handler     │  │   │
│  │  └──────────────┘ └──────────────┘ └─────────────┘  │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              Rendering Layer                          │   │
│  │  ┌──────────────┐ ┌──────────────┐ ┌─────────────┐  │   │
│  │  │ Live2D       │ │ Sprite       │ │ Hybrid      │  │   │
│  │  │ Renderer     │ │ Renderer     │ │ Renderer    │  │   │
│  │  └──────────────┘ └──────────────┘ └─────────────┘  │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

---

## 文件结构设计

```
electron-desktop-pet/
│
├── package.json                    # 项目配置
├── electron-builder.json5          # 打包配置
├── vite.config.ts                  # Vite 构建配置
├── tsconfig.json                   # TypeScript 配置
├── tailwind.config.ts              # Tailwind 配置
├── .env.example                    # 环境变量示例
├── README.md                       # 项目说明
│
├── src/
│   │
│   ├── main/                       # ===== Electron 主进程 =====
│   │   ├── index.ts                # 主进程入口
│   │   │
│   │   ├── windows/                # 窗口管理
│   │   │   ├── pet-window.ts       # 宠物透明窗口
│   │   │   ├── chat-window.ts      # 聊天窗口
│   │   │   ├── settings-window.ts  # 设置窗口
│   │   │   └── wallpaper-window.ts # 壁纸窗口
│   │   │
│   │   ├── ipc/                    # IPC 通信
│   │   │   ├── handlers.ts         # IPC 处理器注册
│   │   │   ├── channels.ts         # 通道定义
│   │   │   └── bridge.ts          # 主进程桥接
│   │   │
│   │   ├── services/               # 主进程服务
│   │   │   ├── tray.ts             # 系统托盘
│   │   │   ├── auto-launch.ts      # 开机自启
│   │   │   ├── shortcut.ts         # 全局快捷键
│   │   │   ├── screen-capture.ts   # 屏幕截图
│   │   │   ├── store.ts            # electron-store 配置持久化
│   │   │   └── updater.ts          # 自动更新
│   │   │
│   │   └── utils/                  # 工具函数
│   │       ├── platform.ts         # 平台检测
│   │       └── logger.ts           # 日志
│   │
│   ├── preload/                    # ===== 预加载脚本 =====
│   │   ├── index.ts                # 预加载入口
│   │   ├── pet-bridge.ts           # 宠物窗口 API
│   │   ├── chat-bridge.ts          # 聊天窗口 API
│   │   └── settings-bridge.ts      # 设置窗口 API
│   │
│   ├── renderer/                   # ===== 渲染进程 (Vue 3) =====
│   │   ├── index.html              # HTML 入口
│   │   ├── main.ts                 # Vue 入口
│   │   ├── App.vue                 # 根组件
│   │   │
│   │   ├── pages/                  # 页面
│   │   │   ├── pet/                # 宠物页面
│   │   │   │   └── PetView.vue     # 宠物主视图
│   │   │   ├── chat/               # 聊天页面
│   │   │   │   └── ChatView.vue    # 聊天主视图
│   │   │   └── settings/           # 设置页面
│   │   │       └── SettingsView.vue # 设置主视图
│   │   │
│   │   ├── components/             # 组件库
│   │   │   ├── pet/                # 宠物相关组件
│   │   │   │   ├── PetCanvas.vue       # Live2D/精灵图画布
│   │   │   │   ├── PetBubble.vue       # 对话气泡
│   │   │   │   ├── PetContextMenu.vue  # 右键菜单
│   │   │   │   └── PetStatus.vue       # 状态指示
│   │   │   ├── chat/               # 聊天相关组件
│   │   │   │   ├── ChatMessage.vue     # 消息组件
│   │   │   │   ├── ChatInput.vue       # 输入组件
│   │   │   │   ├── ChatStream.vue      # 流式输出
│   │   │   │   └── MessageBubble.vue   # 消息气泡
│   │   │   ├── settings/           # 设置相关组件
│   │   │   │   ├── GeneralSettings.vue  # 通用设置
│   │   │   │   ├── ApiSettings.vue      # API 设置
│   │   │   │   ├── AppearanceSettings.vue # 外观设置
│   │   │   │   └── ShortcutSettings.vue   # 快捷键设置
│   │   │   └── common/             # 通用组件
│   │   │       ├── BaseButton.vue
│   │   │       ├── BaseModal.vue
│   │   │       └── BaseTooltip.vue
│   │   │
│   │   ├── composables/            # 组合式函数（核心服务层）
│   │   │   ├── usePetEngine.ts         # 宠物引擎
│   │   │   ├── useAnimationEngine.ts   # 动画引擎
│   │   │   ├── useBehaviorMachine.ts   # 行为状态机
│   │   │   ├── useInteraction.ts       # 交互处理
│   │   │   ├── useChatService.ts       # 聊天服务
│   │   │   ├── useAiProvider.ts        # AI 提供商
│   │   │   ├── useSpeechService.ts     # 语音服务
│   │   │   ├── useMcpClient.ts         # MCP 客户端
│   │   │   ├── useStore.ts             # 配置存储
│   │   │   └── useTheme.ts             # 主题管理
│   │   │
│   │   ├── renderers/              # 渲染器层
│   │   │   ├── types.ts                # 渲染器接口定义
│   │   │   ├── live2d/             # Live2D 渲染器
│   │   │   │   ├── Live2DRenderer.ts   # Live2D 渲染器实现
│   │   │   │   ├── Live2DModel.ts      # 模型管理
│   │   │   │   ├── Live2DHitTest.ts    # 碰撞检测
│   │   │   │   └── Live2DMotion.ts     # 动作管理
│   │   │   ├── sprite/             # 精灵图渲染器
│   │   │   │   ├── SpriteRenderer.ts   # 精灵图渲染器实现
│   │   │   │   ├── SpriteSheet.ts      # 精灵图集管理
│   │   │   │   └── SpriteAnimator.ts   # 帧动画播放
│   │   │   └── hybrid/             # 混合渲染器
│   │   │       └── HybridRenderer.ts   # Live2D + 精灵图混合
│   │   │
│   │   ├── behavior/               # 行为系统
│   │   │   ├── types.ts                # 行为类型定义
│   │   │   ├── StateMachine.ts         # 状态机核心
│   │   │   ├── states/             # 状态定义
│   │   │   │   ├── IdleState.ts        # 空闲状态
│   │   │   │   ├── WalkingState.ts     # 行走状态
│   │   │   │   ├── TalkingState.ts     # 说话状态
│   │   │   │   ├── SleepingState.ts    # 睡眠状态
│   │   │   │   ├── PlayingState.ts     # 玩耍状态
│   │   │   │   └── ReactingState.ts    # 反应状态
│   │   │   ├── transitions/        # 转换规则
│   │   │   │   └── defaultTransitions.ts
│   │   │   └── triggers/           # 触发器
│   │   │       ├── TimeTrigger.ts      # 时间触发
│   │   │       ├── InteractionTrigger.ts # 交互触发
│   │   │       └── SystemTrigger.ts    # 系统事件触发
│   │   │
│   │   ├── ai/                     # AI 集成层
│   │   │   ├── types.ts                # AI 类型定义
│   │   │   ├── providers/          # AI 提供商
│   │   │   │   ├── BaseProvider.ts     # 基类
│   │   │   │   ├── OpenAIProvider.ts   # OpenAI
│   │   │   │   ├── ClaudeProvider.ts   # Claude
│   │   │   │   ├── GeminiProvider.ts   # Gemini
│   │   │   │   └── CustomProvider.ts   # 自定义
│   │   │   ├── chat/               # 聊天逻辑
│   │   │   │   ├── ChatManager.ts      # 聊天管理器
│   │   │   │   ├── MessageHistory.ts   # 消息历史
│   │   │   │   └── StreamParser.ts     # 流式解析
│   │   │   ├── mcp/                # MCP 集成
│   │   │   │   ├── McpClient.ts        # MCP 客户端
│   │   │   │   ├── ToolRegistry.ts     # 工具注册
│   │   │   │   └── ToolExecutor.ts     # 工具执行
│   │   │   └── tts/                # 语音合成
│   │   │       ├── TtsService.ts       # TTS 服务
│   │   │       └── LipSync.ts          # 口型同步
│   │   │
│   │   ├── stores/                 # Pinia 状态管理
│   │   │   ├── pet.ts                  # 宠物状态
│   │   │   ├── chat.ts                 # 聊天状态
│   │   │   ├── settings.ts             # 设置状态
│   │   │   └── ui.ts                   # UI 状态
│   │   │
│   │   ├── assets/                 # 静态资源
│   │   │   ├── models/             # Live2D 模型
│   │   │   │   └── .gitkeep
│   │   │   ├── sprites/            # 精灵图
│   │   │   │   └── .gitkeep
│   │   │   ├── audio/              # 音频文件
│   │   │   │   └── .gitkeep
│   │   │   ├── images/             # 图片资源
│   │   │   │   ├── icons/
│   │   │   │   └── backgrounds/
│   │   │   └── styles/             # 全局样式
│   │   │       ├── main.css
│   │   │       ├── themes/
│   │   │       └── animations.css
│   │   │
│   │   └── types/                  # TypeScript 类型
│   │       ├── electron.d.ts           # Electron API 类型
│   │       ├── pet.ts                  # 宠物相关类型
│   │       ├── chat.ts                 # 聊天相关类型
│   │       └── settings.ts             # 设置相关类型
│   │
│   └── shared/                     # ===== 共享代码 =====
│       ├── constants.ts                # 常量定义
│       ├── config.ts                   # 默认配置
│       └── types.ts                    # 共享类型
│
├── resources/                      # 构建资源
│   ├── icon.ico                    # Windows 图标
│   ├── icon.icns                   # macOS 图标
│   └── icon.png                    # Linux 图标
│
├── scripts/                        # 构建脚本
│   ├── build.ts                    # 构建脚本
│   └── dev.ts                      # 开发脚本
│
├── tests/                          # 测试
│   ├── unit/                       # 单元测试
│   └── e2e/                        # 端到端测试
│
└── docs/                           # 文档
    ├── architecture.md             # 架构文档
    ├── api.md                      # API 文档
    └── development.md              # 开发指南
```

---

## 核心模块设计

### 1. 渲染器接口

```typescript
// src/renderer/renderers/types.ts

export interface IRenderer {
  /** 初始化渲染器 */
  init(container: HTMLElement): Promise<void>
  
  /** 加载模型 */
  loadModel(modelPath: string): Promise<void>
  
  /** 播放动画 */
  playAnimation(animationName: string, priority?: number): void
  
  /** 设置参数 */
  setParameter(name: string, value: number): void
  
  /** 碰撞检测 */
  hitTest(x: number, y: number): string | null
  
  /** 销毁 */
  destroy(): void
}

export type RendererType = 'live2d' | 'sprite' | 'hybrid'
```

### 2. 行为状态机

```typescript
// src/renderer/behavior/types.ts

export enum PetState {
  IDLE = 'idle',
  WALKING = 'walking',
  TALKING = 'talking',
  SLEEPING = 'sleeping',
  PLAYING = 'playing',
  REACTING = 'reacting',
}

export interface IState {
  name: PetState
  enter(): void
  exit(): void
  update(deltaTime: number): void
}

export interface ITransition {
  from: PetState
  to: PetState
  condition: () => boolean
  execute?: () => void
}

export interface IBehaviorMachine {
  currentState: PetState
  addState(state: IState): void
  addTransition(transition: ITransition): void
  update(deltaTime: number): void
  trigger(eventName: string, data?: any): void
}
```

### 3. AI 提供商接口

```typescript
// src/renderer/ai/types.ts

export interface IChatMessage {
  role: 'system' | 'user' | 'assistant'
  content: string
  timestamp: number
}

export interface IStreamingResponse {
  onChunk: (chunk: string) => void
  onComplete: () => void
  onError: (error: Error) => void
}

export interface IAiProvider {
  readonly name: string
  readonly supportedModels: string[]
  
  /** 发送聊天请求 */
  chat(messages: IChatMessage[], options?: ChatOptions): Promise<string>
  
  /** 流式聊天 */
  chatStream(messages: IChatMessage[], options?: ChatOptions): AsyncIterable<string>
  
  /** 工具调用 */
  chatWithTools?(messages: IChatMessage[], tools: ITool[]): Promise<IToolCall[]>
}

export interface IAiProviderFactory {
  create(config: ProviderConfig): IAiProvider
}
```

---

## 扩展性设计

### 1. 渲染器插件化

渲染器通过接口抽象，可以自由扩展：
- **Live2D 渲染器** - 高质量角色动画
- **精灵图渲染器** - 轻量级帧动画
- **Spine 渲染器** - 可扩展支持 Spine 动画
- **Lottie 渲染器** - 可扩展支持 After Effects 动画

### 2. AI 提供商插件化

AI 提供商通过工厂模式创建，支持动态注册：
```typescript
// 注册自定义提供商
AiProviderRegistry.register('my-provider', new MyProviderFactory())

// 使用提供商
const provider = AiProviderRegistry.create('openai', config)
```

### 3. 行为脚本化

行为状态机支持外部配置：
```json
{
  "states": ["idle", "walking", "talking"],
  "transitions": [
    { "from": "idle", "to": "walking", "condition": "time > 30s" },
    { "from": "walking", "to": "idle", "condition": "distance > 200px" }
  ]
}
```

### 4. 主题系统

通过 CSS 变量和配置实现主题切换：
```typescript
// 自定义主题
const theme: Theme = {
  colors: { primary: '#ff6b6b', background: 'transparent' },
  animations: { speed: 1.0 },
  layout: { bubblePosition: 'top' }
}
```

---

## 性能优化策略

1. **渲染优化**
   - 使用 requestAnimationFrame 控制渲染循环
   - 降低非活动状态帧率（idle: 30fps, active: 60fps）
   - 窗口不可见时暂停渲染

2. **内存优化**
   - Live2D 模型懒加载
   - 精灵图按需加载
   - 聊天历史分页

3. **启动优化**
   - 主窗口延迟创建
   - 配置异步加载
   - 模型预缓存

---

## 部署方案

```json
{
  "build": {
    "appId": "com.desktoppet.app",
    "productName": "Desktop Pet",
    "directories": {
      "output": "dist"
    },
    "win": {
      "target": ["nsis", "portable"],
      "icon": "resources/icon.ico"
    },
    "mac": {
      "target": ["dmg", "zip"],
      "icon": "resources/icon.icns"
    },
    "linux": {
      "target": ["AppImage", "deb"],
      "icon": "resources/icon.png"
    }
  }
}
```

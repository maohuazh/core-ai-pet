# CoreAIpet 架构设计文档

> Version: 1.0 | Date: 2026-06-15
> 基于 PRD V1.0 MVP 需求，面向扩展性的项目架构设计

---

## 1. 设计原则

| 原则 | 说明 |
|------|------|
| **宿主极简** | Desktop 只做壳：窗口管理 + 渲染 + AI + 插件加载。业务逻辑全部在插件中 |
| **插件即功能** | 所有用户可见功能（邮件/消息/任务）都是独立插件，可安装/卸载/更新 |
| **Provider 模式** | 同一类功能（如消息）支持多个 Provider，每个 Provider 是独立插件 |
| **接口隔离** | Core 层只有接口和模型，零实现。插件只依赖 Core，不依赖 Desktop |
| **动态菜单** | 径向菜单不硬编码，根据已安装插件动态生成 |

---

## 2. 解决方案结构

```
CoreAIpet.sln
│
├── src/
│   ├── CoreAIpet.Core/                     # 类库: 接口 + 模型 + 枚举 + 事件
│   │   ├── Interfaces/
│   │   │   ├── IPlugin.cs                  # 插件生命周期: Load/Activate/Execute/Unload
│   │   │   ├── IPluginContext.cs            # 注入给插件的服务上下文
│   │   │   ├── IPluginHost.cs              # 插件宿主服务 (注册菜单项等)
│   │   │   ├── IMessageProvider.cs         # 消息源接口 ★ 新增
│   │   │   ├── IEmailProvider.cs           # 邮箱源接口 ★ 新增
│   │   │   ├── IAIService.cs               # 单一 AI 后端接口
│   │   │   ├── IAIServiceProvider.cs       # AI 服务路由（策略模式）
│   │   │   ├── IConfigService.cs           # 配置读写
│   │   │   ├── IPositionService.cs         # 窗口位置持久化
│   │   │   ├── ICharacterController.cs     # 角色状态/动画控制
│   │   │   ├── IWindowService.cs           # 窗口操作（显隐/穿透/置顶）
│   │   │   ├── IEventBus.cs                # 发布-订阅事件总线
│   │   │   ├── ILogService.cs              # 日志服务
│   │   │   └── IPluginMenuProvider.cs      # 为径向菜单提供菜单项
│   │   │
│   │   ├── Models/
│   │   │   ├── Settings/
│   │   │   │   └── AppSettings.cs          # 配置根对象 + 所有配置子类型
│   │   │   ├── Chat/
│   │   │   │   └── ChatModels.cs           # ChatMessage, ChatRequest, ChatResponse
│   │   │   ├── Plugin/
│   │   │   │   └── PluginModels.cs         # PluginManifest, PluginMenuItem, PluginMetadata
│   │   │   └── Character/
│   │   │       └── CharacterModels.cs      # CharacterState enum, EyePosition
│   │   │
│   │   ├── Events/
│   │   │   └── Events.cs                   # IEvent + 5 个事件类
│   │   │
│   │   └── Exceptions/
│   │       └── Exceptions.cs               # PluginLoadException, AIServiceException, ...
│   │
│   ├── CoreAIpet.Desktop/                  # WPF 应用: 宿主壳 + 服务实现 + UI
│   │   ├── App.xaml / App.xaml.cs          # Generic Host 构建 + 启动
│   │   ├── CompositionRoot.cs              # DI 注册组合根
│   │   ├── SingleInstanceGuard.cs          # Mutex 单实例检测
│   │   │
│   │   ├── Services/
│   │   │   ├── Configuration/
│   │   │   │   ├── ConfigService.cs        # IConfigService: 配置读写 + 变更通知
│   │   │   │   ├── JsonConfigStore.cs      # JSON 文件原子读写
│   │   │   │   └── PositionService.cs      # IPositionService: 位置持久化
│   │   │   │
│   │   │   ├── AI/
│   │   │   │   ├── AIServiceProvider.cs    # IAIServiceProvider: 路由 + 切换后端
│   │   │   │   ├── OpenAIService.cs        # IAIService: OpenAI Chat Completions
│   │   │   │   ├── AzureOpenAIService.cs   # IAIService: Azure OpenAI
│   │   │   │   ├── OllamaService.cs        # IAIService: Ollama REST API
│   │   │   │   ├── ChatSessionManager.cs   # 对话上下文管理
│   │   │   │   └── SystemPromptBuilder.cs  # 系统提示词构建
│   │   │   │
│   │   │   ├── Plugins/
│   │   │   │   ├── PluginManager.cs        # 扫描/加载/卸载插件（内置 + 外部）
│   │   │   │   ├── PluginLoadContext.cs    # AssemblyLoadContext: 隔离加载外部 DLL
│   │   │   │   ├── PluginHostService.cs   # IHostedService: 插件生命周期编排
│   │   │   │   └── PluginContext.cs        # IPluginContext/IPluginHost: 服务上下文实现
│   │   │   │
│   │   │   ├── Character/
│   │   │   │   └── CharacterController.cs  # ICharacterController: 状态机 + 动画控制
│   │   │   │
│   │   │   ├── Window/
│   │   │   │   ├── WindowService.cs        # IWindowService: 显隐/穿透/置顶
│   │   │   │   └── TrayIconService.cs      # 系统托盘管理
│   │   │   │
│   │   │   ├── Events/
│   │   │   │   └── EventBus.cs             # IEventBus: 内存发布-订阅
│   │   │   │
│   │   │   └── Diagnostics/
│   │   │       └── PerformanceMonitor.cs   # FPS/CPU/内存采集
│   │   │
│   │   ├── Win32/
│   │   │   ├── NativeMethods.cs            # P/Invoke 声明（partial class）
│   │   │   ├── NativeMethods.WindowStyle.cs
│   │   │   ├── NativeMethods.Layered.cs
│   │   │   ├── NativeMethods.Hook.cs
│   │   │   ├── WindowStyleManager.cs       # 无边框 + 透明 + 置顶封装
│   │   │   └── ClickThroughManager.cs      # 点击穿透切换逻辑
│   │   │
│   │   ├── Live2D/
│   │   │   ├── Bridge/
│   │   │   │   ├── Live2DBridgeNative.cs   # P/Invoke 声明
│   │   │   │   ├── Live2DBridgeWrapper.cs  # 安全封装（异常转换、线程安全）
│   │   │   │   └── NativeStructures.cs     # 与 C++ 共享的 struct
│   │   │   ├── Rendering/
│   │   │   │   ├── Live2DRenderHost.cs     # WPF HwndSource + 渲染循环
│   │   │   │   └── FrameTimer.cs           # 60fps 渲染计时器
│   │   │   └── Animation/
│   │   │       ├── StateAnimationMapper.cs # CharacterState → 动画组映射
│   │   │       └── EyeFollowController.cs  # 眼球追踪（±30°/±15°）
│   │   │
│   │   ├── Views/
│   │   │   ├── MainWindow.xaml             # 主窗口（透明无边框 + Live2D 宿主）
│   │   │   ├── ChatBubbleWindow.xaml       # 聊天气泡（独立透明窗口）
│   │   │   ├── SettingsWindow.xaml         # 设置窗口
│   │   │   └── DebugWindow.xaml            # 调试面板
│   │   │
│   │   ├── ViewModels/
│   │   │   ├── MainViewModel.cs
│   │   │   ├── ChatViewModel.cs
│   │   │   ├── SettingsViewModel.cs
│   │   │   ├── DebugViewModel.cs
│   │   │   ├── RadialMenuViewModel.cs      # ★ 动态渲染已安装插件的菜单项
│   │   │   └── TrayViewModel.cs
│   │   │
│   │   ├── Controls/
│   │   │   ├── Live2DHostControl.cs        # HwndHost: 嵌入 C++ 渲染窗口
│   │   │   └── RadialMenuControl.cs        # ★ 动态径向菜单控件
│   │   │
│   │   ├── Behaviors/
│   │   │   ├── WindowDragBehavior.cs       # 左键拖拽（60fps）
│   │   │   ├── ClickThroughBehavior.cs     # Ctrl+Alt+P 穿透切换
│   │   │   └── AutoHideBehavior.cs         # 鼠标离开 1s 后隐藏
│   │   │
│   │   ├── Converters/
│   │   ├── Themes/
│   │   │   ├── LightTheme.xaml
│   │   │   ├── DarkTheme.xaml
│   │   │   └── ThemeManager.cs
│   │   │
│   │   ├── Assets/
│   │   │   ├── app.ico
│   │   │   ├── tray.ico
│   │   │   ├── menu-icons/                 # 默认菜单图标（插件未提供时使用）
│   │   │   └── live2d/                     # 默认 Live2D 模型
│   │   │
│   │   └── Resources/
│   │       └── Styles/
│   │           ├── WindowStyles.xaml
│   │           └── ButtonStyles.xaml
│   │
│   ├── CoreAIpet.Live2DBridge/             # C++ DLL: 封装 Live2D Cubism Native SDK
│   │   ├── include/
│   │   │   ├── bridge_api.h                # 导出函数 C 接口
│   │   │   └── bridge_types.h              # 共享结构体
│   │   ├── src/
│   │   │   ├── dllmain.cpp
│   │   │   └── bridge_api.cpp              # 导出函数 stub 实现
│   │   ├── CMakeLists.txt
│   │   └── README.md
│   │
│   └── Plugins/                            # ★ 所有插件项目（独立编译）
│       ├── Directory.Build.props           # 插件项目公共构建配置
│       │
│       ├── CoreAIpet.Plugin.Jira/          # Jira 任务管理 + @通知
│       │   ├── plugin.json
│       │   ├── assets/jira.png
│       │   ├── JiraPlugin.cs               # 实现 IPlugin
│       │   ├── JiraApiClient.cs            # Jira REST API 客户端
│       │   └── Models/
│       │
│       ├── CoreAIpet.Plugin.Email.Outlook/ # Outlook 邮箱提供者
│       │   ├── plugin.json
│       │   ├── assets/outlook.png
│       │   ├── OutlookPlugin.cs            # 实现 IPlugin + IEmailProvider
│       │   └── OutlookClient.cs
│       │
│       ├── CoreAIpet.Plugin.Email.Gmail/   # Gmail 邮箱提供者
│       │   ├── plugin.json
│       │   ├── assets/gmail.png
│       │   └── GmailPlugin.cs              # 实现 IPlugin + IEmailProvider
│       │
│       ├── CoreAIpet.Plugin.Email.IMAP/    # 通用 IMAP 邮箱提供者
│       │   ├── plugin.json
│       │   ├── assets/imap.png
│       │   └── ImapPlugin.cs               # 实现 IPlugin + IEmailProvider
│       │
│       ├── CoreAIpet.Plugin.Message.Teams/  # Microsoft Teams
│       │   ├── plugin.json
│       │   ├── assets/teams.png
│       │   └── TeamsPlugin.cs              # 实现 IPlugin + IMessageProvider
│       │
│       ├── CoreAIpet.Plugin.Message.Slack/  # Slack
│       │   ├── plugin.json
│       │   ├── assets/slack.png
│       │   └── SlackPlugin.cs              # 实现 IPlugin + IMessageProvider
│       │
│       ├── CoreAIpet.Plugin.Message.DingTalk/ # 钉钉
│       │   ├── plugin.json
│       │   ├── assets/dingtalk.png
│       │   └── DingTalkPlugin.cs           # 实现 IPlugin + IMessageProvider
│       │
│       ├── CoreAIpet.Plugin.Message.Feishu/ # 飞书
│       │   ├── plugin.json
│       │   ├── assets/feishu.png
│       │   └── FeishuPlugin.cs             # 实现 IPlugin + IMessageProvider
│       │
│       ├── CoreAIpet.Plugin.Message.QQ/     # QQ
│       │   ├── plugin.json
│       │   ├── assets/qq.png
│       │   └── QQPlugin.cs                 # 实现 IPlugin + IMessageProvider
│       │
│       └── CoreAIpet.Plugin.Message.WeChat/ # 企业微信
│           ├── plugin.json
│           ├── assets/wechat.png
│           └── WeChatPlugin.cs             # 实现 IPlugin + IMessageProvider
│
├── plugins/                                # 运行时插件目录（构建产物输出到此）
│   ├── CoreAIpet.Plugin.Jira/
│   │   ├── plugin.json
│   │   ├── CoreAIpet.Plugin.Jira.dll
│   │   └── assets/
│   ├── CoreAIpet.Plugin.Email.Outlook/
│   │   └── ...
│   └── ...
│
├── tests/
│   ├── CoreAIpet.Core.Tests/
│   └── CoreAIpet.Desktop.Tests/
│
└── docs/
    ├── PRD - Windows AI桌面助手 V1.0.md
    └── Architecture.md                     # 本文档
```

---

## 3. 项目职责与依赖

### 3.1 依赖关系

```
                     ┌──────────────────────────────────────────┐
                     │          CoreAIpet.Desktop               │
                     │  (宿主壳: WPF + Services + UI + Live2D)  │
                     └─────────────────┬────────────────────────┘
                                       │
                          ┌────────────┼────────────┐
                          │            │            │
                          ▼            ▼            ▼
                  ┌──────────┐  ┌──────────┐  ┌──────────┐
                  │ Core     │  │ Plugins  │  │ Live2D   │
                  │ (接口)   │  │ (独立DLL) │  │ Bridge   │
                  └──────────┘  └────┬─────┘  │ (C++ DLL)│
                                     │        └──────────┘
                                     │
                              只依赖 Core
                              不依赖 Desktop
```

### 3.2 各项目职责

| 项目 | 类型 | 职责 | NuGet |
|------|------|------|-------|
| **CoreAIpet.Core** | 类库 | 接口 + 模型 + 枚举 + 事件，零实现 | `Microsoft.Extensions.Hosting.Abstractions` |
| **CoreAIpet.Desktop** | WPF Exe | 宿主壳：窗口管理、AI服务、Live2D渲染、插件加载、UI | `CommunityToolkit.Mvvm`, `Microsoft.Extensions.Hosting`, `Serilog`, `OpenAI`, `Azure.AI.OpenAI` |
| **CoreAIpet.Live2DBridge** | C++ DLL | 封装 Live2D Cubism Native SDK，提供 C 导出接口 | 无 |
| **CoreAIpet.Plugin.*** | 类库(DLL) | 独立插件，实现 IPlugin + 业务逻辑 | 按需（如 `MailKit` for IMAP） |

---

## 4. 插件架构

### 4.1 核心理念

**Desktop 只做壳，功能全在插件中。**

```
┌─────────────────────────────────────────────────────────────┐
│                  CoreAIpet.Desktop (宿主壳)                   │
│                                                              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────────┐   │
│  │ 窗口管理  │  │ AI 对话  │  │ Live2D   │  │ PluginMgr  │   │
│  │ 拖拽/穿透 │  │ 流式响应  │  │ 渲染引擎  │  │ 加载/卸载  │   │
│  └──────────┘  └──────────┘  └──────────┘  └─────┬──────┘   │
│                                                   │          │
│  径向菜单 ←── 动态渲染 ──── 来自已安装的插件 ──────┘          │
└─────────────────────────────────────────────────────────────┘
                         │
                         │ 加载 plugins/ 目录
                         ▼
        ┌────────────────┼────────────────────────┐
        ▼                ▼                        ▼
  ┌───────────┐   ┌───────────┐           ┌───────────┐
  │ Jira 插件  │   │ Outlook   │           │ Slack     │
  │ IPlugin   │   │ IEmail    │           │ IMessage  │
  └───────────┘   └───────────┘           └───────────┘
```

### 4.2 插件分类

| 分类 | 接口 | 示例 | 菜单入口 |
|------|------|------|---------|
| **功能插件** | `IPlugin` | Jira | 自定义菜单项 |
| **邮箱提供者** | `IPlugin` + `IEmailProvider` | Outlook, Gmail, IMAP | "Email" 聚合入口 |
| **消息提供者** | `IPlugin` + `IMessageProvider` | Slack, 钉钉, 飞书, Teams, QQ, 企业微信 | "Message" 聚合入口 |

### 4.3 插件生命周期

```
                    PluginManager 启动
                         │
                         ▼
        ┌─ 扫描 plugins/ 目录 ─────────────────────┐
        │  读取每个子目录的 plugin.json              │
        │  通过 AssemblyLoadContext 加载 DLL         │
        │  找到 plugin.json 中 className 指定的类     │
        └──────────────────────────────────────────┘
                         │
                         ▼ 对每个插件:
        ┌──────────────────────────────────────────┐
        │  Load(IPluginContext)                     │
        │    → 获取配置/日志/事件总线/AI服务          │
        │    → 注册菜单项 (RegisterMenuItem)         │
        │                                           │
        │  Activate()                               │
        │    → 连接外部服务                          │
        │    → 启动后台轮询/WebSocket                │
        │                                           │
        │  Execute(ct)                              │
        │    → 核心业务逻辑                          │
        │    → 周期性任务                            │
        │                                           │
        │  [应用关闭时]                              │
        │  Deactivate() → 暂停                      │
        │  Unload() → 释放资源                      │
        └──────────────────────────────────────────┘
```

### 4.4 径向菜单动态渲染

```
原设计 (6个固定入口):        改进后 (动态渲染):

      Task                      [插件A]  [插件B]
                           [插件C]          [插件D]
  Jira        Email
        [角色]                 [插件E]  [插件F]
                           [插件G]
 Message      Setting
                            ↑ 根据已安装插件自动生成
      Debug                   支持分组 + 折叠 + 更多页面
```

`RadialMenuViewModel` 从 `PluginManager` 获取所有已注册菜单项，动态计算位置。
超过一页时支持翻页或分组显示。

### 4.5 插件目录布局 (运行时)

```
plugins/
└── CoreAIpet.Plugin.Message.Slack/
    ├── plugin.json                 # 清单文件
    ├── CoreAIpet.Plugin.Message.Slack.dll
    ├── CoreAIpet.Core.dll          # 共享引用 (由 PluginLoadContext 处理)
    └── assets/
        └── slack.png
```

**plugin.json 格式:**

```json
{
  "id": "provider.message.slack",
  "name": "Slack",
  "version": "1.0.0",
  "author": "CoreAIpet",
  "entryPoint": "CoreAIpet.Plugin.Message.Slack.dll",
  "className": "CoreAIpet.Plugin.Message.Slack.SlackPlugin",
  "icon": "assets/slack.png",
  "description": "Slack 消息提供者"
}
```

| 字段 | 说明 |
|------|------|
| `id` | 唯一标识，建议格式: `provider.{category}.{name}` 或 `coreai.{name}` |
| `entryPoint` | 插件 DLL 文件名 |
| `className` | 实现 `IPlugin` 的完整类名（含命名空间） |
| `icon` | 图标路径，相对于插件目录 |

---

## 5. 核心接口

### 5.1 IPlugin — 插件生命周期

```csharp
public interface IPlugin : IDisposable
{
    string Id { get; }                // "provider.message.slack"
    string Name { get; }              // "Slack"
    string Version { get; }           // "1.0.0"
    PluginState State { get; }

    Task LoadAsync(IPluginContext context);     // 初始化 + 注册菜单项
    Task ActivateAsync();                       // 连接外部服务
    Task ExecuteAsync(CancellationToken ct);    // 核心逻辑
    Task DeactivateAsync();                     // 暂停
    Task UnloadAsync();                         // 释放

    IReadOnlyList<PluginMenuItem> GetMenuItems();
    Task HandleMenuActionAsync(string actionId);
}
```

### 5.2 IMessageProvider — 消息源

```csharp
public interface IMessageProvider : IDisposable
{
    string Id { get; }                 // "provider.message.slack"
    string Name { get; }               // "Slack"
    string Icon { get; }
    ProviderConnectionState ConnectionState { get; }

    Task<int> GetUnreadCountAsync(CancellationToken ct);
    Task<IReadOnlyList<MessageItem>> GetRecentMessagesAsync(int count, CancellationToken ct);
    Task ConnectAsync(CancellationToken ct);
    Task DisconnectAsync(CancellationToken ct);

    event EventHandler<MessageItem>? MessageReceived;
    event EventHandler<ProviderConnectionState>? ConnectionStateChanged;
}
```

### 5.3 IEmailProvider — 邮箱源

```csharp
public interface IEmailProvider : IDisposable
{
    string Id { get; }                 // "provider.email.outlook"
    string Name { get; }               // "Outlook"
    string Icon { get; }
    ProviderConnectionState ConnectionState { get; }

    Task<int> GetUnreadCountAsync(CancellationToken ct);
    Task<IReadOnlyList<EmailItem>> GetRecentEmailsAsync(int count, CancellationToken ct);
    Task ConnectAsync(CancellationToken ct);
    Task DisconnectAsync(CancellationToken ct);

    event EventHandler<EmailItem>? EmailReceived;
    event EventHandler<ProviderConnectionState>? ConnectionStateChanged;
}
```

### 5.4 IPluginHost — 宿主服务

```csharp
public interface IPluginHost
{
    void RegisterMenuItem(PluginMenuItem menuItem);
    void UnregisterMenuItem(string menuItemId);
    IConfigService GetConfigService();
    ILogService GetLogService();
    IEventBus GetEventBus();
    IAIServiceProvider GetAIService();
    string GetPluginDataDirectory(string pluginId);
}
```

### 5.5 IAIService / IAIServiceProvider — AI 策略模式

```csharp
public interface IAIService
{
    AIProvider Provider { get; }        // OpenAI / AzureOpenAI / Ollama
    bool IsConfigured { get; }
    Task<ChatResponse> SendMessageAsync(ChatRequest request, CancellationToken ct);
    IAsyncEnumerable<string> SendMessageStreamAsync(ChatRequest request, CancellationToken ct);
    Task<bool> TestConnectionAsync(CancellationToken ct);
}

public interface IAIServiceProvider
{
    IAIService Current { get; }
    AIProvider ActiveProvider { get; }
    void SwitchProvider(AIProvider provider);
    IReadOnlyList<IAIService> GetAllProviders();
}
```

### 5.6 ICharacterController — 角色状态机

```csharp
public interface ICharacterController
{
    CharacterState CurrentState { get; }    // Idle / Happy / Thinking / Talking
    void SetState(CharacterState newState);
    void SetState(CharacterState newState, TimeSpan minDuration);
    void UpdateEyeTracking(double mouseX, double mouseY);
    void PlayAnimation(string group, string name);
    event EventHandler<CharacterStateChangedEventArgs>? StateChanged;
}
```

状态转移规则:
```
Idle ──hover──► Happy ──leave──► Idle
Idle ──send──► Thinking ──response──► Talking ──3s──► Idle
Thinking ──new msg──► Thinking (自循环)
Talking ──new msg──► Thinking (打断)
```

---

## 6. 数据流

### 6.1 发送聊天消息

```
用户点击角色 / Alt+Space
    │
    ▼
MainViewModel.OpenChatCommand
    │
    ▼
ChatViewModel.ShowBubble()              ← 聊天气泡淡入
    │
    ▼ ChatViewModel.SendCommand
    ├──► ICharacterController.SetState(Thinking)    ← Live2D 切换 thinking 动画
    ├──► IAIServiceProvider.SendMessageAsync(req)
    │       └──► OpenAI / Azure / Ollama (HTTP → API)
    ├──► ICharacterController.SetState(Talking)     ← 收到首个 token
    ├──► ChatViewModel.AppendToken(token)           ← 逐字显示
    ├──► ICharacterController.SetState(Idle)        ← 完成 3s 后
    └──► IEventBus.Publish(ChatMessageEvent)        ← 插件可订阅
```

### 6.2 插件加载流程

```
应用启动
    │
    ▼
PluginHostService.StartAsync()
    │
    ├──► 扫描 plugins/ 目录
    │       对每个子目录:
    │       ├── 读取 plugin.json
    │       ├── 创建 PluginLoadContext (隔离加载)
    │       ├── 加载 entryPoint DLL
    │       └── 通过反射创建 className 实例 (IPlugin)
    │
    ├──► 对每个插件调用 LoadAsync(context)
    │       插件内部:
    │       ├── 读取配置
    │       ├── 注册菜单项 → PluginHost.RegisterMenuItem()
    │       └── 初始化内部资源
    │
    ├──► 对每个插件调用 ActivateAsync()
    │       插件内部:
    │       ├── 连接外部服务
    │       └── 启动后台任务
    │
    └──► PluginManager 通知 RadialMenuViewModel
            └── 重新渲染径向菜单 (包含新注册的菜单项)
```

### 6.3 消息聚合流

```
SlackPlugin (IMessageProvider)
    │ MessageReceived 事件
    ▼
PluginManager
    │ 收集所有 IMessageProvider 的消息
    ▼
MessageViewModel (聚合展示)
    ├── 未读总数 = Slack(5) + 钉钉(3) + 飞书(1) + ...
    ├── 最新消息列表 (按时间排序, 混合所有源)
    └── 点击某条 → 打开 DeepLink → 跳转到源应用
```

---

## 7. Win32 互操作

| 功能 | Win32 API | 封装类 |
|------|-----------|--------|
| 无边框窗口 | `SetWindowLong(GWL_STYLE, WS_POPUP)` | `WindowStyleManager` |
| 桌面置顶 | `SetWindowPos(HWND_TOPMOST)` | `WindowStyleManager` |
| 透明背景 | `SetLayeredWindowAttributes` + `DwmExtendFrameIntoClientArea` | `WindowStyleManager` |
| 点击穿透 | `SetWindowLong(GWL_EXSTYLE, WS_EX_TRANSPARENT)` | `ClickThroughManager` |
| 全局热键 | `RegisterHotKey` / `UnregisterHotKey` | `HotkeyManager` |
| 鼠标钩子 | `SetWindowsHookEx(WH_MOUSE_LL)` | `NativeMethods.Hook` |
| 系统托盘 | `Shell_NotifyIcon` | `TrayIconService` |
| 光标位置 | `GetCursorPos` | `NativeMethods` |

---

## 8. Live2D 渲染架构

```
C# WPF                                  C++ DLL
┌──────────────────────────┐            ┌──────────────────────────┐
│ Live2DHostControl        │            │ Live2DBridge.dll         │
│ (HwndHost)               │            │                          │
│   │                      │            │  ┌────────────────────┐  │
│   ▼ Bridge_Render() ──── │── P/Invoke ─│► │ Cubism Renderer    │  │
│   │                      │            │  │ (Direct3D 11)      │  │
│   ▼ Bridge_SetEyeTarget()│── P/Invoke ─│► │ Eye Tracking       │  │
│   │                      │            │  │                    │  │
│   ▼ Bridge_SetMotion()   │── P/Invoke ─│► │ Animation Manager  │  │
│                          │            │  └────────┬───────────┘  │
│ HwndSource (子窗口 HWND) │◄── 渲染 ────│           │              │
└──────────────────────────┘            │  Live2D Cubism SDK      │
                                        └──────────────────────────┘
```

每帧由 `CompositionTarget.Rendering` (60fps) 触发 C# 调用 `Bridge_Render()`。

---

## 9. 配置 Schema

存储路径: `%APPDATA%\CoreAIpet\config.json`

```json
{
  "appearance": {
    "scale": 1.0,
    "opacity": 1.0,
    "theme": "dark"
  },
  "system": {
    "autoStart": false,
    "alwaysOnTop": true,
    "clickThrough": false
  },
  "ai": {
    "activeProvider": "openai",
    "openai": { "endpoint": "https://api.openai.com/v1", "apiKey": "", "model": "gpt-4" },
    "azureOpenAI": { "endpoint": "", "apiKey": "", "deploymentName": "", "model": "" },
    "ollama": { "endpoint": "http://localhost:11434", "model": "llama3" }
  },
  "position": { "x": 1600, "y": 820 },
  "plugins": {
    "provider.message.slack": { "enabled": true, "token": "", "workspace": "" },
    "provider.message.dingtalk": { "enabled": true, "appKey": "", "appSecret": "" },
    "provider.message.feishu": { "enabled": true, "appId": "", "appSecret": "" },
    "provider.email.outlook": { "enabled": true, "clientId": "", "tenantId": "" },
    "provider.email.gmail": { "enabled": true, "clientId": "", "clientSecret": "" },
    "coreai.jira": { "enabled": true, "baseUrl": "", "username": "", "apiToken": "" }
  },
  "debug": { "logLevel": "Information", "showFps": false }
}
```

**关键设计**: 插件配置以插件 `id` 为 key，不预定义结构。插件在 `LoadAsync` 中自行读取和解析。

---

## 10. 构建顺序

```
1. CoreAIpet.Live2DBridge.dll       (C++ DLL, 无 .NET 依赖)
2. CoreAIpet.Core.dll               (类库, 无实现依赖)
3. CoreAIpet.Desktop.exe            (WPF 宿主, 依赖 1+2)
4. CoreAIpet.Plugin.*.dll           (各插件, 依赖 2, 输出到 plugins/)
```

---

## 11. 扩展指南: 添加新 Provider

以「添加 Telegram 消息提供者」为例:

```
步骤 1: 在 src/Plugins/ 下创建目录
        src/Plugins/CoreAIpet.Plugin.Message.Telegram/
        ├── plugin.json
        ├── assets/telegram.png
        ├── TelegramPlugin.cs
        └── TelegramClient.cs

步骤 2: plugin.json 声明
        {
          "id": "provider.message.telegram",
          "className": "CoreAIpet.Plugin.Message.Telegram.TelegramPlugin",
          ...
        }

步骤 3: TelegramPlugin.cs 实现
        - 继承 IPlugin (生命周期)
        - 实现 IMessageProvider (消息能力)
        - LoadAsync 中注册菜单项
        - ActivateAsync 中连接 Telegram Bot API

步骤 4: csproj 引用 Core + Telegram.Bot NuGet

步骤 5: 编译 → DLL 自动输出到 plugins/ 目录

步骤 6: 重启 Desktop → 自动发现并加载 → 径向菜单自动出现 Telegram 入口
```

**无需修改 Desktop 任何代码。**

---

## 12. 验证方案

| 验证项 | 方法 |
|--------|------|
| 插件发现 | 在 plugins/ 放入一个插件 DLL，重启应用，确认自动加载 |
| 插件卸载 | 删除 plugins/ 中某插件目录，重启应用，确认不再加载 |
| 动态菜单 | 安装/卸载不同插件，确认径向菜单自动增减入口 |
| 消息聚合 | 同时启用 Slack + 钉钉插件，确认未读数合并显示 |
| 邮箱聚合 | 同时启用 Outlook + Gmail，确认未读邮件合并显示 |
| AI 对话 | 切换 OpenAI/Ollama，确认流式回复正常 |
| 角色状态 | 发消息验证 Idle→Thinking→Talking→Idle 完整流转 |
| 拖动 + 位置 | 拖动角色，重启，确认位置恢复 |
| 点击穿透 | Ctrl+Alt+P，确认角色可见但不阻挡点击 |
| 性能 | 启动<5s, 内存<300MB, CPU<5%, 60fps |

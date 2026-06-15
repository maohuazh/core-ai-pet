## Why

构建一个 Windows 桌面 AI 虚拟助手 (CoreAIpet V1.0 MVP)，以 Live2D 角色形式常驻桌面，提供 AI 对话、办公系统集成、可扩展插件框架。当前项目仅有空壳结构和接口定义，需要实现全部业务逻辑以达到 MVP 验收标准 (AC-001 ~ AC-012)。

## What Changes

- 实现 Live2D 角色渲染引擎（C++ Bridge + C# P/Invoke），支持 4 种状态动画和眼球追踪
- 实现透明无边框 WPF 主窗口，支持拖动、位置保存/恢复、点击穿透
- 实现 Hover 径向快捷菜单，动态渲染已安装插件的菜单项
- 实现 AI 聊天系统，支持 OpenAI / Azure OpenAI / Ollama 三种后端，流式回复
- 实现系统托盘驻留，右键菜单 + 双击恢复
- 实现插件框架，通过 AssemblyLoadContext 动态加载外部 DLL 插件
- 实现配置系统 (config.json)，支持外观/系统/AI/插件配置
- 实现 Debug 面板，展示 FPS/CPU/内存/插件状态/日志
- 提供邮件和消息 Provider 插件的项目骨架 (Outlook/Gmail/IMAP + Slack/钉钉/飞书/Teams/QQ/企业微信)

## Capabilities

### New Capabilities

- `live2d-character`: Live2D 角色展示、多状态动画 (Idle/Happy/Thinking/Talking)、眼球追踪、待机动画循环
- `ai-chat`: AI 聊天系统，支持多后端路由 (OpenAI/Azure/Ollama)、流式响应、对话上下文管理、角色状态联动
- `desktop-interaction`: 透明无边框窗口、拖动 (60fps)、位置保存/恢复、点击穿透 (Ctrl+Alt+P)、全局热键
- `hover-menu`: Hover 径向快捷菜单，动态渲染插件菜单项、Fade/Scale 动画、自动隐藏
- `system-tray`: 系统托盘图标、右键菜单 (显示/隐藏/设置/重启/退出)、双击恢复
- `plugin-framework`: 插件发现/加载/卸载、AssemblyLoadContext 隔离、生命周期管理 (Load/Activate/Execute/Unload)、动态菜单注册
- `settings`: 设置中心，外观 (缩放/透明度/主题)、系统 (开机启动/置顶/穿透)、AI 后端配置
- `debug-panel`: 调试面板，FPS/CPU/内存监控、插件状态、日志查看
- `email-provider`: 邮箱提供者插件接口 (IEmailProvider) + Outlook/Gmail/IMAP 插件骨架
- `message-provider`: 消息提供者插件接口 (IMessageProvider) + Slack/钉钉/飞书/Teams/QQ/企业微信插件骨架
- `live2d-bridge`: C++ DLL 桥接 Live2D Cubism Native SDK，D3D11 渲染、动画管理、眼球追踪参数计算

### Modified Capabilities

(无，这是全新项目)

## Impact

- **代码**: src/CoreAIpet.Core (接口已完成，需补充 Base 类), src/CoreAIpet.Desktop (全部 Services/Views/ViewModels 待实现), src/CoreAIpet.Live2DBridge (stub 需完善)
- **依赖**: CommunityToolkit.Mvvm, Microsoft.Extensions.Hosting, Serilog, OpenAI SDK, Azure.AI.OpenAI SDK, Live2D Cubism Native SDK (C++)
- **系统**: Windows 10/11, Win32 API (SetWindowLong, SetLayeredWindowAttributes, RegisterHotKey, Shell_NotifyIcon), Direct3D 11
- **运行时**: plugins/ 目录存放外部插件 DLL, %APPDATA%\CoreAIpet\config.json 存储配置

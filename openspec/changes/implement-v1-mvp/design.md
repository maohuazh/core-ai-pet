## Context

CoreAIpet 是一个 Windows 桌面 AI 虚拟助手项目，技术栈为 .NET 8 + WPF + Live2D Cubism Native SDK。

**当前状态**: 项目骨架已搭建完成：
- `CoreAIpet.Core` 类库：14 个核心接口 + 数据模型 + 事件定义，已编译通过
- `CoreAIpet.Desktop` WPF 项目：App.xaml 入口 + CompositionRoot DI 组合根 + MainWindow 骨架，已编译通过
- `CoreAIpet.Live2DBridge` C++ 项目：导出接口头文件 + stub 实现，待 Live2D SDK 集成
- `src/Plugins/` 10 个插件项目：已创建 plugin.json 清单 + 目录结构，待实现

**约束**:
- 目标平台: Windows 10/11
- 性能要求: 启动 <5s, 内存 <300MB, CPU 待机 <5%, 动画 60fps
- Live2D SDK: 需通过 C++ DLL 桥接 (P/Invoke)
- 插件隔离: 每个 Provider 独立 DLL，通过 AssemblyLoadContext 加载

## Goals / Non-Goals

**Goals:**
- 实现 MVP 全部 12 项验收标准 (AC-001 ~ AC-012)
- 实现完整的插件生命周期管理 (扫描/加载/激活/执行/卸载)
- 实现 AI 聊天流式响应 + 角色状态联动
- 实现动态径向菜单（根据已安装插件自动生成）
- 提供 10 个 Provider 插件的可编译骨架（含 plugin.json + 入口类）

**Non-Goals:**
- 不实现实际的第三方 API 对接（Slack/钉钉/飞书等），只提供插件骨架和接口
- 不实现 Live2D 模型的实际渲染（C++ Bridge 只提供接口框架，需要 SDK 后完善）
- 不实现安装包/自动更新机制
- 不实现多显示器支持
- 不实现插件市场/在线安装

## Decisions

### D1: 宿主极简 + 全插件化

**决策**: Desktop 项目只做窗口管理、AI 服务、Live2D 渲染、插件加载。所有业务功能（邮件/消息/任务）通过独立插件实现。

**理由**: 
- 满足扩展性需求（新增消息源/邮箱无需改宿主代码）
- 插件可独立开发/测试/部署
- 符合 PRD 的插件架构要求 (FR-501)

**备选方案**: 将 Jira/Email/Message 作为 Desktop 内置模块 → 否决，因为无法独立扩展

### D2: Provider 聚合模式

**决策**: 定义 `IEmailProvider` 和 `IMessageProvider` 接口在 Core 层，每个具体 Provider（Outlook/Gmail/Slack/钉钉...）是独立插件，实现 `IPlugin` + Provider 接口。Desktop 中的 UI 聚合展示所有 Provider 的数据。

**理由**:
- 同一类功能支持多个源（如同时启用 Outlook + Gmail）
- 未读数可合并计算
- 新增 Provider 只需添加插件，不改宿主

### D3: MVVM + Generic Host + CommunityToolkit.Mvvm

**决策**: 使用 `Microsoft.Extensions.Hosting` 作为 DI 容器和应用生命周期管理，使用 `CommunityToolkit.Mvvm` 的源生成器 (`[ObservableProperty]`, `[RelayCommand]`) 简化 ViewModel。

**理由**:
- Generic Host 提供 IHostedService 管理插件生命周期
- CommunityToolkit.Mvvm 减少 MVVM 样板代码 80%+
- .NET 8 原生支持，无额外运行时开销

### D4: Live2D 通过 C++ DLL 桥接

**决策**: Live2D Cubism Native SDK 封装在独立 C++ DLL 中，通过 P/Invoke 暴露 C 接口。WPF 使用 HwndHost 创建子窗口给 C++ 渲染，D3D11 渲染到该 HWND。

**理由**:
- Live2D 官方只提供 C++ Native SDK，无 .NET 绑定
- 隔离 Native SDK 依赖，C# 侧只依赖导出函数
- D3D11 渲染性能可满足 60fps 需求

**备选方案**: WebView2 + JS SDK → 否决，性能开销大，内存占用高

### D5: 配置使用 JSON 文件 + 强类型模型

**决策**: 配置存储在 `%APPDATA%\CoreAIpet\config.json`，使用 `System.Text.Json` 序列化。Core 层定义强类型配置模型 (`AppSettings`, `AISettings` 等)。插件配置以插件 ID 为 key，结构由插件自行定义。

**理由**:
- 简单直接，无需引入数据库
- 强类型模型提供编译时检查
- 插件配置灵活扩展

### D6: 插件通过 AssemblyLoadContext 隔离加载

**决策**: 外部插件 DLL 通过自定义 `AssemblyLoadContext(isCollectible: true)` 加载，实现依赖隔离。插件通过 `plugin.json` 清单声明入口点和元数据。

**理由**:
- 插件可携带自己的依赖版本，不与宿主冲突
- `isCollectible: true` 支持卸载时回收
- plugin.json 提供标准化的插件发现机制

## Risks / Trade-offs

- **[Live2D SDK 许可]** → Live2D Cubism SDK 需要商业授权。MVP 阶段先用开源/免费模型验证流程，正式上线前确认许可证。
- **[C++ Bridge 复杂度]** → P/Invoke 桥接涉及内存管理、线程安全、HWND 共享。→ 缓解：封装层 (Live2DBridgeWrapper) 统一处理异常转换和资源释放。
- **[多 Provider 并发]** → 多个消息/邮箱 Provider 同时轮询可能造成网络压力。→ 缓解：错峰轮询 + 指数退避 + 可配置轮询间隔。
- **[点击穿透与交互冲突]** → 穿透模式下无法接收鼠标事件。→ 缓解：仅对角色像素区域做 hit-test，非角色区域穿透。
- **[插件 DLL 版本冲突]** → 不同插件可能依赖不同版本的同一 NuGet 包。→ 缓解：AssemblyLoadContext 隔离 + Core 接口层保持最小依赖。
- **[WPF 渲染性能]** → 透明窗口 + 动画可能对 GPU 有额外负担。→ 缓解：使用 D3D11 硬件加速渲染 Live2D，WPF 层只做菜单/气泡等轻量 UI。

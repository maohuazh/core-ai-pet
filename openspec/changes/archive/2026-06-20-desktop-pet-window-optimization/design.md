## Context

当前应用使用 Tauri 2 的 transparent + decorations:false 配置实现透明无边框窗口，窗口尺寸 200x200，内部 PixiJS 渲染区域 240x240。窗口已经可以置顶显示、跳过任务栏、透明背景。

但存在以下体验问题：
1. 整个 200x200 矩形区域都会拦截鼠标事件，透明区域无法点击下层窗口
2. 窗口点击时会抢夺系统焦点
3. 窗口存在系统默认阴影
4. 缺少系统托盘入口

参考设计方案（docs/CoreAIpet 真正桌宠级窗口设计方案.md）中的 P0-P2 优先级。

## Goals / Non-Goals

**Goals:**
- 实现透明区域的鼠标点击穿透（P1）
- 实现智能穿透模式：悬停角色区域时自动取消穿透（P1）
- 窗口不抢夺系统焦点（P2）
- 去除窗口阴影（P0）
- 添加系统托盘图标和右键菜单（P2）

**Non-Goals:**
- 多窗口架构（Pet Window + Workspace Window）— 后续独立变更
- 边缘吸附、自动避让窗口 — 后续独立变更
- 多显示器支持、DPI自适应 — 后续独立变更
- Linux/macOS 平台支持 — 当前仅 Windows

## Decisions

### 1. 点击穿透：Windows 原生窗口样式 vs 前端 alpha 检测

**选择：Windows 原生窗口样式（WS_EX_LAYERED + WS_EX_TRANSPARENT）**

通过 Rust 后端调用 Win32 API 设置窗口扩展样式，配合前端动态切换。

**替代方案：** 纯前端方案监听 canvas 像素 alpha 值决定是否穿透 — 性能差、实现复杂、跨平台不一致。

**理由：** 原生方案性能好，系统级穿透，与桌宠产品标准做法一致。

### 2. 智能穿透：前端角色区域检测 vs 全局穿透

**选择：前端检测鼠标是否在模型渲染区域内，通过 Tauri command 切换穿透状态**

鼠标 hover 到 pet-container 时前端发 Tauri command 关闭穿透，mouseleave 时恢复穿透。因为整个 pet-container 就是角色区域（200x200窗口），实际上只需在 mouseenter/mouseleave 时切换即可。

**替代方案：** 每帧检测鼠标位置下像素的 alpha 值 — 过于复杂。

**理由：** pet-container 覆盖整个窗口，mouseenter 即表示鼠标在角色区域，简单可靠。

### 3. 不抢焦点：WS_EX_NOACTIVATE

**选择：设置 WS_EX_NOACTIVATE 扩展样式**

窗口可以接收鼠标点击但不会激活/获取焦点。

**理由：** 标准 Windows 做法，Tauri 本身不直接提供此 API。

### 4. 系统托盘：Tauri TrayIcon API

**选择：使用 Tauri 2 内置的 `tray` 模块**

在 Rust 端创建 TrayIcon，绑定右键菜单。

**理由：** Tauri 官方支持，无需额外依赖。

### 5. 去阴影：Tauri Window API

**选择：Rust 端调用 `window.set_shadow(false)`**

tauri.conf.json 中没有 shadow 配置项，需通过 Window API 在运行时设置。

## Risks / Trade-offs

- **[风险] WS_EX_TRANSPARENT 与 alwaysOnTop 交互** → 测试确认穿透后窗口仍能正确置顶显示
- **[风险] 智能穿透切换延迟** → mouseenter/mouseleave 事件响应快，切换应无明显延迟；若出现闪烁，可加 debounce
- **[风险] WS_EX_NOACTIVATE 导致悬浮菜单无法点击** → 需测试确认；如果菜单按钮无法响应点击，可能需要仅在非交互模式下设置 NOACTIVATE
- **[权衡] 仅支持 Windows** → 当前产品定位就是 Windows 桌宠，可接受

## Context

当前 Tauri 2 应用使用默认系统托盘图标，缺少品牌 logo 展示。托盘菜单的"显示窗口"和"退出"功能绑定失效，用户无法通过托盘菜单正常操作应用。

**技术栈:**
- Tauri 2 (Rust 后端)
- Vue 3 (前端)
- 系统托盘通过 Tauri 的 `tray` 模块实现

**当前状态:**
- 托盘图标使用 Tauri 默认图标
- 托盘菜单事件处理逻辑可能存在绑定错误或未正确实现
- logo 图片资源尚未集成到项目中

## Goals / Non-Goals

**Goals:**
- 将 AI 宠物 logo 集成到项目资源中
- 托盘图标使用自定义 logo（支持多分辨率）
- 应用最小化到托盘时显示 logo
- 修复托盘"显示窗口"功能，点击后恢复宠物窗口
- 修复托盘"退出"功能，点击后完全退出应用

**Non-Goals:**
- 不实现托盘图标的动画效果
- 不修改托盘菜单的 UI 样式（保持系统原生风格）
- 不添加托盘图标的拖拽功能
- 不实现多主题 logo 切换

## Decisions

### 1. Logo 资源格式和位置

**Decision:** 使用 PNG 格式，存放在 `src-tauri/icons/` 目录

**Rationale:**
- Tauri 的托盘图标配置直接引用 `icons/` 目录下的文件
- PNG 支持透明度，适合不规则形状的 logo
- 需要提供多种尺寸（16x16, 32x32, 48x48, 128x128, 256x256）以适配不同 DPI

**Alternatives considered:**
- ICO 格式：Windows 原生支持多分辨率，但 PNG 更通用且 Tauri 支持良好
- SVG 格式：矢量可缩放，但 Tauri 托盘图标不支持 SVG

### 2. 托盘图标配置方式

**Decision:** 在 Rust 代码中动态创建托盘，而非使用 `tauri.conf.json` 静态配置

**Rationale:**
- 动态创建允许在运行时切换图标（如最小化时显示 logo，正常时显示其他状态）
- 更灵活地处理菜单事件绑定
- 符合 Tauri 2 推荐的托盘使用方式

**Alternatives considered:**
- 静态配置：简单但灵活性差，无法动态切换图标

### 3. 托盘菜单事件处理

**Decision:** 使用 Tauri 的 `tray_menu` 事件系统，通过 `on_tray_menu_event` 处理菜单点击

**Rationale:**
- Tauri 2 标准做法，与窗口事件系统集成良好
- 可以直接访问 `AppHandle` 来操作窗口和执行退出

**实现要点:**
- "显示窗口"：获取主窗口引用，调用 `show()` 和 `set_focus()`
- "退出"：调用 `app.exit(0)`

## Risks / Trade-offs

### [Risk] Logo 尺寸适配
→ **Mitigation:** 提供多种分辨率的 PNG 文件，Tauri 会根据系统 DPI 自动选择最合适的尺寸

### [Risk] 托盘菜单事件未正确绑定
→ **Mitigation:** 在应用启动时验证事件绑定，添加日志输出以便调试

### [Risk] 窗口隐藏后无法恢复
→ **Mitigation:** 确保窗口使用 `hide()` 而非 `close()`，恢复时使用 `show()` + `set_focus()`

### [Trade-off] 动态托盘 vs 静态配置
→ 动态托盘增加代码复杂度，但提供更大的灵活性。当前需求明确需要动态切换图标，因此选择动态方式。

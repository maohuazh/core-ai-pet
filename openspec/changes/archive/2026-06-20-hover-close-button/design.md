## Context

桌面宠物窗口无边框、无装饰、跳过任务栏，目前没有退出方式。现有悬停交互模式：鼠标进入 `pet-container` 时显示悬浮菜单，离开时 200ms 延迟隐藏。需要在同一悬停周期内显示一个关闭按钮。

现有权限 `core:window:allow-close` 已配置，前端可直接使用 Tauri window API。

## Goals / Non-Goals

**Goals:**
- 提供唯一的程序退出入口（右上角 × 按钮）
- 按钮与悬浮菜单同步显示/隐藏
- 按钮视觉透明、不干扰模型观赏

**Non-Goals:**
- 不实现确认对话框（直接退出）
- 不实现键盘快捷键退出
- 不修改现有悬浮菜单的 6 个按钮

## Decisions

### 1. 使用独立组件而非嵌入 PetHoverMenu
**选择**: 新建 `WindowCloseButton.vue`，与 `PetHoverMenu.vue` 平级放在 `App.vue` 中。
**理由**: 关闭按钮位于右上角，与环形菜单的布局和语义完全独立。hover-menu-layout spec 明确"SHALL NOT 包含关闭按钮"，新组件避免与现有 spec 冲突。
**替代方案**: 在 PetHoverMenu 中添加 — 违反现有 spec 且布局耦合。

### 2. 使用 Tauri `getCurrentWindow().close()` 直接关闭
**选择**: 前端直接调用 `@tauri-apps/api/window` 的 close 方法。
**理由**: 权限已配置，无需新增 Rust 命令。简单直接。
**替代方案**: 新增 `close_app` Tauri command — 过度工程化，增加不必要的后端代码。

### 3. 样式：半透明白色 × 图标，hover 时高亮
**选择**: 默认 `opacity: 0.3`，hover 时 `opacity: 1` + 红色背景。
**理由**: 透明不干扰观赏，hover 高亮提供明确的可交互反馈。红色暗示关闭/退出语义。

## Risks / Trade-offs

- **[误触退出]** → 按钮默认低透明度 + 仅在悬停时出现，降低误触概率。不做确认弹窗以保持简洁。
- **[按钮遮挡模型]** → 定位在右上角固定位置（top: 8px, right: 8px），尺寸小（28x28px），遮挡面积最小。

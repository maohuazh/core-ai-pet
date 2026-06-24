## Why

当前窗口虽然已配置 `decorations: false` 和 `transparent: true`，但在 Windows 上仍可能看到窗口边缘的细微边框或背景色。需要确保窗口完全透明，只渲染 Live2D 宠物模型本身，实现"宠物直接站在桌面上"的效果。

## What Changes

- 在 Tauri Rust 后端显式设置窗口背景色为完全透明（`set_background_color`）
- 确保 HTML/CSS 层面所有容器和 canvas 元素无可见边框
- 验证 PixiJS canvas 的 WebGL 渲染不会产生边缘伪影

## Capabilities

### New Capabilities

_(无新增能力)_

### Modified Capabilities

- `transparent-window`: 增加完全透明背景的强制保障，确保窗口在所有平台上仅显示模型像素，无任何边框或底色

## Impact

- `src-tauri/src/main.rs`：添加窗口背景色透明设置
- `src/App.vue`：确保 CSS 完全透明
- `src/components/Live2DCanvas.vue`：确保 canvas 无额外样式
- `index.html`：确保全局样式透明

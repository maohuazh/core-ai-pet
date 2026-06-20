## Why

桌面宠物目前没有退出入口。窗口无边框、无任务栏图标、无系统菜单，用户无法关闭程序。需要在鼠标悬停时提供一个关闭按钮作为唯一的退出方式。

## What Changes

- 在显示区域右上角添加一个透明的 × 按钮
- 按钮仅在鼠标悬停在模型上时显示（与悬浮菜单同时出现/消失）
- 点击按钮退出桌面宠物程序
- 使用 Tauri `@tauri-apps/api/window` 的 `close()` 方法关闭窗口

## Capabilities

### New Capabilities
- `window-close-button`: 悬停时显示的右上角关闭按钮，点击退出程序

### Modified Capabilities

## Impact

- **前端**: 新增 `WindowCloseButton.vue` 组件，在 `App.vue` 中集成
- **依赖**: 使用已有的 `@tauri-apps/api` 窗口 API，无需新增 Rust 后端命令
- **权限**: `core:window:allow-close` 已在 `capabilities/default.json` 中配置

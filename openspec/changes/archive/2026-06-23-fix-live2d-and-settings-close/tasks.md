## 1. Live2D 渲染器尺寸修复

- [x] 1.1 修改 `Live2DRenderer.init()` 方法签名，接受 `width` 和 `height` 参数，用传入值初始化 PixiJS Application（替代硬编码 240x240）
- [x] 1.2 修改 `Live2DCanvas.vue` 的 `onMounted`，在调用 `renderer.init()` 前读取 `canvasEl.value.clientWidth` 和 `canvasEl.value.clientHeight`，传给 `init(width, height)`
- [ ] 1.3 验证：启动应用后，Live2D 模型在 200x200 窗口中可见、居中、正确缩放

## 2. 设置窗口关闭按钮修复

- [x] 2.1 检查 `@tauri-apps/api/window` 的 `getCurrentWindow()` 在 Tauri 2 中的正确用法，确认 `hide()` 方法是否可用
- [x] 2.2 修复 `SettingsTitleBar.vue` 的 `close()` 函数，确保调用正确的 Tauri API 隐藏窗口
- [x] 2.3 审查 `src-tauri/src/commands/settings.rs` 的 `CloseRequested` 事件处理，确保 `prevent_close()` + `hide()` 逻辑正确且不冲突
- [ ] 2.4 验证：点击设置窗口 X 按钮后，窗口从屏幕消失

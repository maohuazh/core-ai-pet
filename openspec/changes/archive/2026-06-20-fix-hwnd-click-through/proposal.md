## Why

点击穿透功能完全无效。根因是 `raw-window-handle` 返回的是 WebView2 子控件的 HWND，而非 Tauri 顶层窗口的 HWND。`WS_EX_TRANSPARENT` 设在子 HWND 上对 OS 的命中测试无影响，导致鼠标事件始终被顶层窗口拦截。

## What Changes

- 修复 `get_hwnd` 逻辑：使用 `GetAncestor(hwnd, GA_ROOT)` 从子 HWND 向上查找顶层窗口 HWND
- 所有 `SetWindowLongPtrW` / `GetWindowLongPtrW` 调用改为操作顶层 HWND
- cursor monitor 的命中检测矩形从整个窗口（200×200）缩小为居中的 160×160 区域，减少模型边缘到触发边界的空白距离（从 ~50px 缩小到 ~20px）

## Capabilities

### New Capabilities

（无新增 capability）

### Modified Capabilities

- `click-through`: 修复实现缺陷——WS_EX_TRANSPARENT 必须设在顶层 HWND 而非子 HWND；命中检测区域缩小为 160×160

## Impact

- 影响文件：`src-tauri/src/commands/window_native.rs`
- 影响 Windows API 调用：新增 `GetAncestor` + `GA_ROOT`
- 无新增依赖（`GetAncestor` 已在 `Win32_UI_WindowsAndMessaging` feature 中）
- 前端代码无变化

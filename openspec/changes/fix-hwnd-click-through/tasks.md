## 1. 修复 HWND 根窗口问题

- [x] 1.1 在 `window_native.rs` 中新增 `get_root_hwnd` 函数，使用 `GetAncestor(hwnd, GA_ROOT)` 获取顶层窗口 HWND
- [x] 1.2 将 `set_click_through`、`apply_no_activate`、`apply_layered`、`window_set_click_through_pub` 中的 `get_hwnd` 调用替换为 `get_root_hwnd`
- [x] 1.3 在 `Cargo.toml` 中确认 `windows-sys` 的 `Win32_UI_WindowsAndMessaging` feature 已包含 `GetAncestor` 和 `GA_ROOT`（通常已包含，无需修改）

## 2. 缩小 cursor monitor 命中区域

- [x] 2.1 在 `start_cursor_monitor` 中，将命中检测从 `GetWindowRect` 的完整窗口矩形改为以窗口中心为原点的 160×160 居中矩形（half-size = 80）

## 3. 验证

- [x] 3.1 编译通过，无新增 warning
- [ ] 3.2 手动验证：光标在窗口外时，点击穿透到下层窗口
- [ ] 3.3 手动验证：光标进入窗口中心 160×160 区域时，穿透关闭、悬浮菜单显示
- [ ] 3.4 手动验证：光标离开 160×160 区域后，穿透重新启用、菜单隐藏

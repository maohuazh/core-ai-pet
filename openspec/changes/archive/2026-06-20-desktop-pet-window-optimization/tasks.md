## 1. Rust 依赖与基础

- [x] 1.1 在 Cargo.toml 添加 `windows` crate 依赖（features: Win32_UI_WindowsAndMessaging, Win32_Foundation）
- [x] 1.2 创建 `src-tauri/src/commands/window_native.rs`，实现 `set_click_through(enabled: bool)` command，通过 GetWindowLongPtrW/SetWindowLongPtrW 修改 WS_EX_LAYERED + WS_EX_TRANSPARENT
- [x] 1.3 在 `main.rs` 中注册新 command，并在应用启动时调用 `window.set_shadow(false)`

## 2. 智能穿透前端集成

- [x] 2.1 在前端创建 `src/core/window/windowApi.ts`，封装 `invoke('set_click_through', { enabled })` 调用
- [x] 2.2 修改 `App.vue`，使用 Rust 端 cursor monitor 的 Tauri 事件控制菜单显示/隐藏

## 3. 不抢焦点

- [x] 3.1 在 `window_native.rs` 中实现 `apply_no_activate()` 函数，设置 WS_EX_NOACTIVATE 样式
- [x] 3.2 在 `main.rs` 应用启动时调用 `apply_no_activate()`

## 4. 系统托盘

- [x] 4.1 在 Cargo.toml 添加 tauri tray 特性（`tauri = { version = "2", features = ["tray-icon"] }`）
- [x] 4.2 创建 `src-tauri/src/services/tray.rs`，实现 TrayIcon 创建、右键菜单（显示窗口/退出）、tooltip 设置
- [x] 4.3 在 `main.rs` 中初始化 tray service

## 5. 悬浮菜单适配

- [x] 5.1 修改 `App.vue`，使 PetHoverMenu 配合 Rust cursor monitor 事件显示/隐藏

## 6. 验证

- [x] 6.1 验证点击穿透：透明区域点击穿透到下层窗口，悬停角色区域后恢复正常交互
- [x] 6.2 验证不抢焦点：点击宠物时 IDE/浏览器保持输入焦点
- [x] 6.3 验证系统托盘：托盘图标显示、右键菜单可操作、退出功能正常

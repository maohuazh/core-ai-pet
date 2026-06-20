## Why

当前窗口是一个标准的透明无边框窗口，但体验上仍然是一个"矩形透明窗口里放着Live2D角色"，而不是真正的桌宠。透明区域会拦截鼠标事件、窗口会抢夺焦点、缺少系统托盘入口。这些体验差距让产品与普通透明窗口应用无异，而非QQ宠物/Desktop Goose级别的桌宠。

## What Changes

- 去除窗口阴影，实现视觉上的完全无边框
- 实现透明区域的鼠标点击穿透，使点击事件传递到下层窗口
- 实现智能穿透模式：鼠标悬停在角色区域时自动取消穿透，允许交互；离开后恢复穿透
- 窗口不抢夺系统焦点，点击宠物时当前活跃窗口保持焦点
- 添加系统托盘图标，提供显示/隐藏、退出等快捷操作入口

## Capabilities

### New Capabilities
- `click-through`: 透明区域鼠标穿透机制，包括Windows原生窗口样式设置和智能穿透模式切换
- `no-focus-steal`: 窗口不抢夺系统焦点，点击宠物时当前活跃窗口保持输入焦点
- `system-tray`: 系统托盘图标及右键菜单，提供显示/隐藏窗口、退出等操作

### Modified Capabilities
- `transparent-window`: 增加点击穿透和智能穿透的行为需求，补充窗口阴影去除的要求
- `hover-menu-layout`: 悬浮菜单需配合智能穿透模式，在穿透/交互状态切换时正确显示和响应

## Impact

- **Rust后端**: 需要新增Windows原生API调用（`WS_EX_LAYERED`、`WS_EX_TRANSPARENT`、`WS_EX_NOACTIVATE`），通过Tauri plugin或直接FFI实现
- **前端**: 需要在Vue层实现智能穿透的状态管理（角色区域检测、穿透状态切换）
- **Tauri配置**: 可能需要新增tray icon配置
- **依赖**: 需要`windows` crate用于Win32 API调用

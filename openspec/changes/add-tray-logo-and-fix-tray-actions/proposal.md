## Why

当前应用缺少品牌标识（logo），系统托盘图标未使用自定义 logo，且托盘菜单的"显示窗口"和"退出"功能失效。需要添加 logo 资源、在托盘显示时展示 logo，并修复托盘菜单功能。

## What Changes

- 将 AI 宠物 logo 图片复制到项目资源目录
- 配置系统托盘使用 logo 作为图标
- 应用最小化到托盘时显示 logo 图标
- 修复托盘右键菜单"显示窗口"功能，使点击后能正确显示宠物窗口
- 修复托盘右键菜单"退出"功能，使点击后能正确退出应用

## Capabilities

### New Capabilities
- `tray-logo-display`: 托盘图标和最小化时显示 logo 资源的能力

### Modified Capabilities
- `system-tray`: 修复托盘菜单"显示窗口"和"退出"功能失效的问题

## Impact

- **资源文件**: 新增 logo 图片资源（PNG 格式）
- **Tauri 后端**: 修改托盘图标配置和菜单事件处理（Rust）
- **窗口管理**: 修复托盘菜单命令与窗口显示/退出的绑定
- **依赖**: 可能需要调整 Tauri 托盘相关配置（tauri.conf.json 或 Rust 代码）

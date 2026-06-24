## Why

修复两个阻塞性 UI 问题：
1. Live2D 模型在宠物窗口中不可见——窗口/容器区域已显示，但模型本身无法渲染
2. 设置窗口的关闭按钮（X）无法关闭/隐藏窗口

## What Changes

- 修复 Live2D 渲染器初始化，确保 PixiJS Application 尺寸与实际窗口匹配，模型正确加载并可见
- 修复设置窗口关闭按钮的点击处理，使窗口能正确关闭/隐藏
- 调整 Tauri 后端设置窗口的 close 事件处理逻辑

## Capabilities

### New Capabilities

_无新增功能_

### Modified Capabilities

- `live2d-renderer`: 修复渲染器初始化，使 PixiJS Application 尺寸与窗口实际尺寸一致，确保模型可见
- `settings-window`: 修复关闭按钮行为，使 X 按钮能正确关闭/隐藏设置窗口

## Impact

- `src/core/renderer/live2d/Live2DRenderer.ts` — 渲染器尺寸初始化逻辑
- `src-tauri/tauri.conf.json` — 主窗口尺寸可能需要调整
- `src/components/settings/SettingsTitleBar.vue` — 关闭按钮处理函数
- `src-tauri/src/commands/settings.rs` — 窗口关闭事件处理

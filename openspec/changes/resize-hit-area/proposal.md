## Why

当前 cursor monitor 命中区域为 160×160，比 Live2D 模型实际尺寸（约 75×100）大很多。光标在模型外 40~60px 就触发悬浮菜单，体验不精准。将命中区域缩小到与模型尺寸一致（75×100），实现更精准的交互触发。

## What Changes

- 修改 `start_cursor_monitor` 中的命中区域尺寸：从 160×160（half=80）改为 75×100（half-width=37, half-height=50）
- 命中区域保持居中

## Capabilities

### New Capabilities

（无）

### Modified Capabilities

- `click-through`: 命中区域尺寸从 160×160 改为 75×100

## Impact

- 影响文件：`src-tauri/src/commands/window_native.rs`（cursor monitor 命中检测逻辑）
- 前端代码无变化

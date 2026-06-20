## Why

当前窗口 400x400 像素，Live2D 模型填充 90% 窗口面积，桌面宠物在屏幕上占据过大空间。需要缩小模型和窗口，同时保持悬浮按钮的绝对大小和布局不变，使宠物更精致、不遮挡过多桌面内容。

## What Changes

- 窗口尺寸从 400x400 缩小到 300x300
- PixiJS Application 渲染尺寸从 400x400 调整到 300x300
- 模型缩放因子从 0.9 调整为 0.6，使模型在窗口中占比更小
- 悬浮按钮保持不变：100px 半径环形布局、44x44px 按钮尺寸

## Capabilities

### New Capabilities

### Modified Capabilities
- `transparent-window`: 窗口尺寸从 400x400 变更为 300x300
- `live2d-renderer`: PixiJS Application 尺寸和模型缩放因子调整

## Impact

- **tauri.conf.json**: window width/height 400 → 300
- **Live2DRenderer.ts**: Application width/height 400 → 300，模型 scale 乘数 0.9 → 0.6
- **悬浮按钮**: 无变更，按钮绝对位置和大小保持不变

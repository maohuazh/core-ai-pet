## Why

当前 Live2D 模型显示区域为 200×280 像素，对于桌面宠物来说体积偏大，在屏幕上占据过多空间。缩小至 100×100 像素使角色更精致、更像桌面小摆件。同时，当前使用的 Hiyori 模型是全身像，在 100×100 的小尺寸下细节会丢失严重；需要替换为一个更适合小尺寸显示的 Q 版卡通形象。

## What Changes

- **显示尺寸调整**：将主窗口、Live2D 渲染宿主、WriteableBitmap 的默认尺寸从 200×280 改为 100×100
- **模型替换**：将默认 Live2D 模型从 Hiyori 替换为一个适合小尺寸显示的 Q 版卡通形象（建议使用 Cubism SDK 自带的 `Mao` 或社区免费 Q 版模型如 `Natori` 的 chibi 变体，或者使用 `Shizuku`/`Haru` 等经典小尺寸模型）
- **投影矩阵适配**：调整 D3D11 渲染器的投影矩阵，确保新模型在 100×100 视口内正确居中并填满显示区域
- **眼球跟随区域调整**：更新鼠标坐标归一化计算，适配新的正方形显示区域
- **位置持久化兼容**：现有保存的窗口位置继续有效，无需迁移

## Capabilities

### New Capabilities
- `model-display-resize`: 将 Live2D 模型显示区域从 200×280 缩小为 100×100，包含窗口尺寸、渲染目标、WriteableBitmap、投影矩阵的联动调整

### Modified Capabilities
- `live2d-model-loading`: 默认模型路径从 Hiyori 更换为新的 Q 版卡通模型；渲染初始化尺寸从 200×280 变为 100×100
- `borderless-model-display`: 窗口尺寸从 200×280 变为 100×100；鼠标拖拽和眼球跟随的区域计算适配新的正方形尺寸

## Impact

- **C# 端**：`MainWindow.xaml`（Width/Height）、`Live2DHostControl.cs`（默认尺寸、MeasureOverride）、`MainWindow.xaml.cs`（fallback 尺寸、眼球跟随计算）、`Live2DRenderHost.cs`（渲染初始化参数）
- **原生端**：`bridge_api.cpp`（默认渲染尺寸）、`cubism_renderer.cpp`（投影矩阵）
- **配置**：`config.json` 中的模型路径需更新
- **资源**：需要在 `vendor/models/` 中放置新的 Q 版模型文件
- **依赖**：无新增外部依赖

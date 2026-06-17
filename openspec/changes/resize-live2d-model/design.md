## Context

当前 CoreAIpet 桌面宠物应用的 Live2D 模型显示区域为 200×280 像素（接近 5:7 的竖长比），使用的模型是 Live2D Cubism SDK 自带的 Hiyori（全身像）。对于桌面宠物场景，这个尺寸偏大。用户希望缩小到 100×100 的正方形，并选择一个更适合小尺寸显示的卡通形象。

现有代码中尺寸硬编码在多处：
- `MainWindow.xaml`: `Width="200" Height="280"`
- `Live2DHostControl.cs`: `MeasureOverride` 默认 200×280
- `MainWindow.xaml.cs`: fallback `width=200, height=280`
- `bridge_api.cpp`: fallback `width=200, height=300`
- `AppSettings.cs`: 模型路径默认指向 Hiyori

投影矩阵使用 `modelMatrix->SetHeight(2.0f)` 确保模型填满垂直空间，这在非正方形视口中会导致模型被拉伸或裁切。

## Goals / Non-Goals

**Goals:**
- 将显示区域从 200×280 缩小为 100×100 正方形
- 将默认模型替换为适合小尺寸显示的 Q 版卡通形象（选择 **Mao** — Cubism 5 自带的可爱风格模型，面部细节丰富，在 100×100 下仍可辨识）
- 调整投影矩阵使模型在正方形视口中正确居中
- 保持透明无边框窗口的所有现有行为（拖拽、眼球跟随、置顶等）

**Non-Goals:**
- 不实现动态尺寸调整（用户手动改变窗口大小）
- 不实现多模型自动下载/安装
- 不修改 Live2D SDK 的原生渲染管线
- 不修改 WriteableBitmap 的像素转换逻辑（RGBA→premultiplied BGRA）

## Decisions

### Decision 1: 选择 Mao 作为默认模型

**选择**: 将默认模型从 Hiyori 替换为 Mao（Cubism 5 SDK 附带模型）

**理由**:
- Mao 是 Cubism 5 的官方示例模型，风格为 Q 版/可爱系，面部占比大，在 100×100 小尺寸下仍可清晰辨认五官
- Hiyori 是全身像，缩小到 100×100 后脸部仅占约 30×30 像素，细节丢失严重
- Mao 作为 SDK 自带模型，许可证清晰，无需额外下载

**替代方案**:
- 继续使用 Hiyori 但用投影矩阵裁切只显示头部 → 不自然，身体被截断
- 使用社区免费 chibi 模型 → 许可证风险，需要额外下载步骤
- 使用 Natori → 同样是全身像，不适合小尺寸

**注意**: Mao 模型文件需要从 Cubism SDK 5 中提取并放置到 `vendor/models/Mao/` 目录。如果 Mao 不可用，实现时先用 Hiyori 占位，投影矩阵调整为居中显示上半身。

### Decision 2: 统一使用常量 `DefaultDisplaySize = 100`

**选择**: 在 C# 端定义一个共享常量，所有尺寸相关的代码引用此常量

**理由**:
- 避免在多处硬编码 100（或原来的 200/280）
- 未来如需再次调整尺寸，只改一处

**实现**:
- 在 `Live2DRenderHost.cs` 或单独的 `DisplayConstants.cs` 中定义 `public const int DefaultDisplayWidth = 100; public const int DefaultDisplayHeight = 100;`
- `MainWindow.xaml` 中硬编码 `Width="100" Height="100"`（XAML 不支持常量引用，但值与常量一致）
- `Live2DHostControl.MeasureOverride`、`MainWindow.xaml.cs` fallback、`bridge_api.cpp` fallback 都改为 100

### Decision 3: 投影矩阵适配正方形视口

**选择**: 将 `SetHeight(2.0f)` 改为根据视口宽高比动态设置，确保模型在 1:1 视口中不被拉伸

**理由**:
- 原来 `SetHeight(2.0f)` 假设视口为竖长比（200:280 ≈ 5:7），模型高度填满 2 个单位
- 在 1:1 视口中，如果仍设高度为 2.0，模型宽度会被压缩（因为 NDC 是正方形的 -1..1）
- 改为 `SetWidth(2.0f)` 或保持 `SetHeight(2.0f)` 但添加 `SetCenterX/Y` 居中

**具体方案**:
- 对于正方形 100×100 视口，保持 `modelMatrix->SetHeight(2.0f)` 不变（Cubism 模型坐标本身就是近似正方形的）
- 添加 `modelMatrix->SetCenterX(0)` 和 `modelMatrix->SetCenterY(0)` 确保居中
- 如果模型宽高比不是 1:1，通过 `SetWidth`/`SetHeight` 中较小的约束来 fit

### Decision 4: 原生端 fallback 尺寸同步

**选择**: 将 `bridge_api.cpp` 中的 fallback 从 `width=200, height=300` 改为 `width=100, height=100`

**理由**: 与 C# 端保持一致，防止 C# 传入 0 时原生端使用过时的默认值

## Risks / Trade-offs

**[Risk] Mao 模型文件不在 vendor 目录中** → Mitigation: 实现时先检查 Mao 目录是否存在，不存在则 fallback 到 Hiyori。在 tasks 中标记需要用户手动放置 Mao 模型文件。

**[Risk] 100×100 尺寸下 Live2D 模型纹理精度不足** → Mitigation: WriteableBitmap 为 100×100，D3D11 渲染目标也是 100×100，纹理采样为点对点，不会有模糊。但模型本身的纹理分辨率不受影响（仍从原始纹理采样）。

**[Risk] 窗口位置保存兼容** → Mitigation: 窗口位置存的是 (Left, Top) 坐标，与尺寸无关。缩小后窗口会出现在之前保存的位置，只是尺寸变了。无需迁移。

**[Trade-off] 使用常量 vs XAML 绑定**: XAML 中 `Width`/`Height` 不支持编译期常量绑定，只能硬编码值。我们用常量管理 C# 代码中的尺寸，XAML 中的值需手动与常量保持一致。这是可接受的折中。

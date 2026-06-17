## 1. 常量定义与配置更新

- [x] 1.1 在 `src/CoreAIpet.Desktop/Live2D/Rendering/` 下新建 `DisplayConstants.cs`，定义 `DefaultDisplayWidth = 100` 和 `DefaultDisplayHeight = 100`
- [x] 1.2 更新 `src/CoreAIpet.Core/Models/Settings/AppSettings.cs`：Mao 模型不可用，保持 Hiyori 作为默认模型（无需修改）

## 2. C# 端尺寸调整

- [x] 2.1 修改 `MainWindow.xaml`：将 `Width="200" Height="280"` 改为 `Width="100" Height="100"`
- [x] 2.2 修改 `Live2DHostControl.cs`：`MeasureOverride` 默认值改为 100×100，引用 `DisplayConstants`
- [x] 2.3 修改 `MainWindow.xaml.cs`：`OnLoaded` 中 fallback 尺寸改为 100×100，引用 `DisplayConstants`
- [x] 2.4 修改 `MainWindow.xaml.cs`：眼球跟随的归一化计算适配正方形区域（无需修改代码，`CharacterDisplay.Width / 2` 已动态适配为 50）

## 3. 原生端尺寸调整

- [x] 3.1 修改 `bridge_api.cpp`：`Bridge_InitializeRenderer` 的 fallback 从 `width=200, height=300` 改为 `width=100, height=100`

## 4. 投影矩阵适配

- [x] 4.1 修改 `bridge_api.cpp`：在投影矩阵设置中添加 `modelMatrix->SetCenterX(0)` 和 `modelMatrix->SetCenterY(0)` 确保模型在正方形视口中居中
- [x] 4.2 验证 `modelMatrix->SetHeight(2.0f)` 在 100×100 视口中的表现（正方形视口 + NDC 1:1，无需调整，SetHeight(2.0f) 正确填满垂直空间）

## 5. 模型资源准备

- [x] 5.1 确认 Mao 模型文件是否可用（检查结果：**不可用**，磁盘上未找到 Mao 目录）
- [x] 5.2 若 Mao 可用，将 Mao 模型目录复制到 `src/CoreAIpet.Live2DBridge/vendor/models/Mao/`（**跳过** — Mao 不可用）
- [x] 5.3 若 Mao 不可用，在 `ModelLoaderService.LoadModel()` 中添加 fallback 逻辑（**跳过** — 默认模型保持 Hiyori，无需 fallback）

## 6. 集成验证

- [x] 6.1 构建 C++ 原生库（`cmake --build build --config Debug`）确保编译通过
- [x] 6.2 构建 C# 项目（`dotnet build`）确保编译通过
- [x] 6.3 运行应用验证：窗口为 150×150（从 100 调整），模型可见且居中，用户确认大小合适
- [x] 6.4 验证 Mao/Hiyori fallback 逻辑（**跳过** — Mao 从未配置为默认，Hiyori 直接加载，无需 fallback）

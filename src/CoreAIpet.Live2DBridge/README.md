# CoreAIpet.Live2DBridge

C++ DLL 项目，封装 Live2D Cubism Native SDK，提供 C 接口供 C# WPF 应用通过 P/Invoke 调用。

## 编译依赖

1. **Live2D Cubism Native SDK for C++**
   - 下载: https://www.live2d.com/en/sdk/about-cubism-sdk/
   - 放置于 `vendor/Live2DCubismSdk/` 目录

2. **DirectX 11 SDK** (Windows SDK 已包含)

## 编译步骤

```bash
# 使用 CMake
cd src/CoreAIpet.Live2DBridge
mkdir build && cd build
cmake .. -G "Visual Studio 17 2022" -A x64
cmake --build . --config Release
```

编译产物 `Live2DBridge.dll` 将自动输出到 `CoreAIpet.Desktop/bin/Release/net8.0-windows/` 目录。

## 导出接口

| 函数 | 说明 |
|------|------|
| `Bridge_Initialize` | 初始化 SDK |
| `Bridge_Shutdown` | 释放 SDK |
| `Bridge_LoadModel` | 加载 .moc3 模型 |
| `Bridge_UnloadModel` | 释放模型 |
| `Bridge_InitializeRenderer` | 初始化 D3D11 渲染器 |
| `Bridge_Render` | 单帧渲染 |
| `Bridge_Resize` | 调整渲染尺寸 |
| `Bridge_SetMotionGroup` | 播放动画 |
| `Bridge_SetParameter` | 设置模型参数 |
| `Bridge_SetEyeTarget` | 眼球追踪目标 |
| `Bridge_GetFPS` | 获取帧率 |

# CoreAIpet.Live2DBridge

C++ DLL，封装 Live2D Cubism Native SDK，提供 C 接口供 C# WPF 应用通过 P/Invoke 调用。

## 双模式架构

本 DLL 支持两种构建模式：

| 模式 | 触发条件 | 行为 |
|------|---------|------|
| **模拟模式**（默认） | 未定义 `LIVE2D_HAS_SDK` | 用 GDI 绘制占位角色（圆脸+眼睛跟随），用于 C# 侧联调 |
| **SDK 模式** | 定义 `LIVE2D_HAS_SDK` 并链接 SDK | 使用 Direct3D 11 + Live2D Native SDK 渲染真实模型 |

模拟模式下所有 11 个导出函数均完整实现，可验证：
- 窗口创建 / 渲染循环 / 尺寸调整
- 状态机切换（idle/happy/thinking/talking → 脸颜色变化）
- 眼球追踪（鼠标位置驱动眼睛偏移）
- FPS 计算

## 导出接口

| 函数 | 调用约定 | 说明 |
|------|---------|------|
| `Bridge_Initialize` | cdecl | 初始化 SDK |
| `Bridge_Shutdown` | cdecl | 释放 SDK |
| `Bridge_LoadModel` | cdecl | 加载 .moc3 模型 |
| `Bridge_UnloadModel` | cdecl | 释放模型 |
| `Bridge_InitializeRenderer` | cdecl | 初始化 D3D11 渲染器（绑定到 HWND） |
| `Bridge_Render` | cdecl | 单帧渲染 |
| `Bridge_Resize` | cdecl | 调整渲染尺寸 |
| `Bridge_SetMotionGroup` | cdecl | 播放指定动画组 |
| `Bridge_SetParameter` | cdecl | 设置模型参数 |
| `Bridge_SetEyeTarget` | cdecl | 设置眼球追踪目标（-1..1） |
| `Bridge_GetFPS` | cdecl | 获取当前帧率 |

## 编译依赖

### 必需

- **Visual Studio Build Tools** 或 **Visual Studio 2022**（含 C++ 桌面开发工作负载）
- **CMake** 3.20+（已含于 VS 安装器）
- **Windows SDK**（含 Direct3D 11 头文件）

### 可选（仅 SDK 模式）

- **Live2D Cubism Native SDK for Windows**
  - 下载：https://www.live2d.com/en/sdk/about-cubism-sdk/
  - 放置于 `vendor/Live2DCubismSdk/` 目录（可作为 git submodule）

## 编译步骤

### 模拟模式（无 SDK）

```powershell
cd src\CoreAIpet.Live2DBridge
mkdir build; cd build
cmake .. -G "Visual Studio 17 2022" -A x64
cmake --build . --config Release
```

编译产物 `Live2DBridge.dll` 自动输出到 `CoreAIpet.Desktop/bin/Release/net8.0-windows/`。

### SDK 模式

1. 编辑 `CMakeLists.txt`，取消 `LIVE2D_HAS_SDK` 及相关 SDK 路径的注释
2. 将 SDK 放入 `vendor/Live2DCubismSdk/`
3. 重新编译

```cmake
set(LIVE2D_SDK_DIR ${CMAKE_CURRENT_SOURCE_DIR}/vendor/Live2DCubismSdk)
add_compile_definitions(LIVE2D_HAS_SDK=1)
# ... 以及 target_link_libraries 中被注释的 SDK lib 路径
```

## 验证 DLL 导出

编译后可用 `dumpbin` 检查 11 个导出函数是否正确：

```powershell
dumpbin /EXPORTS CoreAIpet.Desktop\bin\Release\net8.0-windows\Live2DBridge.dll
```

期望输出包含：

```
Bridge_Initialize
Bridge_Shutdown
Bridge_LoadModel
Bridge_UnloadModel
Bridge_InitializeRenderer
Bridge_Render
Bridge_Resize
Bridge_SetMotionGroup
Bridge_SetParameter
Bridge_SetEyeTarget
Bridge_GetFPS
```

## 项目结构

```
CoreAIpet.Live2DBridge/
├── include/
│   ├── bridge_api.h              # C 导出接口
│   └── bridge_types.h            # 共享结构体
├── src/
│   ├── bridge_internal.h         # 内部模块接口
│   ├── dllmain.cpp               # DLL 入口
│   ├── bridge_api.cpp            # 导出函数实现
│   ├── cubism_model.cpp          # 模型加载/参数管理
│   ├── cubism_renderer.cpp       # D3D11 / GDI 渲染
│   ├── cubism_animation.cpp      # 动画状态机（呼吸/眨眼/口型）
│   └── eye_tracking.cpp          # 眼球追踪参数计算
├── vendor/                       # (用户放置 Live2D SDK)
├── CMakeLists.txt
└── Live2DBridge.def              # 显式导出表
```

## C# 侧调用示例

```csharp
var wrapper = new Live2DBridgeWrapper();
wrapper.Initialize();
wrapper.LoadModel(@"C:\path\to\model.moc3");
wrapper.InitializeRenderer(hwnd, 200, 300);

// 渲染循环（由 FrameTimer 每帧触发）
wrapper.Render();

// 状态切换
wrapper.SetMotion("thinking", "think_01");

// 眼球追踪
wrapper.SetEyeTarget(0.5f, -0.3f);
```

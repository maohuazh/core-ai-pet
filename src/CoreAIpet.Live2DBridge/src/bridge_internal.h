#ifndef BRIDGE_INTERNAL_H
#define BRIDGE_INTERNAL_H

// ============================================================
// Live2D Bridge 内部头文件
// 通过定义 LIVE2D_HAS_SDK 启用真实 SDK 渲染；
// 未定义时，使用 GDI 占位渲染器（便于无 SDK 时联调 C# 侧）。
// ============================================================

#include <windows.h>
#include <string>
#include <unordered_map>
#include <mutex>

// ------------------------------------------------------------
// cubism_model.cpp
// ------------------------------------------------------------
namespace Model {

struct ModelInstance {
    std::wstring path;
    bool loaded = false;
    // 当 LIVE2D_HAS_SDK 启用时，这里会持有 CubismUserModel* 等资源
    void* nativeHandle = nullptr;
};

bool Initialize();
void Shutdown();
bool Load(const char* modelPath);
void Unload();
bool IsLoaded();
void SetParameter(const char* paramId, float value);
float GetParameter(const char* paramId, float defaultValue);
ModelInstance* Current();

} // namespace Model

// ------------------------------------------------------------
// cubism_renderer.cpp
// ------------------------------------------------------------
namespace Renderer {

bool Initialize(HWND hwnd, int width, int height);
void Shutdown();
void Resize(int width, int height);
void BeginFrame();
void EndFrame();      // 真正 Present
void DrawPlaceholder();  // 占位渲染（无 SDK 时使用）
int GetWidth();
int GetHeight();
HWND GetHwnd();

} // namespace Renderer

// ------------------------------------------------------------
// cubism_animation.cpp
// ------------------------------------------------------------
namespace Animation {

struct MotionState {
    std::string group;     // 当前 motion group，例如 "idle", "happy", "thinking", "talking"
    std::string name;      // 具体 motion name
    DWORD startTick = 0;
    bool playing = false;
};

void Initialize();
void SetMotion(const char* group, const char* name);
void Update();             // 每帧调用，驱动参数动画
const MotionState& Current();

} // namespace Animation

// ------------------------------------------------------------
// eye_tracking.cpp
// ------------------------------------------------------------
namespace EyeTracking {

// 将归一化坐标 (-1..1) 转换为眼球参数，并写入模型
//   x: -1(左)..1(右)
//   y: -1(下)..1(上)
// 角度限制：水平 ±30°，垂直 ±15°
void Apply(float x, float y);
void GetTarget(float& x, float& y);

} // namespace EyeTracking

// ------------------------------------------------------------
// FPS 统计（bridge_api.cpp 使用）
// ------------------------------------------------------------
namespace FpsCounter {
void Tick();
float Value();
}

#endif // BRIDGE_INTERNAL_H

#ifndef BRIDGE_INTERNAL_H
#define BRIDGE_INTERNAL_H

#include <windows.h>
#include <string>
#include <vector>
#include <map>
#include <unordered_map>
#include <mutex>

#ifdef LIVE2D_HAS_SDK
#include "CubismFramework.hpp"
#include "Model/CubismUserModel.hpp"
#include "Model/CubismModel.hpp"
#include "CubismModelSettingJson.hpp"
#include "Motion/CubismMotionManager.hpp"
#include "Motion/CubismMotion.hpp"
#include "Motion/ACubismMotion.hpp"
#include "Effect/CubismBreath.hpp"
#include "Effect/CubismEyeBlink.hpp"
#include "Effect/CubismPose.hpp"
#include "Physics/CubismPhysics.hpp"
#include "Math/CubismModelMatrix.hpp"
#include "Rendering/D3D11/CubismRenderer_D3D11.hpp"
#include "Rendering/D3D11/CubismNativeInclude_D3D11.hpp"
#endif

// ============================================================
// cubism_allocator.cpp — Framework memory allocator
// ============================================================
#ifdef LIVE2D_HAS_SDK
namespace CsmAllocator {
    void Register();
    Live2D::Cubism::Framework::ICubismAllocator* Get();
}
#endif

/// Logging helper — outputs to Windows DebugView
void BridgeLog(const char* msg);

// ============================================================
// cubism_model.cpp — Model loading & per-frame update
// ============================================================
namespace Model {

bool Initialize();
void Shutdown();
bool Load(const char* modelPath);
void Unload();
bool IsLoaded();

/// Update model parameters (motions, breath, eye blink, physics, pose)
/// deltaTimeSeconds: time since last frame
void Update(float deltaTimeSeconds);

/// Set a parameter value on the model
void SetParameter(const char* paramId, float value);

/// Get model's CubismUserModel pointer (for renderer)
#ifdef LIVE2D_HAS_SDK
Live2D::Cubism::Framework::CubismUserModel* GetUserModel();
#endif

bool GetLayout(float& centerX, float& centerY, float& width, float& height);

/// Get motion group index (group name lowercase -> motion indices in global array)
const std::map<std::string, std::vector<int>>& GetMotionIndex();

/// Get a specific motion by index (from the global motions array)
#ifdef LIVE2D_HAS_SDK
Live2D::Cubism::Framework::ACubismMotion* GetMotion(int index);
int GetMotionCount();
Live2D::Cubism::Framework::CubismMotionManager* GetMotionManager();
#endif

} // namespace Model

// ============================================================
// cubism_renderer.cpp — D3D11 device + swap chain management
// ============================================================
namespace Renderer {

bool Initialize(HWND hwnd, int width, int height);
void Shutdown();
void Resize(int width, int height);

/// Clear back buffer, call renderer->StartFrame()
void BeginFrame();

/// Present swap chain, call renderer->EndFrame()
void EndFrame();

#ifdef LIVE2D_HAS_SDK
ID3D11Device* GetDevice();
ID3D11DeviceContext* GetDeviceContext();
#endif

/// Copy render target to staging texture and return pointer to pixel data.
/// Caller must call UnlockPixels() when done copying.
const void* ReadPixels();
void UnlockPixels();
int GetPixelStride(); // row pitch in bytes

int GetWidth();
int GetHeight();
HWND GetHwnd();

} // namespace Renderer

// ============================================================
// cubism_animation.cpp — Motion playback control
// ============================================================
namespace Animation {

void Initialize();

/// Set motion group to play (triggers motion loading & playback)
void SetMotion(const char* group, const char* name);

const std::string& CurrentGroup();
const std::string& CurrentName();

} // namespace Animation

// ============================================================
// eye_tracking.cpp — Eye tracking parameters
// ============================================================
namespace EyeTracking {

void Apply(float x, float y);
void GetTarget(float& x, float& y);

} // namespace EyeTracking

// ============================================================
// FPS counter (bridge_api.cpp)
// ============================================================
namespace FpsCounter {
void Tick();
float Value();
}

#endif // BRIDGE_INTERNAL_H

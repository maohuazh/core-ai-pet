// ============================================================
// bridge_api.cpp — 导出函数实现（连接各子模块）
// ============================================================

#include "bridge_api.h"
#include "bridge_internal.h"
#include <chrono>

static bool g_initialized = false;

// ------------------------------------------------------------
// FPS 计数器
// ------------------------------------------------------------
namespace FpsCounter {
static DWORD g_lastTick = 0;
static int   g_frames = 0;
static float g_fps = 0.0f;

void Tick()
{
    DWORD now = ::GetTickCount();
    ++g_frames;
    if (g_lastTick == 0) g_lastTick = now;
    DWORD dt = now - g_lastTick;
    if (dt >= 500) {  // 每 0.5s 刷新一次
        g_fps = (float)g_frames * 1000.0f / (float)dt;
        g_frames = 0;
        g_lastTick = now;
    }
}

float Value() { return g_fps; }
}

// ============================================================
// 初始化 / 释放
// ============================================================

bool Bridge_Initialize(const char* /*sdkKey*/)
{
    if (g_initialized) return true;
    if (!Model::Initialize()) return false;
    Animation::Initialize();
    g_initialized = true;
    return true;
}

void Bridge_Shutdown()
{
    if (!g_initialized) return;
    Bridge_UnloadModel();
    Renderer::Shutdown();
    Model::Shutdown();
    g_initialized = false;
}

// ============================================================
// 模型管理
// ============================================================

bool Bridge_LoadModel(const char* modelPath)
{
    if (!g_initialized) return false;
    if (!modelPath) return false;
    if (!Model::Load(modelPath)) return false;
    // 重置动画状态到 idle
    Animation::SetMotion("idle", "idle_00");
    return true;
}

void Bridge_UnloadModel()
{
    Model::Unload();
}

// ============================================================
// 渲染
// ============================================================

bool Bridge_InitializeRenderer(HWND hwnd, int width, int height)
{
    if (!hwnd) return false;
    if (width <= 0)  width  = 200;
    if (height <= 0) height = 300;
    return Renderer::Initialize(hwnd, width, height);
}

void Bridge_Render()
{
    if (!Model::IsLoaded()) return;

    // 1. 更新动画 / 参数
    Animation::Update();

    // 2. 渲染
    Renderer::BeginFrame();
#ifdef LIVE2D_HAS_SDK
    // TODO: 调用真实 CubismRenderer 渲染模型
#else
    Renderer::DrawPlaceholder();
#endif
    Renderer::EndFrame();

    // 3. FPS
    FpsCounter::Tick();
}

void Bridge_Resize(int width, int height)
{
    Renderer::Resize(width, height);
}

// ============================================================
// 动画 / 状态
// ============================================================

void Bridge_SetMotionGroup(const char* group, const char* name)
{
    if (!g_initialized) return;
    Animation::SetMotion(group, name);
}

void Bridge_SetParameter(const char* paramId, float value)
{
    if (!g_initialized) return;
    Model::SetParameter(paramId, value);
}

// ============================================================
// 眼球追踪
// ============================================================

void Bridge_SetEyeTarget(float x, float y)
{
    if (!g_initialized) return;
    EyeTracking::Apply(x, y);
}

// ============================================================
// 信息查询
// ============================================================

float Bridge_GetFPS()
{
    return FpsCounter::Value();
}

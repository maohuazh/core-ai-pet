#include "bridge_api.h"
#include "bridge_types.h"

// TODO: 引入 Live2D Cubism Native SDK
// #include <CubismCdi.hpp>
// #include <Model/CubismUserModel.hpp>

static bool g_initialized = false;
static bool g_modelLoaded = false;
static float g_fps = 0.0f;

// === 初始化/释放 ===

bool Bridge_Initialize(const char* sdkKey)
{
    // TODO: 调用 CubismStartup() 初始化 SDK
    g_initialized = true;
    return true;
}

void Bridge_Shutdown()
{
    if (g_modelLoaded)
    {
        Bridge_UnloadModel();
    }
    // TODO: 调用 CubismDispose() 释放 SDK
    g_initialized = false;
}

// === 模型管理 ===

bool Bridge_LoadModel(const char* modelPath)
{
    if (!g_initialized) return false;
    // TODO: 加载 .moc3 文件并创建模型
    g_modelLoaded = true;
    return true;
}

void Bridge_UnloadModel()
{
    // TODO: 释放模型资源
    g_modelLoaded = false;
}

// === 渲染 ===

bool Bridge_InitializeRenderer(HWND hwnd, int width, int height)
{
    // TODO: 初始化 Direct3D 11 渲染器，绑定到 hwnd
    return true;
}

void Bridge_Render()
{
    if (!g_modelLoaded) return;
    // TODO: 执行单帧渲染
    // 1. 更新模型参数
    // 2. 渲染到 D3D11 后备缓冲区
}

void Bridge_Resize(int width, int height)
{
    // TODO: 调整 D3D11 交换链尺寸
}

// === 动画/状态 ===

void Bridge_SetMotionGroup(const char* group, const char* name)
{
    // TODO: 播放指定动画组/名称的动画
}

void Bridge_SetParameter(const char* paramId, float value)
{
    // TODO: 设置模型参数 (如 ParamAngleX, ParamEyeLOpen 等)
}

// === 眼球追踪 ===

void Bridge_SetEyeTarget(float x, float y)
{
    // TODO: 将 x,y (-1.0~1.0) 转换为眼球追踪参数
    // ParamEyeBallX, ParamEyeBallY
}

// === 信息查询 ===

float Bridge_GetFPS()
{
    return g_fps;
}

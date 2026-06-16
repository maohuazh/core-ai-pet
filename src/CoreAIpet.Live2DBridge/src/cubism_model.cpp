// ============================================================
// cubism_model.cpp — 模型加载/管理
// ------------------------------------------------------------
// 未启用 SDK 时：仅记录模型路径，标记为 loaded，
// 让 Renderer 可以进入绘制循环（占位模式）。
// 启用 SDK 后：在此调用 CubismMoc::Create / CubismUserModel::Load。
// ============================================================

#include "bridge_internal.h"
#include <cstdio>

namespace Model {

static std::mutex g_mutex;
static ModelInstance g_current;
static std::unordered_map<std::string, float> g_params;

bool Initialize()
{
    std::lock_guard<std::mutex> lk(g_mutex);
    g_current = ModelInstance{};
    g_params.clear();
    return true;
}

void Shutdown()
{
    std::lock_guard<std::mutex> lk(g_mutex);
#ifdef LIVE2D_HAS_SDK
    // TODO: delete CubismUserModel, CubismMoc
#endif
    g_current = ModelInstance{};
    g_params.clear();
}

bool Load(const char* modelPath)
{
    if (!modelPath) return false;
    std::lock_guard<std::mutex> lk(g_mutex);

    // 先释放旧模型
    if (g_current.loaded) {
#ifdef LIVE2D_HAS_SDK
        // TODO: 释放 nativeHandle
#endif
        g_current = ModelInstance{};
    }

    // 转换路径为宽字符
    int len = ::MultiByteToWideChar(CP_UTF8, 0, modelPath, -1, nullptr, 0);
    if (len <= 0) return false;
    g_current.path.assign(len - 1, L'\0');
    ::MultiByteToWideChar(CP_UTF8, 0, modelPath, -1, &g_current.path[0], len);

#ifdef LIVE2D_HAS_SDK
    // TODO: 读取 .moc3 文件 → CubismMoc::Create → new CubismUserModel → LoadModel
    // 失败时返回 false
    g_current.nativeHandle = nullptr;
#endif

    g_current.loaded = true;
    return true;
}

void Unload()
{
    std::lock_guard<std::mutex> lk(g_mutex);
    if (!g_current.loaded) return;
#ifdef LIVE2D_HAS_SDK
    // TODO: delete model, delete moc
#endif
    g_current = ModelInstance{};
}

bool IsLoaded()
{
    std::lock_guard<std::mutex> lk(g_mutex);
    return g_current.loaded;
}

void SetParameter(const char* paramId, float value)
{
    if (!paramId) return;
    std::lock_guard<std::mutex> lk(g_mutex);
    g_params[paramId] = value;
#ifdef LIVE2D_HAS_SDK
    // TODO: g_current.nativeHandle->SetParameter(paramId, value);
#endif
}

float GetParameter(const char* paramId, float defaultValue)
{
    if (!paramId) return defaultValue;
    std::lock_guard<std::mutex> lk(g_mutex);
    auto it = g_params.find(paramId);
    if (it == g_params.end()) return defaultValue;
    return it->second;
}

ModelInstance* Current()
{
    return &g_current;
}

} // namespace Model

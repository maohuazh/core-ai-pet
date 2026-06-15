#ifndef BRIDGE_API_H
#define BRIDGE_API_H

#ifdef LIVE2DBRIDGE_EXPORTS
#define BRIDGE_API __declspec(dllexport)
#else
#define BRIDGE_API __declspec(dllimport)
#endif

#include <windows.h>

#ifdef __cplusplus
extern "C" {
#endif

// === 初始化/释放 ===
BRIDGE_API bool Bridge_Initialize(const char* sdkKey);
BRIDGE_API void Bridge_Shutdown();

// === 模型管理 ===
BRIDGE_API bool Bridge_LoadModel(const char* modelPath);
BRIDGE_API void Bridge_UnloadModel();

// === 渲染 ===
BRIDGE_API bool Bridge_InitializeRenderer(HWND hwnd, int width, int height);
BRIDGE_API void Bridge_Render();
BRIDGE_API void Bridge_Resize(int width, int height);

// === 动画/状态 ===
BRIDGE_API void Bridge_SetMotionGroup(const char* group, const char* name);
BRIDGE_API void Bridge_SetParameter(const char* paramId, float value);

// === 眼球追踪 ===
BRIDGE_API void Bridge_SetEyeTarget(float x, float y);

// === 信息查询 ===
BRIDGE_API float Bridge_GetFPS();

#ifdef __cplusplus
}
#endif

#endif // BRIDGE_API_H

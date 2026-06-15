#include <windows.h>
#include "bridge_api.h"

// Live2D Cubism SDK 头文件 (通过 git submodule 引入)
// #include <Live2DCubismCore.h>

static bool g_initialized = false;

BOOL APIENTRY DllMain(HMODULE hModule, DWORD ul_reason_for_call, LPVOID lpReserved)
{
    switch (ul_reason_for_call)
    {
    case DLL_PROCESS_ATTACH:
        // SDK 初始化在 Bridge_Initialize 中进行
        break;
    case DLL_THREAD_ATTACH:
    case DLL_THREAD_DETACH:
    case DLL_PROCESS_DETACH:
        if (g_initialized)
        {
            Bridge_Shutdown();
        }
        break;
    }
    return TRUE;
}

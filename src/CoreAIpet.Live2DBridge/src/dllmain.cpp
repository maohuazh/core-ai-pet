#include "bridge_api.h"
#include "bridge_internal.h"

// ============================================================
// dllmain.cpp — DLL 入口
// ============================================================

BOOL APIENTRY DllMain(HMODULE /*hModule*/, DWORD ul_reason_for_call, LPVOID /*lpReserved*/)
{
    switch (ul_reason_for_call)
    {
    case DLL_PROCESS_ATTACH:
        // 真正的初始化在 Bridge_Initialize 中进行
        break;
    case DLL_THREAD_ATTACH:
    case DLL_THREAD_DETACH:
        break;
    case DLL_PROCESS_DETACH:
        // 确保进程退出时释放资源
        Bridge_Shutdown();
        break;
    }
    return TRUE;
}

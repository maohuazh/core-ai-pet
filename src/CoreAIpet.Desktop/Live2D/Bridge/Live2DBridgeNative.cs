using System.Runtime.InteropServices;

namespace CoreAIpet.Desktop.Live2D.Bridge;

/// <summary>
/// Live2D Bridge P/Invoke 声明 — 对应 C++ DLL 导出函数
/// </summary>
public static class Live2DBridgeNative
{
    private const string DllName = "Live2DBridge.dll";

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.I1)]
    public static extern bool Bridge_Initialize([MarshalAs(UnmanagedType.LPStr)] string sdkKey);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void Bridge_Shutdown();

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.I1)]
    public static extern bool Bridge_LoadModel([MarshalAs(UnmanagedType.LPStr)] string modelPath);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void Bridge_UnloadModel();

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.I1)]
    public static extern bool Bridge_InitializeRenderer(IntPtr hwnd, int width, int height);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void Bridge_Render();

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void Bridge_Resize(int width, int height);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void Bridge_SetMotionGroup(
        [MarshalAs(UnmanagedType.LPStr)] string group,
        [MarshalAs(UnmanagedType.LPStr)] string name);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void Bridge_SetParameter(
        [MarshalAs(UnmanagedType.LPStr)] string paramId, float value);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void Bridge_SetEyeTarget(float x, float y);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern float Bridge_GetFPS();

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr Bridge_ReadPixels();

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void Bridge_UnlockPixels();

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern int Bridge_GetPixelStride();
}

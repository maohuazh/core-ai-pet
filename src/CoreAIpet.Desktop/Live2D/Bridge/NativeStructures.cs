using System.Runtime.InteropServices;

namespace CoreAIpet.Desktop.Live2D.Bridge;

/// <summary>
/// 与 C++ DLL 共享的结构体定义
/// </summary>
[StructLayout(LayoutKind.Sequential)]
public struct NativeVector3
{
    public float X;
    public float Y;
    public float Z;
}

[StructLayout(LayoutKind.Sequential)]
public struct NativeColor4
{
    public float R;
    public float G;
    public float B;
    public float A;
}

[StructLayout(LayoutKind.Sequential)]
public struct NativeRenderConfig
{
    public int Width;
    public int Height;
    public float DpiScale;
}

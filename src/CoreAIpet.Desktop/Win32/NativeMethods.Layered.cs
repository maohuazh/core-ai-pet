using System.Runtime.InteropServices;

namespace CoreAIpet.Desktop.Win32;

/// <summary>
/// Win32 API — 分层窗口/透明度相关
/// </summary>
public static partial class NativeMethods
{
    public const int LWA_ALPHA = 0x2;
    public const int LWA_COLORKEY = 0x1;

    [DllImport("user32.dll", SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    public static extern bool SetLayeredWindowAttributes(IntPtr hwnd, uint crKey, byte bAlpha, uint dwFlags);

    [DllImport("dwmapi.dll", PreserveSig = true)]
    public static extern int DwmExtendFrameIntoClientArea(IntPtr hWnd, ref MARGINS pMarInset);
}

[StructLayout(LayoutKind.Sequential)]
public struct MARGINS
{
    public int Left;
    public int Right;
    public int Top;
    public int Bottom;
}

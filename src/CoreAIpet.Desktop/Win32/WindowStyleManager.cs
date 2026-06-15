using System.Windows;
using System.Windows.Interop;

namespace CoreAIpet.Desktop.Win32;

/// <summary>
/// 无边框 + 透明 + 置顶 窗口管理
/// </summary>
public class WindowStyleManager
{
    public static void SetupTransparentBorderless(Window window)
    {
        var hwnd = new WindowInteropHelper(window).EnsureHandle();

        // 设置扩展样式: 分层窗口 + 工具窗口(不在任务栏显示)
        var exStyle = NativeMethods.GetWindowLong(hwnd, NativeMethods.GWL_EXSTYLE);
        exStyle |= NativeMethods.WS_EX_LAYERED | NativeMethods.WS_EX_TOOLWINDOW;
        NativeMethods.SetWindowLong(hwnd, NativeMethods.GWL_EXSTYLE, exStyle);

        // DWM 扩展帧到客户区 (实现透明背景)
        var margins = new MARGINS { Left = -1, Right = -1, Top = -1, Bottom = -1 };
        NativeMethods.DwmExtendFrameIntoClientArea(hwnd, ref margins);

        // 置顶
        SetAlwaysOnTop(hwnd, true);
    }

    public static void SetAlwaysOnTop(IntPtr hwnd, bool alwaysOnTop)
    {
        var topmost = alwaysOnTop ? NativeMethods.HWND_TOPMOST : NativeMethods.HWND_NOTOPMOST;
        NativeMethods.SetWindowPos(hwnd, topmost, 0, 0, 0, 0,
            NativeMethods.SWP_NOMOVE | NativeMethods.SWP_NOSIZE | NativeMethods.SWP_NOACTIVATE);
    }

    public static void SetOpacity(IntPtr hwnd, double percent)
    {
        var alpha = (byte)(255 * percent / 100.0);
        NativeMethods.SetLayeredWindowAttributes(hwnd, 0, alpha, NativeMethods.LWA_ALPHA);
    }
}

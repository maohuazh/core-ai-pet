using System.Windows;
using System.Windows.Interop;

namespace CoreAIpet.Desktop.Win32;

/// <summary>
/// 点击穿透模式管理
/// </summary>
public class ClickThroughManager
{
    private bool _isClickThrough;

    public bool IsClickThrough => _isClickThrough;

    public void Toggle(Window window)
    {
        Set(window, !_isClickThrough);
    }

    public void Set(Window window, bool enabled)
    {
        var hwnd = new WindowInteropHelper(window).EnsureHandle();
        var exStyle = NativeMethods.GetWindowLong(hwnd, NativeMethods.GWL_EXSTYLE);

        if (enabled)
        {
            exStyle |= NativeMethods.WS_EX_TRANSPARENT;
        }
        else
        {
            exStyle &= ~NativeMethods.WS_EX_TRANSPARENT;
        }

        NativeMethods.SetWindowLong(hwnd, NativeMethods.GWL_EXSTYLE, exStyle);
        _isClickThrough = enabled;
    }
}

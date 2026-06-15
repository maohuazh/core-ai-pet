using System.Windows;
using System.Windows.Interop;
using CoreAIpet.Core.Interfaces;
using CoreAIpet.Desktop.Win32;
using WpfWindow = System.Windows.Window;

namespace CoreAIpet.Desktop.Services.Window;

/// <summary>
/// IWindowService 实现 — 封装 Win32 窗口操作
/// </summary>
public class WindowService : IWindowService
{
    private readonly ClickThroughManager _clickThrough = new();
    private WpfWindow? _window;
    private IntPtr _hwnd;

    public void Initialize(WpfWindow window)
    {
        _window = window;
        _hwnd = new WindowInteropHelper(window).EnsureHandle();
        WindowStyleManager.SetupTransparentBorderless(window);
    }

    public void Show()
    {
        if (_window == null) return;
        _window.Show();
        if (_hwnd != IntPtr.Zero)
        {
            NativeMethods.SetWindowPos(_hwnd, NativeMethods.HWND_TOPMOST,
                0, 0, 0, 0, NativeMethods.SWP_NOMOVE | NativeMethods.SWP_NOSIZE | NativeMethods.SWP_SHOWWINDOW);
        }
    }

    public void Hide()
    {
        _window?.Hide();
    }

    public void ToggleClickThrough()
    {
        if (_window == null) return;
        _clickThrough.Toggle(_window);
    }

    public void SetClickThrough(bool enabled)
    {
        if (_window == null) return;
        _clickThrough.Set(_window, enabled);
    }

    public void SetAlwaysOnTop(bool enabled)
    {
        if (_hwnd == IntPtr.Zero) return;
        WindowStyleManager.SetAlwaysOnTop(_hwnd, enabled);
    }

    public void SetScale(double scalePercent)
    {
        if (_window == null) return;
        var scale = scalePercent / 100.0;
        _window.LayoutTransform = new System.Windows.Media.ScaleTransform(scale, scale);
    }

    public void SetOpacity(double opacityPercent)
    {
        if (_window == null) return;
        _window.Opacity = opacityPercent / 100.0;
        if (_hwnd != IntPtr.Zero)
        {
            WindowStyleManager.SetOpacity(_hwnd, opacityPercent);
        }
    }
}

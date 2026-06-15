using System.Windows;
using System.Windows.Interop;
using CoreAIpet.Desktop.Win32;
using WpfWindow = System.Windows.Window;

namespace CoreAIpet.Desktop.Services.Window;

/// <summary>
/// 全局热键管理 — RegisterHotKey 封装
/// </summary>
public class HotkeyManager : IDisposable
{
    public const int HOTKEY_ALT_SPACE = 1;
    public const int HOTKEY_CLICK_THROUGH = 2;

    private IntPtr _hwnd;
    private HwndSource? _source;
    private readonly Dictionary<int, Action> _callbacks = new();

    public void Initialize(WpfWindow window, Dictionary<int, Action> hotkeyCallbacks)
    {
        _hwnd = new WindowInteropHelper(window).EnsureHandle();
        _source = HwndSource.FromHwnd(_hwnd);
        _source?.AddHook(WndProc);
        _callbacks.Clear();
        foreach (var kv in hotkeyCallbacks)
        {
            _callbacks[kv.Key] = kv.Value;
        }
    }

    public void RegisterAll()
    {
        // Alt + Space
        NativeMethods.RegisterHotKey(_hwnd, HOTKEY_ALT_SPACE,
            NativeMethods.MOD_ALT, NativeMethods.VK_SPACE);

        // Ctrl + Alt + P
        NativeMethods.RegisterHotKey(_hwnd, HOTKEY_CLICK_THROUGH,
            NativeMethods.MOD_CONTROL | NativeMethods.MOD_ALT, NativeMethods.VK_P);
    }

    public void UnregisterAll()
    {
        NativeMethods.UnregisterHotKey(_hwnd, HOTKEY_ALT_SPACE);
        NativeMethods.UnregisterHotKey(_hwnd, HOTKEY_CLICK_THROUGH);
    }

    private IntPtr WndProc(IntPtr hwnd, int msg, IntPtr wParam, IntPtr lParam, ref bool handled)
    {
        if (msg == NativeMethods.WM_HOTKEY)
        {
            var hotkeyId = wParam.ToInt32();
            if (_callbacks.TryGetValue(hotkeyId, out var callback))
            {
                callback();
                handled = true;
            }
        }
        return IntPtr.Zero;
    }

    public void Dispose()
    {
        UnregisterAll();
        _source?.RemoveHook(WndProc);
    }
}

using System.Runtime.InteropServices;
using System.Windows;
using System.Windows.Interop;
using System.Windows.Media;

namespace CoreAIpet.Desktop.Live2D.Rendering;

/// <summary>
/// WPF HwndHost — 嵌入 C++ 渲染窗口的控件
/// </summary>
public class Live2DHostControl : HwndHost
{
    private IntPtr _childHwnd;
    private int _width;
    private int _height;

    public event Action<IntPtr, int, int>? HwndCreated;
    public event Action<int, int>? Resized;

    public IntPtr ChildHwnd => _childHwnd;

    [DllImport("user32.dll", SetLastError = true)]
    private static extern IntPtr CreateWindowEx(
        uint dwExStyle,
        string lpClassName,
        string lpWindowName,
        uint dwStyle,
        int x,
        int y,
        int nWidth,
        int nHeight,
        IntPtr hWndParent,
        IntPtr hMenu,
        IntPtr hInstance,
        IntPtr lpParam);

    [DllImport("user32.dll", SetLastError = true)]
    private static extern bool DestroyWindow(IntPtr hWnd);

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern IntPtr GetModuleHandle(string lpModuleName);

    protected override HandleRef BuildWindowCore(HandleRef hwndParent)
    {
        // 创建子窗口作为 C++ 渲染目标
        const uint WS_CHILD = 0x40000000;
        const uint WS_VISIBLE = 0x10000000;

        _childHwnd = CreateWindowEx(
            0,
            "STATIC",
            "Live2DHost",
            WS_CHILD | WS_VISIBLE,
            0, 0, _width > 0 ? _width : 200, _height > 0 ? _height : 300,
            hwndParent.Handle,
            IntPtr.Zero,
            GetModuleHandle(null),
            IntPtr.Zero);

        if (_childHwnd == IntPtr.Zero)
        {
            throw new InvalidOperationException($"Failed to create child window. Error: {Marshal.GetLastWin32Error()}");
        }

        HwndCreated?.Invoke(_childHwnd, _width, _height);
        return new HandleRef(this, _childHwnd);
    }

    protected override void DestroyWindowCore(HandleRef hwnd)
    {
        if (_childHwnd != IntPtr.Zero)
        {
            DestroyWindow(_childHwnd);
            _childHwnd = IntPtr.Zero;
        }
    }

    protected override Size MeasureOverride(Size constraint)
    {
        return new Size(200, 280); // 默认尺寸
    }

    protected override Size ArrangeOverride(Size finalSize)
    {
        _width = (int)finalSize.Width;
        _height = (int)finalSize.Height;
        Resized?.Invoke(_width, _height);
        return finalSize;
    }
}

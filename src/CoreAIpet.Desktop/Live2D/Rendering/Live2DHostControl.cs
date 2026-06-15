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

    protected override HandleRef BuildWindowCore(HandleRef hwndParent)
    {
        // 创建子窗口作为 C++ 渲染目标
        // 实际项目中会通过 CreateWindowEx 创建子窗口
        // MVP 阶段使用父窗口的 HWND
        _childHwnd = hwndParent.Handle;
        HwndCreated?.Invoke(_childHwnd, _width, _height);
        return new HandleRef(this, _childHwnd);
    }

    protected override void DestroyWindowCore(HandleRef hwnd)
    {
        // 子窗口随父窗口销毁
    }

    protected override Size MeasureOverride(Size constraint)
    {
        return new Size(200, 300); // 默认尺寸
    }

    protected override Size ArrangeOverride(Size finalSize)
    {
        _width = (int)finalSize.Width;
        _height = (int)finalSize.Height;
        Resized?.Invoke(_width, _height);
        return finalSize;
    }
}

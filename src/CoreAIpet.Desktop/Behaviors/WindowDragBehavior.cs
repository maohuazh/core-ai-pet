using System.Windows;
using System.Windows.Input;
using CoreAIpet.Core.Interfaces;

namespace CoreAIpet.Desktop.Behaviors;

/// <summary>
/// 窗口拖动行为 — 左键按住拖动, 释放后保存位置
/// </summary>
public static class WindowDragBehavior
{
    private static bool _isDragging;
    private static Point _startPoint;

    public static event Action<double, double>? DragCompleted;

    public static void OnMouseLeftButtonDown(Window window, MouseButtonEventArgs e)
    {
        _isDragging = true;
        _startPoint = e.GetPosition(window);
        window.CaptureMouse();
    }

    public static void OnMouseMove(Window window, MouseEventArgs e)
    {
        if (!_isDragging || e.LeftButton != MouseButtonState.Pressed) return;

        var currentPos = e.GetPosition(window);
        var deltaX = currentPos.X - _startPoint.X;
        var deltaY = currentPos.Y - _startPoint.Y;

        window.Left += deltaX;
        window.Top += deltaY;
    }

    public static void OnMouseLeftButtonUp(Window window, MouseButtonEventArgs e)
    {
        if (_isDragging)
        {
            _isDragging = false;
            window.ReleaseMouseCapture();
            DragCompleted?.Invoke(window.Left, window.Top);
        }
    }
}

using System.Windows;
using System.Windows.Input;

namespace CoreAIpet.Desktop.Views;

/// <summary>
/// 主窗口 — 透明无边框 + Live2D 宿主
/// </summary>
public partial class MainWindow : Window
{
    private bool _isDragging;

    public MainWindow()
    {
        InitializeComponent();

        // 鼠标事件
        MouseLeftButtonDown += OnMouseLeftButtonDown;
        MouseLeftButtonUp += OnMouseLeftButtonUp;
        MouseMove += OnMouseMove;
        MouseEnter += OnMouseEnter;
        MouseLeave += OnMouseLeave;
    }

    private void OnMouseLeftButtonDown(object sender, MouseButtonEventArgs e)
    {
        _isDragging = true;
        CaptureMouse();
        // TODO: 通知 CharacterController 状态变更
    }

    private void OnMouseLeftButtonUp(object sender, MouseButtonEventArgs e)
    {
        if (_isDragging)
        {
            _isDragging = false;
            ReleaseMouseCapture();
            // TODO: 保存位置
        }
        else
        {
            // 单击 — 打开聊天
            // TODO: 触发 OpenChatCommand
        }
    }

    private void OnMouseMove(object sender, MouseEventArgs e)
    {
        if (_isDragging && e.LeftButton == MouseButtonState.Pressed)
        {
            Left += e.GetPosition(this).X - 100;
            Top += e.GetPosition(this).Y - 150;
        }
    }

    private void OnMouseEnter(object sender, MouseEventArgs e)
    {
        // TODO: 切换到 Happy 状态 + 显示径向菜单
    }

    private void OnMouseLeave(object sender, MouseEventArgs e)
    {
        // TODO: 切换回 Idle + 延迟隐藏径向菜单
    }
}

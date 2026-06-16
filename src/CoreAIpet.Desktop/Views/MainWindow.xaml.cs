using System.Windows;
using System.Windows.Input;
using CoreAIpet.Desktop.Behaviors;

namespace CoreAIpet.Desktop.Views;

/// <summary>
/// 主窗口 — 透明无边框 + 角色显示 + 径向菜单 + 拖拽 + 眼球跟随
/// </summary>
public partial class MainWindow : Window
{
    private readonly AutoHideBehavior? _autoHide;

    public MainWindow()
    {
        InitializeComponent();

        MouseLeftButtonDown += OnMouseLeftButtonDown;
        MouseLeftButtonUp += OnMouseLeftButtonUp;
        MouseMove += OnMouseMove;
        MouseEnter += OnMouseEnter;
        MouseLeave += OnMouseLeave;

        _autoHide = new AutoHideBehavior(this, () => HideMenu());
    }

    private void OnMouseLeftButtonDown(object sender, MouseButtonEventArgs e)
    {
        WindowDragBehavior.OnMouseLeftButtonDown(this, e);
    }

    private void OnMouseLeftButtonUp(object sender, MouseButtonEventArgs e)
    {
        WindowDragBehavior.OnMouseLeftButtonUp(this, e);
    }

    private void OnMouseMove(object sender, MouseEventArgs e)
    {
        WindowDragBehavior.OnMouseMove(this, e);

        // 眼球跟随：将鼠标位置转换为归一化坐标 (-1..1)
        if (CharacterDisplay?.IsReady == true)
        {
            var pos = e.GetPosition(CharacterDisplay);
            double cx = CharacterDisplay.Width / 2;
            double cy = CharacterDisplay.Height / 2;
            float nx = (float)((pos.X - cx) / cx);
            float ny = (float)((pos.Y - cy) / cy);
            CharacterDisplay.SetEyeTarget(nx, ny);
        }
    }

    private void OnMouseEnter(object sender, MouseEventArgs e)
    {
        ShowMenu();
    }

    private void OnMouseLeave(object sender, MouseEventArgs e)
    {
        // AutoHideBehavior 处理延迟隐藏
    }

    private void ShowMenu()
    {
        if (RadialMenu.Visibility != Visibility.Visible)
        {
            RadialMenuAnimations.AnimateShow(RadialMenu);
        }
    }

    private void HideMenu()
    {
        if (RadialMenu.Visibility == Visibility.Visible)
        {
            RadialMenuAnimations.AnimateHide(RadialMenu);
        }
    }
}

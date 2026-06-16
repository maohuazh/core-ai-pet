using System.Windows;
using System.Windows.Input;
using Microsoft.Extensions.DependencyInjection;
using CoreAIpet.Core.Interfaces;
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

    // ============================================================
    // 右键菜单事件处理
    // ============================================================

    private void OnOpenChat_Click(object sender, RoutedEventArgs e)
    {
        // TODO: 打开聊天窗口
    }

    private void OnToggleMenu_Click(object sender, RoutedEventArgs e)
    {
        if (RadialMenu.Visibility == Visibility.Visible)
        {
            HideMenu();
        }
        else
        {
            ShowMenu();
        }
    }

    private void OnSettings_Click(object sender, RoutedEventArgs e)
    {
        // TODO: 打开设置窗口
    }

    private void OnDebug_Click(object sender, RoutedEventArgs e)
    {
        // TODO: 打开调试面板
    }

    private void OnExit_Click(object sender, RoutedEventArgs e)
    {
        // 保存窗口位置
        try
        {
            var posService = ((App)App.Current).Host.Services
                .GetRequiredService<IPositionService>();
            posService.SaveAsync(new WindowPosition(Left, Top)).Wait();
        }
        catch { /* 忽略保存失败 */ }

        // 退出应用
        App.Current.Shutdown();
    }
}

using System.Windows;
using System.Windows.Controls;
using System.Windows.Input;
using System.Windows.Interop;
using Microsoft.Extensions.DependencyInjection;
using CoreAIpet.Core.Interfaces;
using CoreAIpet.Desktop.Behaviors;
using CoreAIpet.Desktop.Services.Live2D;

namespace CoreAIpet.Desktop.Views;

/// <summary>
/// 主窗口 — 透明无边框 + 角色显示 + 径向菜单 + 拖拽 + 眼球跟随
/// </summary>
public partial class MainWindow : Window
{
    private readonly AutoHideBehavior? _autoHide;
    private readonly ModelLoaderService? _modelLoader;
    private bool _live2DInitialized;

    public MainWindow()
    {
        InitializeComponent();

        MouseLeftButtonDown += OnMouseLeftButtonDown;
        MouseLeftButtonUp += OnMouseLeftButtonUp;
        MouseMove += OnMouseMove;
        MouseEnter += OnMouseEnter;
        MouseLeave += OnMouseLeave;
        Loaded += OnLoaded;

        _autoHide = new AutoHideBehavior(this, () => HideMenu());

        // 从 DI 容器获取服务
        var app = (App)App.Current;
        _modelLoader = app.Host.Services.GetService<ModelLoaderService>();

        // 动态创建右键菜单（无边框窗口需要在代码中创建）
        CreateContextMenu();
    }

    private void OnLoaded(object sender, RoutedEventArgs e)
    {
        if (_live2DInitialized || _modelLoader == null) return;

        try
        {
            // 获取渲染宿主
            var app = (App)App.Current;
            var renderHost = app.Host.Services.GetService<Live2D.Rendering.Live2DRenderHost>();
            if (renderHost == null)
            {
                System.Diagnostics.Debug.WriteLine("[MainWindow] Live2DRenderHost not available");
                return;
            }

            var width = (int)Live2DDisplay.ActualWidth;
            var height = (int)Live2DDisplay.ActualHeight;
            if (width <= 0) width = 200;
            if (height <= 0) height = 280;

            // 初始化渲染器（offscreen，不需要 HWND）
            var success = renderHost.Initialize(Live2DDisplay, width, height);

            if (!success)
            {
                System.Diagnostics.Debug.WriteLine("[MainWindow] Live2D renderer initialization failed");
                return;
            }

            // 加载模型
            success = _modelLoader.LoadModel();
            if (!success)
            {
                System.Diagnostics.Debug.WriteLine($"[MainWindow] Model loading failed: {_modelLoader.LastError}");
                return;
            }

            // 启动渲染循环
            renderHost.StartRendering();
            _live2DInitialized = true;

            // 隐藏 Canvas 占位符
            CharacterDisplay.Visibility = Visibility.Collapsed;

            System.Diagnostics.Debug.WriteLine("[MainWindow] Live2D initialized successfully");
        }
        catch (Exception ex)
        {
            System.Diagnostics.Debug.WriteLine($"[MainWindow] Live2D initialization error: {ex.Message}");
        }
    }

    private void CreateContextMenu()
    {
        var menu = new ContextMenu();

        var openChat = new MenuItem { Header = "打开聊天" };
        openChat.Click += OnOpenChat_Click;

        var toggleMenu = new MenuItem { Header = "隐藏菜单" };
        toggleMenu.Click += OnToggleMenu_Click;

        var settings = new MenuItem { Header = "设置" };
        settings.Click += OnSettings_Click;

        var debug = new MenuItem { Header = "调试面板" };
        debug.Click += OnDebug_Click;

        var exit = new MenuItem { Header = "退出", FontWeight = FontWeights.Bold };
        exit.Click += OnExit_Click;

        menu.Items.Add(openChat);
        menu.Items.Add(toggleMenu);
        menu.Items.Add(new Separator());
        menu.Items.Add(settings);
        menu.Items.Add(debug);
        menu.Items.Add(new Separator());
        menu.Items.Add(exit);

        ContextMenu = menu;
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

            // 发送到 Canvas 渲染器
            CharacterDisplay.SetEyeTarget(nx, ny);

            // 如果 Live2D 已初始化，也发送到 Live2D 渲染器
            if (_live2DInitialized)
            {
                var app = (App)App.Current;
                var renderHost = app.Host.Services.GetService<Live2D.Rendering.Live2DRenderHost>();
                renderHost?.SetEyeTarget(nx, ny);
            }
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
        // 直接关闭，位置保存由 App.OnExit 处理
        App.Current.Shutdown();
    }
}

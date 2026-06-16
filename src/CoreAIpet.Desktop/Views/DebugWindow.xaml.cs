using System.Windows;
using System.Windows.Threading;
using Microsoft.Extensions.DependencyInjection;
using CoreAIpet.Desktop.Services.Live2D;
using CoreAIpet.Desktop.Live2D.Rendering;

namespace CoreAIpet.Desktop.Views;

public partial class DebugWindow : Window
{
    private readonly DispatcherTimer _updateTimer;
    private readonly ModelLoaderService _modelLoader;
    private readonly Live2DRenderHost _renderHost;

    public DebugWindow()
    {
        InitializeComponent();

        var app = (App)App.Current;
        _modelLoader = app.Host.Services.GetRequiredService<ModelLoaderService>();
        _renderHost = app.Host.Services.GetRequiredService<Live2DRenderHost>();

        // 设置定时器，每秒更新一次
        _updateTimer = new DispatcherTimer
        {
            Interval = TimeSpan.FromSeconds(1)
        };
        _updateTimer.Tick += UpdateTimer_Tick;
        _updateTimer.Start();

        // 初始更新
        UpdateLive2DStatus();
    }

    private void UpdateTimer_Tick(object? sender, EventArgs e)
    {
        UpdateFps();
        UpdatePerformanceMetrics();
        UpdateLive2DStatus();
    }

    private void UpdateFps()
    {
        if (_modelLoader.IsLive2DActive)
        {
            var fps = _renderHost.FPS;
            FpsText.Text = $"FPS: {fps:F1}";
        }
        else
        {
            FpsText.Text = "FPS: -- (Canvas)";
        }
    }

    private void UpdatePerformanceMetrics()
    {
        var process = System.Diagnostics.Process.GetCurrentProcess();
        var cpuTime = process.TotalProcessorTime.TotalMilliseconds;
        var memory = process.WorkingSet64 / 1024.0 / 1024.0; // MB

        CpuText.Text = $"CPU: {cpuTime / 1000:F1}s";
        MemText.Text = $"Memory: {memory:F1} MB";
    }

    private void UpdateLive2DStatus()
    {
        if (_modelLoader.IsLive2DActive)
        {
            RendererText.Text = "渲染器: Live2D Cubism SDK";
            ModelStatusText.Text = "模型: ✓ 已加载";
            ModelErrorText.Text = "";
        }
        else
        {
            RendererText.Text = "渲染器: WpfCanvas (Mock)";
            ModelStatusText.Text = "模型: ⚠ 未加载";
            ModelErrorText.Text = _modelLoader.LastError ?? "使用 Canvas 占位符";
        }
    }

    protected override void OnClosed(EventArgs e)
    {
        _updateTimer.Stop();
        base.OnClosed(e);
    }
}

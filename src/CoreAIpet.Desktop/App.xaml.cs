using System.IO;
using System.Windows;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using CoreAIpet.Core.Interfaces;
using CoreAIpet.Desktop.Views;
using HostingHost = Microsoft.Extensions.Hosting.Host;

namespace CoreAIpet.Desktop;

/// <summary>
/// 应用入口 — Generic Host 构建 + 启动
/// </summary>
public partial class App : Application
{
    private IHost? _host;

    /// <summary>
    /// 公开的 Host 属性，供其他组件访问 DI 容器
    /// </summary>
    public IHost Host => _host!;

    public App()
    {
        var crashPath = Path.Combine(AppContext.BaseDirectory, "crash.log");
        // 全局异常捕获：避免 async void / UI 线程异常被静默吞掉
        AppDomain.CurrentDomain.UnhandledException += (s, e) =>
            File.AppendAllText(crashPath,
                $"[AppDomain] {e.ExceptionObject}{Environment.NewLine}");
        DispatcherUnhandledException += (s, e) =>
        {
            File.AppendAllText(crashPath,
                $"[Dispatcher] {e.Exception}{Environment.NewLine}");
            e.Handled = false;
        };
        TaskScheduler.UnobservedTaskException += (s, e) =>
        {
            File.AppendAllText(crashPath,
                $"[TaskScheduler] {e.Exception}{Environment.NewLine}");
            e.SetObserved();
        };
    }

    protected override async void OnStartup(StartupEventArgs e)
    {
        base.OnStartup(e);
        // 确保工作目录为程序集所在目录（Live2D 着色器从相对路径加载）
        Environment.CurrentDirectory = AppContext.BaseDirectory;

        var crashPath = Path.Combine(AppContext.BaseDirectory, "crash.log");
        try
        {
            // 单实例检测
            if (!SingleInstanceGuard.TryAcquire())
            {
                Shutdown();
                return;
            }

            // 构建 Generic Host
            _host = HostingHost.CreateDefaultBuilder()
                .UseContentRoot(AppContext.BaseDirectory)
                .ConfigureAppConfiguration((ctx, config) =>
                {
                    config.SetBasePath(AppContext.BaseDirectory);
                    config.AddJsonFile("config.json", optional: true, reloadOnChange: true);
                })
                .ConfigureServices((ctx, services) =>
                {
                    CompositionRoot.RegisterServices(services, ctx.Configuration);
                })
                .Build();

            await _host.StartAsync();

            // 获取主窗口并恢复位置
            var mainWindow = _host.Services.GetRequiredService<MainWindow>();
            var posService = _host.Services.GetRequiredService<IPositionService>();
            var pos = await posService.LoadAsync();
            if (pos != null)
            {
                mainWindow.Left = pos.X;
                mainWindow.Top = pos.Y;
            }
            else
            {
                // 默认位置：屏幕左侧偏上，确保可见
                mainWindow.Left = 100;
                mainWindow.Top = 100;
            }

            mainWindow.Show();
        }
        catch (Exception ex)
        {
            File.AppendAllText(crashPath,
                $"[OnStartup] {ex}{Environment.NewLine}");
        }
    }

    protected override async void OnExit(ExitEventArgs e)
    {
        if (_host != null)
        {
            // 保存窗口位置
            try
            {
                var mainWindow = _host.Services.GetRequiredService<MainWindow>();
                var posService = _host.Services.GetRequiredService<IPositionService>();
                await posService.SaveAsync(new WindowPosition(mainWindow.Left, mainWindow.Top));
            }
            catch { /* 忽略保存失败 */ }

            await _host.StopAsync();
            _host.Dispose();
        }

        SingleInstanceGuard.Release();
        base.OnExit(e);
    }
}

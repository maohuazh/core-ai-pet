using System.IO;
using System.Windows;
using System.Windows.Threading;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using CoreAIpet.Core.Interfaces;
using CoreAIpet.Desktop.Views;

namespace CoreAIpet.Desktop;

/// <summary>
/// 应用入口 — Generic Host 构建 + 启动
/// </summary>
public partial class App : Application
{
    private IHost? _host;

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
            _host = Host.CreateDefaultBuilder()
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

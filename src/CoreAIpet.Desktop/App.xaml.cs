using System.Windows;
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

    protected override async void OnStartup(StartupEventArgs e)
    {
        base.OnStartup(e);

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

        mainWindow.Show();
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

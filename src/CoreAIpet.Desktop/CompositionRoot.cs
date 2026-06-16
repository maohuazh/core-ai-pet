using System.IO;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using CoreAIpet.Core.Interfaces;
using CoreAIpet.Desktop.Services.Configuration;
using CoreAIpet.Desktop.Services.Events;
using CoreAIpet.Desktop.Services.Diagnostics;
using CoreAIpet.Desktop.Live2D.Bridge;
using CoreAIpet.Desktop.Live2D.Rendering;
using CoreAIpet.Desktop.Services.Character;
using CoreAIpet.Desktop.Services.Live2D;

namespace CoreAIpet.Desktop;

/// <summary>
/// DI 注册组合根 — 所有服务的注册入口
/// </summary>
public static class CompositionRoot
{
    public static void RegisterServices(IServiceCollection services, IConfiguration configuration)
    {
        // === Configuration ===
        var configPath = Path.Combine(
            Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData),
            "CoreAIpet", "config.json");
        var configStore = new JsonConfigStore(configPath);
        services.AddSingleton(configStore);

        var configService = new ConfigService(configStore);
        services.AddSingleton(configService);
        services.AddSingleton<IConfigService>(configService);

        services.AddSingleton<IPositionService>(sp =>
            new PositionService(sp.GetRequiredService<ConfigService>()));

        // === Events & Logging ===
        services.AddSingleton<IEventBus, EventBus>();
        services.AddSingleton<ILogService>(sp =>
            new LogService(sp.GetRequiredService<Microsoft.Extensions.Logging.ILogger<LogService>>()));

        // === Window Services (Group 2) ===
        // services.AddSingleton<IWindowService, WindowService>();

        // === AI Services (Group 7) ===
        // services.AddAIServices(configuration);

        // === Live2D (Group 4) ===
        services.AddSingleton<Live2DBridgeWrapper>();
        services.AddSingleton<Live2DRenderHost>();
        services.AddSingleton<ModelLoaderService>();

        // === Character (Group 4) ===
        services.AddSingleton<ICharacterController, CharacterController>();

        // === Plugins (Group 8) ===
        // services.AddSingleton<PluginManager>();
        // services.AddHostedService<PluginHostService>();

        // === UI ===
        services.AddSingleton<Views.MainWindow>();
        // services.AddTransient<MainViewModel>();
        // services.AddTransient<ChatViewModel>();
        // services.AddTransient<SettingsViewModel>();
        // services.AddTransient<DebugViewModel>();
        // services.AddTransient<RadialMenuViewModel>();
    }
}

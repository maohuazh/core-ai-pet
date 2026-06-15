using Microsoft.Extensions.Hosting;
using CoreAIpet.Core.Interfaces;

namespace CoreAIpet.Desktop.Services.Plugins;

/// <summary>
/// 插件宿主服务 — IHostedService 编排插件生命周期
/// </summary>
public class PluginHostService : IHostedService
{
    private readonly PluginManager _pluginManager;
    private readonly PluginContext _pluginContext;
    private readonly ILogService _logger;

    public PluginHostService(PluginManager pluginManager, PluginContext pluginContext, ILogService logger)
    {
        _pluginManager = pluginManager;
        _pluginContext = pluginContext;
        _logger = logger;
    }

    public async Task StartAsync(CancellationToken cancellationToken)
    {
        _logger.Information("Plugin system starting...");

        _pluginManager.DiscoverPlugins();
        await _pluginManager.LoadAllAsync(_pluginContext);
        await _pluginManager.ActivateAllAsync();

        _logger.Information($"Plugin system started. {_pluginManager.Plugins.Count} plugins loaded.");
    }

    public async Task StopAsync(CancellationToken cancellationToken)
    {
        _logger.Information("Plugin system stopping...");

        await _pluginManager.DeactivateAllAsync();
        await _pluginManager.UnloadAllAsync();

        _logger.Information("Plugin system stopped.");
    }
}

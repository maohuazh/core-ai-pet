using System.IO;
using CoreAIpet.Core.Interfaces;
using CoreAIpet.Core.Models.Plugin;

namespace CoreAIpet.Desktop.Services.Plugins;

/// <summary>
/// 插件上下文 — 注入给插件的宿主服务
/// </summary>
public class PluginContext : IPluginContext, IPluginHost
{
    private readonly IConfigService _config;
    private readonly ILogService _logger;
    private readonly IEventBus _eventBus;
    private readonly IAIServiceProvider _ai;
    private readonly List<PluginMenuItem> _menuItems = new();

    public PluginContext(IConfigService config, ILogService logger, IEventBus eventBus, IAIServiceProvider ai)
    {
        _config = config;
        _logger = logger;
        _eventBus = eventBus;
        _ai = ai;
    }

    // IPluginContext
    public IConfigService Config => _config;
    public ILogService Logger => _logger;
    public IEventBus EventBus => _eventBus;
    public IAIServiceProvider AI => _ai;
    public string DataDirectory { get; set; } = string.Empty;
    public string ConfigSection { get; set; } = string.Empty;

    // IPluginHost
    public IReadOnlyList<PluginMenuItem> RegisteredMenuItems => _menuItems.AsReadOnly();

    public void RegisterMenuItem(PluginMenuItem menuItem)
    {
        _menuItems.Add(menuItem);
    }

    public void UnregisterMenuItem(string menuItemId)
    {
        _menuItems.RemoveAll(m => m.Id == menuItemId);
    }

    public IConfigService GetConfigService() => _config;
    public ILogService GetLogService() => _logger;
    public IEventBus GetEventBus() => _eventBus;
    public IAIServiceProvider GetAIService() => _ai;

    public string GetPluginDataDirectory(string pluginId)
    {
        var dir = Path.Combine(
            Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData),
            "CoreAIpet", "plugins", pluginId);
        if (!Directory.Exists(dir))
            Directory.CreateDirectory(dir);
        return dir;
    }
}

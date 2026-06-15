using CoreAIpet.Core.Models.Plugin;

namespace CoreAIpet.Core.Interfaces;

/// <summary>
/// 插件宿主服务 — 提供给插件的宿主能力
/// 插件通过此接口与宿主交互, 注册菜单项, 获取配置等
/// </summary>
public interface IPluginHost
{
    /// <summary>注册菜单项 (插件在 Load 阶段调用)</summary>
    void RegisterMenuItem(PluginMenuItem menuItem);

    /// <summary>注销菜单项</summary>
    void UnregisterMenuItem(string menuItemId);

    /// <summary>获取宿主配置服务</summary>
    IConfigService GetConfigService();

    /// <summary>获取宿主日志服务</summary>
    ILogService GetLogService();

    /// <summary>获取宿主事件总线</summary>
    IEventBus GetEventBus();

    /// <summary>获取宿主 AI 服务</summary>
    IAIServiceProvider GetAIService();

    /// <summary>获取插件专属数据目录 (不存在则自动创建)</summary>
    string GetPluginDataDirectory(string pluginId);
}

namespace CoreAIpet.Core.Interfaces;

/// <summary>
/// 注入给插件的服务上下文 — 插件通过这些接口与宿主交互
/// </summary>
public interface IPluginContext
{
    /// <summary>配置服务 — 读写插件自身配置</summary>
    IConfigService Config { get; }

    /// <summary>日志服务</summary>
    ILogService Logger { get; }

    /// <summary>事件总线 — 发布/订阅事件</summary>
    IEventBus EventBus { get; }

    /// <summary>AI 服务 — 插件可以调用 AI</summary>
    IAIServiceProvider AI { get; }

    /// <summary>插件专属持久化目录</summary>
    string DataDirectory { get; }

    /// <summary>插件自身配置节名称</summary>
    string ConfigSection { get; }
}

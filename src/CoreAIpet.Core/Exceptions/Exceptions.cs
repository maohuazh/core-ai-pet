namespace CoreAIpet.Core.Exceptions;

/// <summary>
/// 插件加载异常
/// </summary>
public class PluginLoadException : Exception
{
    public string PluginId { get; }

    public PluginLoadException(string pluginId, string message)
        : base(message)
    {
        PluginId = pluginId;
    }

    public PluginLoadException(string pluginId, string message, Exception innerException)
        : base(message, innerException)
    {
        PluginId = pluginId;
    }
}

/// <summary>
/// AI 服务异常
/// </summary>
public class AIServiceException : Exception
{
    public string Provider { get; }

    public AIServiceException(string provider, string message)
        : base(message)
    {
        Provider = provider;
    }

    public AIServiceException(string provider, string message, Exception innerException)
        : base(message, innerException)
    {
        Provider = provider;
    }
}

/// <summary>
/// Live2D Bridge 异常
/// </summary>
public class Live2DBridgeException : Exception
{
    public Live2DBridgeException(string message) : base(message) { }

    public Live2DBridgeException(string message, Exception innerException)
        : base(message, innerException) { }
}

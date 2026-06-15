using CoreAIpet.Core.Models.Settings;

namespace CoreAIpet.Core.Interfaces;

/// <summary>
/// 配置服务 — 管理应用配置的读写、持久化、变更通知
/// </summary>
public interface IConfigService
{
    /// <summary>获取指定配置节 (强类型)</summary>
    T Get<T>(string section) where T : class, new();

    /// <summary>设置指定配置节</summary>
    void Set<T>(string section, T value) where T : class;

    /// <summary>持久化到 JSON 文件 (原子写入)</summary>
    Task SaveAsync();

    /// <summary>重新从文件加载配置</summary>
    Task ReloadAsync();

    /// <summary>配置变更事件</summary>
    event EventHandler<SettingsChangedEventArgs>? SettingsChanged;

    /// <summary>完整配置根对象</summary>
    AppSettings AppSettings { get; }

    /// <summary>外观设置</summary>
    AppearanceSettings Appearance { get; }

    /// <summary>系统设置</summary>
    SystemSettings System { get; }

    /// <summary>AI 设置</summary>
    AISettings AISettings { get; }

    /// <summary>调试设置</summary>
    DebugSettings Debug { get; }
}

/// <summary>
/// 设置变更事件参数
/// </summary>
public class SettingsChangedEventArgs : EventArgs
{
    public string Section { get; init; } = string.Empty;
}

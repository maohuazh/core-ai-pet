using System.Text.Json;
using CoreAIpet.Core.Interfaces;
using CoreAIpet.Core.Models.Settings;

namespace CoreAIpet.Desktop.Services.Configuration;

/// <summary>
/// 配置服务 — 强类型配置节读写、变更通知、JSON 持久化
/// </summary>
public class ConfigService : IConfigService
{
    private readonly JsonConfigStore _store;
    private AppSettings _settings = new();

    public event EventHandler<SettingsChangedEventArgs>? SettingsChanged;

    public AppSettings AppSettings => _settings;
    public AppearanceSettings Appearance => _settings.Appearance;
    public SystemSettings System => _settings.System;
    public AISettings AISettings => _settings.AI;
    public DebugSettings Debug => _settings.Debug;

    public ConfigService(JsonConfigStore store)
    {
        _store = store;
    }

    public async Task InitializeAsync()
    {
        _settings = await _store.LoadAsync<AppSettings>();
    }

    public T Get<T>(string section) where T : class, new()
    {
        return section.ToLowerInvariant() switch
        {
            "appearance" => (_settings.Appearance as T) ?? new T(),
            "system" => (_settings.System as T) ?? new T(),
            "ai" => (_settings.AI as T) ?? new T(),
            "debug" => (_settings.Debug as T) ?? new T(),
            "position" => (_settings.Position as T) ?? new T(),
            "plugins" => (_settings.Plugins as T) ?? new T(),
            _ => new T()
        };
    }

    public void Set<T>(string section, T value) where T : class
    {
        switch (section.ToLowerInvariant())
        {
            case "appearance": _settings.Appearance = value as AppearanceSettings ?? _settings.Appearance; break;
            case "system": _settings.System = value as SystemSettings ?? _settings.System; break;
            case "ai": _settings.AI = value as AISettings ?? _settings.AI; break;
            case "debug": _settings.Debug = value as DebugSettings ?? _settings.Debug; break;
            case "position": _settings.Position = value as PositionConfig ?? _settings.Position; break;
            case "plugins": _settings.Plugins = value as PluginsConfig ?? _settings.Plugins; break;
        }
        SettingsChanged?.Invoke(this, new SettingsChangedEventArgs { Section = section });
    }

    public async Task SaveAsync()
    {
        await _store.SaveAsync(_settings);
    }

    public async Task ReloadAsync()
    {
        _settings = await _store.LoadAsync<AppSettings>();
        SettingsChanged?.Invoke(this, new SettingsChangedEventArgs { Section = "*" });
    }
}

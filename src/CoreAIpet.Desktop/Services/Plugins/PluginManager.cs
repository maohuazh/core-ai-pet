using System.IO;
using System.Reflection;
using System.Text.Json;
using CoreAIpet.Core.Exceptions;
using CoreAIpet.Core.Interfaces;
using CoreAIpet.Core.Models.Plugin;

namespace CoreAIpet.Desktop.Services.Plugins;

/// <summary>
/// 插件管理器 — 扫描/发现/加载/卸载插件
/// </summary>
public class PluginManager
{
    private readonly List<LoadedPlugin> _plugins = new();
    private readonly ILogService _logger;
    private readonly string _pluginsDirectory;

    public IReadOnlyList<LoadedPlugin> Plugins => _plugins.AsReadOnly();

    public event Action? PluginsChanged;

    public PluginManager(ILogService logger)
    {
        _logger = logger;
        _pluginsDirectory = Path.Combine(
            AppContext.BaseDirectory, "..", "..", "..", "..", "plugins");
        if (!Directory.Exists(_pluginsDirectory))
        {
            _pluginsDirectory = Path.Combine(AppContext.BaseDirectory, "plugins");
        }
    }

    public void DiscoverPlugins()
    {
        if (!Directory.Exists(_pluginsDirectory))
        {
            _logger.Information($"Plugins directory not found: {_pluginsDirectory}");
            return;
        }

        foreach (var dir in Directory.GetDirectories(_pluginsDirectory))
        {
            try
            {
                var manifestPath = Path.Combine(dir, "plugin.json");
                if (!File.Exists(manifestPath)) continue;

                var json = File.ReadAllText(manifestPath);
                var manifest = JsonSerializer.Deserialize<PluginManifest>(json);
                if (manifest == null || string.IsNullOrEmpty(manifest.EntryPoint)) continue;

                var dllPath = Path.Combine(dir, manifest.EntryPoint);
                if (!File.Exists(dllPath))
                {
                    _logger.Warning($"Plugin DLL not found: {dllPath}");
                    continue;
                }

                var context = new PluginLoadContext(dllPath);
                var assembly = context.LoadFromAssemblyPath(dllPath);
                var type = assembly.GetType(manifest.ClassName);

                if (type == null || !typeof(IPlugin).IsAssignableFrom(type))
                {
                    _logger.Error($"Plugin class not found or invalid: {manifest.ClassName} in {dir}");
                    continue;
                }

                var instance = (IPlugin)Activator.CreateInstance(type)!;
                _plugins.Add(new LoadedPlugin
                {
                    Plugin = instance,
                    Manifest = manifest,
                    Directory = dir,
                    LoadContext = context
                });

                _logger.Information($"Discovered plugin: {manifest.Name} ({manifest.Id})");
            }
            catch (Exception ex)
            {
                _logger.Error($"Failed to discover plugin in {dir}", ex);
            }
        }
    }

    public async Task LoadAllAsync(PluginContext context)
    {
        foreach (var loaded in _plugins)
        {
            try
            {
                loaded.CurrentState = PluginState.Loading;
                context.DataDirectory = context.GetPluginDataDirectory(loaded.Plugin.Id);
                context.ConfigSection = loaded.Plugin.Id;
                await loaded.Plugin.LoadAsync(context);
                loaded.CurrentState = PluginState.Loaded;
                _logger.Information($"Plugin loaded: {loaded.Plugin.Name}");
            }
            catch (Exception ex)
            {
                loaded.CurrentState = PluginState.Error;
                _logger.Error($"Failed to load plugin: {loaded.Plugin.Name}", ex);
            }
        }
    }

    public async Task ActivateAllAsync()
    {
        foreach (var loaded in _plugins.Where(p => p.CurrentState == PluginState.Loaded))
        {
            try
            {
                loaded.CurrentState = PluginState.Activating;
                await loaded.Plugin.ActivateAsync();
                loaded.CurrentState = PluginState.Active;
            }
            catch (Exception ex)
            {
                loaded.CurrentState = PluginState.Error;
                _logger.Error($"Failed to activate plugin: {loaded.Plugin.Name}", ex);
            }
        }
        PluginsChanged?.Invoke();
    }

    public async Task DeactivateAllAsync()
    {
        foreach (var loaded in _plugins.Where(p => p.CurrentState == PluginState.Active))
        {
            try
            {
                loaded.CurrentState = PluginState.Deactivating;
                await loaded.Plugin.DeactivateAsync();
                loaded.CurrentState = PluginState.Deactivated;
            }
            catch (Exception ex)
            {
                _logger.Error($"Failed to deactivate plugin: {loaded.Plugin.Name}", ex);
            }
        }
    }

    public async Task UnloadAllAsync()
    {
        foreach (var loaded in _plugins)
        {
            try
            {
                loaded.CurrentState = PluginState.Unloading;
                await loaded.Plugin.UnloadAsync();
                loaded.Plugin.Dispose();
                loaded.LoadContext.Unload();
            }
            catch (Exception ex)
            {
                _logger.Error($"Failed to unload plugin: {loaded.Plugin.Name}", ex);
            }
        }
        _plugins.Clear();
    }
}

public class LoadedPlugin
{
    public IPlugin Plugin { get; set; } = null!;
    public PluginManifest Manifest { get; set; } = null!;
    public string Directory { get; set; } = string.Empty;
    public PluginLoadContext LoadContext { get; set; } = null!;
    public PluginState CurrentState { get; set; } = PluginState.Unloaded;
}

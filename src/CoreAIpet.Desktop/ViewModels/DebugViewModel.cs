using System.Collections.ObjectModel;
using CommunityToolkit.Mvvm.ComponentModel;
using CoreAIpet.Core.Interfaces;
using CoreAIpet.Desktop.Services.Diagnostics;

namespace CoreAIpet.Desktop.ViewModels;

/// <summary>
/// 调试 ViewModel — 性能指标、插件状态、日志
/// </summary>
public partial class DebugViewModel : ObservableObject
{
    private readonly PerformanceMonitor _monitor;
    private readonly ILogService _logService;
    private readonly Services.Plugins.PluginManager _pluginManager;

    [ObservableProperty] private string _fpsText = "FPS: --";
    [ObservableProperty] private string _cpuText = "CPU: --";
    [ObservableProperty] private string _memText = "Memory: --";
    [ObservableProperty] private ObservableCollection<string> _logEntries = new();
    [ObservableProperty] private ObservableCollection<string> _pluginStatuses = new();

    public DebugViewModel(PerformanceMonitor monitor, ILogService logService, Services.Plugins.PluginManager pluginManager)
    {
        _monitor = monitor;
        _logService = logService;
        _pluginManager = pluginManager;

        _logService.LogAdded += (s, entry) =>
        {
            LogEntries.Add($"[{entry.Timestamp:HH:mm:ss}] [{entry.Level}] {entry.Message}");
            if (LogEntries.Count > 200) LogEntries.RemoveAt(0);
        };
    }

    public void RefreshMetrics()
    {
        _monitor.Refresh();
        FpsText = $"FPS: {_monitor.FPS:F1}";
        CpuText = $"CPU: {_monitor.CpuPercent:F1}%";
        MemText = $"Memory: {_monitor.MemoryMB} MB";
    }

    public void RefreshPlugins()
    {
        PluginStatuses.Clear();
        foreach (var p in _pluginManager.Plugins)
        {
            PluginStatuses.Add($"{p.Manifest.Name} [{p.CurrentState}]");
        }
    }
}

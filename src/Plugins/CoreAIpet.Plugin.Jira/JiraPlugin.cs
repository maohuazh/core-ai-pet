using CoreAIpet.Core.Interfaces;
using CoreAIpet.Core.Models.Plugin;

namespace CoreAIpet.Plugin.Jira;

/// <summary>
/// Jira 插件 — 任务管理 + @通知
/// </summary>
public class JiraPlugin : IPlugin
{
    public string Id => "coreai.jira";
    public string Name => "Jira";
    public string Version => "1.0.0";
    public PluginState State { get; private set; } = PluginState.Unloaded;

    private IPluginContext? _context;

    public Task LoadAsync(IPluginContext context)
    {
        _context = context;
        context.Logger.Information("Jira plugin loaded");
        return Task.CompletedTask;
    }

    public Task ActivateAsync() { State = PluginState.Active; return Task.CompletedTask; }
    public Task ExecuteAsync(CancellationToken ct) => Task.CompletedTask;
    public Task DeactivateAsync() { State = PluginState.Deactivated; return Task.CompletedTask; }
    public Task UnloadAsync() { State = PluginState.Unloaded; return Task.CompletedTask; }

    public IReadOnlyList<PluginMenuItem> GetMenuItems() => new List<PluginMenuItem>
    {
        new() { Id = "jira.tasks", Label = "Jira", Icon = "assets/jira.png", Tooltip = "Jira 任务", Action = "open_jira" }
    };

    public Task HandleMenuActionAsync(string actionId)
    {
        _context?.Logger.Information($"Jira action: {actionId}");
        return Task.CompletedTask;
    }

    public void Dispose() { }
}

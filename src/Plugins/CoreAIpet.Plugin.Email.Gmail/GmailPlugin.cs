using CoreAIpet.Core.Interfaces;
using CoreAIpet.Core.Models.Plugin;

namespace CoreAIpet.Plugin.Email.Gmail;

public class GmailPlugin : IPlugin, IEmailProvider
{
    public string Id => "provider.email.gmail";
    public string Name => "Gmail";
    public string Version => "1.0.0";
    public string Icon => "assets/gmail.png";
    public PluginState State { get; private set; } = PluginState.Unloaded;
    public ProviderConnectionState ConnectionState { get; private set; } = ProviderConnectionState.Disconnected;
    public event EventHandler<EmailItem>? EmailReceived;
    public event EventHandler<ProviderConnectionState>? ConnectionStateChanged;

    public Task LoadAsync(IPluginContext context) => Task.CompletedTask;
    public Task ActivateAsync() { ConnectionState = ProviderConnectionState.Connected; ConnectionStateChanged?.Invoke(this, ConnectionState); return Task.CompletedTask; }
    public Task ExecuteAsync(CancellationToken ct) => Task.CompletedTask;
    public Task DeactivateAsync() { ConnectionState = ProviderConnectionState.Disconnected; return Task.CompletedTask; }
    public Task UnloadAsync() => Task.CompletedTask;
    public Task<int> GetUnreadCountAsync(CancellationToken ct = default) => Task.FromResult(0);
    public Task<IReadOnlyList<EmailItem>> GetRecentEmailsAsync(int count = 10, CancellationToken ct = default) => Task.FromResult<IReadOnlyList<EmailItem>>(Array.Empty<EmailItem>());
    public Task ConnectAsync(CancellationToken ct = default) => ActivateAsync();
    public Task DisconnectAsync(CancellationToken ct = default) => DeactivateAsync();
    public IReadOnlyList<PluginMenuItem> GetMenuItems() => Array.Empty<PluginMenuItem>();
    public Task HandleMenuActionAsync(string actionId) => Task.CompletedTask;
    public void Dispose() { }
}

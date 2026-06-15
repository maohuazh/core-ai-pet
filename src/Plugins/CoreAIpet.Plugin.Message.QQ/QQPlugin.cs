using CoreAIpet.Core.Interfaces;
using CoreAIpet.Core.Models.Plugin;

namespace CoreAIpet.Plugin.Message.QQ;

public class QQPlugin : IPlugin, IMessageProvider
{
    public string Id => "provider.message.qq";
    public string Name => "QQ";
    public string Version => "1.0.0";
    public string Icon => "assets/qq.png";
    public PluginState State { get; private set; } = PluginState.Unloaded;
    public ProviderConnectionState ConnectionState { get; private set; } = ProviderConnectionState.Disconnected;
    public event EventHandler<MessageItem>? MessageReceived;
    public event EventHandler<ProviderConnectionState>? ConnectionStateChanged;

    public Task LoadAsync(IPluginContext context) => Task.CompletedTask;
    public Task ActivateAsync() { ConnectionState = ProviderConnectionState.Connected; ConnectionStateChanged?.Invoke(this, ConnectionState); return Task.CompletedTask; }
    public Task ExecuteAsync(CancellationToken ct) => Task.CompletedTask;
    public Task DeactivateAsync() { ConnectionState = ProviderConnectionState.Disconnected; return Task.CompletedTask; }
    public Task UnloadAsync() => Task.CompletedTask;
    public Task<int> GetUnreadCountAsync(CancellationToken ct = default) => Task.FromResult(0);
    public Task<IReadOnlyList<MessageItem>> GetRecentMessagesAsync(int count = 10, CancellationToken ct = default) => Task.FromResult<IReadOnlyList<MessageItem>>(Array.Empty<MessageItem>());
    public Task ConnectAsync(CancellationToken ct = default) => ActivateAsync();
    public Task DisconnectAsync(CancellationToken ct = default) => DeactivateAsync();
    public IReadOnlyList<PluginMenuItem> GetMenuItems() => Array.Empty<PluginMenuItem>();
    public Task HandleMenuActionAsync(string actionId) => Task.CompletedTask;
    public void Dispose() { }
}

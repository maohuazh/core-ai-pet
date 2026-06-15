using CoreAIpet.Core.Models.Plugin;

namespace CoreAIpet.Core.Interfaces;

/// <summary>
/// 消息源提供者接口 — 每个消息平台 (Slack/钉钉/飞书/Teams/QQ/微信 等) 实现此接口
/// 作为独立插件加载, 可独立安装/卸载/更新
/// </summary>
public interface IMessageProvider : IDisposable
{
    /// <summary>提供者唯一标识 (如 "provider.slack", "provider.dingtalk")</summary>
    string Id { get; }

    /// <summary>显示名称 (如 "Slack", "钉钉")</summary>
    string Name { get; }

    /// <summary>图标路径 (相对插件目录)</summary>
    string Icon { get; }

    /// <summary>当前连接状态</summary>
    ProviderConnectionState ConnectionState { get; }

    /// <summary>获取未读消息数量</summary>
    Task<int> GetUnreadCountAsync(CancellationToken ct = default);

    /// <summary>获取最新消息列表</summary>
    Task<IReadOnlyList<MessageItem>> GetRecentMessagesAsync(int count = 10, CancellationToken ct = default);

    /// <summary>连接/登录</summary>
    Task ConnectAsync(CancellationToken ct = default);

    /// <summary>断开连接</summary>
    Task DisconnectAsync(CancellationToken ct = default);

    /// <summary>新消息到达事件</summary>
    event EventHandler<MessageItem>? MessageReceived;

    /// <summary>连接状态变更事件</summary>
    event EventHandler<ProviderConnectionState>? ConnectionStateChanged;
}

/// <summary>
/// 消息条目
/// </summary>
public class MessageItem
{
    public string Id { get; set; } = string.Empty;
    public string ProviderId { get; set; } = string.Empty;
    public string Sender { get; set; } = string.Empty;
    public string SenderAvatar { get; set; } = string.Empty;
    public string Channel { get; set; } = string.Empty;
    public string Content { get; set; } = string.Empty;
    public DateTimeOffset Timestamp { get; set; }
    public bool IsRead { get; set; }
    public string? DeepLink { get; set; }
}

/// <summary>
/// 提供者连接状态
/// </summary>
public enum ProviderConnectionState
{
    Disconnected,
    Connecting,
    Connected,
    Error
}

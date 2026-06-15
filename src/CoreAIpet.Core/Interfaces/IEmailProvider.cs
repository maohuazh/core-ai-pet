namespace CoreAIpet.Core.Interfaces;

/// <summary>
/// 邮箱提供者接口 — 每个邮箱服务 (Outlook/Gmail/Exchange/IMAP 等) 实现此接口
/// 作为独立插件加载, 可独立安装/卸载/更新
/// </summary>
public interface IEmailProvider : IDisposable
{
    /// <summary>提供者唯一标识 (如 "provider.outlook", "provider.gmail")</summary>
    string Id { get; }

    /// <summary>显示名称 (如 "Outlook", "Gmail")</summary>
    string Name { get; }

    /// <summary>图标路径 (相对插件目录)</summary>
    string Icon { get; }

    /// <summary>当前连接状态</summary>
    ProviderConnectionState ConnectionState { get; }

    /// <summary>获取未读邮件数量</summary>
    Task<int> GetUnreadCountAsync(CancellationToken ct = default);

    /// <summary>获取最新邮件列表</summary>
    Task<IReadOnlyList<EmailItem>> GetRecentEmailsAsync(int count = 10, CancellationToken ct = default);

    /// <summary>连接/登录</summary>
    Task ConnectAsync(CancellationToken ct = default);

    /// <summary>断开连接</summary>
    Task DisconnectAsync(CancellationToken ct = default);

    /// <summary>新邮件到达事件</summary>
    event EventHandler<EmailItem>? EmailReceived;

    /// <summary>连接状态变更事件</summary>
    event EventHandler<ProviderConnectionState>? ConnectionStateChanged;
}

/// <summary>
/// 邮件条目
/// </summary>
public class EmailItem
{
    public string Id { get; set; } = string.Empty;
    public string ProviderId { get; set; } = string.Empty;
    public string From { get; set; } = string.Empty;
    public string FromAvatar { get; set; } = string.Empty;
    public string Subject { get; set; } = string.Empty;
    public string Preview { get; set; } = string.Empty;
    public DateTimeOffset Timestamp { get; set; }
    public bool IsRead { get; set; }
    public bool IsImportant { get; set; }
    public string? DeepLink { get; set; }
}

using CoreAIpet.Core.Models.Chat;

namespace CoreAIpet.Core.Interfaces;

/// <summary>
/// AI 服务提供者 — 策略模式入口, 管理多个后端并路由请求
/// </summary>
public interface IAIServiceProvider
{
    /// <summary>当前活跃的后端</summary>
    IAIService Current { get; }

    /// <summary>当前活跃的后端类型</summary>
    AIProvider ActiveProvider { get; }

    /// <summary>切换到指定后端</summary>
    void SwitchProvider(AIProvider provider);

    /// <summary>获取所有已注册的后端</summary>
    IReadOnlyList<IAIService> GetAllProviders();

    /// <summary>使用当前后端发送消息</summary>
    Task<ChatResponse> SendMessageAsync(ChatRequest request, CancellationToken ct = default);

    /// <summary>使用当前后端流式发送</summary>
    IAsyncEnumerable<string> SendMessageStreamAsync(ChatRequest request, CancellationToken ct = default);
}

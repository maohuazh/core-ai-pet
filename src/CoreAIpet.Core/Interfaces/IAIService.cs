using CoreAIpet.Core.Models.Chat;

namespace CoreAIpet.Core.Interfaces;

/// <summary>
/// AI 后端类型枚举
/// </summary>
public enum AIProvider
{
    OpenAI,
    AzureOpenAI,
    Ollama
}

/// <summary>
/// 单一 AI 后端接口 — 每个后端 (OpenAI/Azure/Ollama) 实现此接口
/// </summary>
public interface IAIService
{
    /// <summary>后端类型标识</summary>
    AIProvider Provider { get; }

    /// <summary>是否已配置且可用</summary>
    bool IsConfigured { get; }

    /// <summary>发送消息 (非流式)</summary>
    Task<ChatResponse> SendMessageAsync(ChatRequest request, CancellationToken ct = default);

    /// <summary>发送消息 (流式, 逐 token 返回)</summary>
    IAsyncEnumerable<string> SendMessageStreamAsync(ChatRequest request, CancellationToken ct = default);

    /// <summary>测试连接是否正常</summary>
    Task<bool> TestConnectionAsync(CancellationToken ct = default);
}

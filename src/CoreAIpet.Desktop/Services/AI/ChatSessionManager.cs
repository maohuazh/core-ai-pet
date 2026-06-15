using CoreAIpet.Core.Models.Chat;

namespace CoreAIpet.Desktop.Services.AI;

/// <summary>
/// 对话上下文管理 — 维护最近 N 条消息历史
/// </summary>
public class ChatSessionManager
{
    private readonly List<ChatMessage> _history = new();
    private const int MaxHistoryMessages = 20;
    private const int MaxContentLength = 4000;

    public IReadOnlyList<ChatMessage> History => _history.AsReadOnly();

    public void AddUserMessage(string content)
    {
        if (content.Length > MaxContentLength)
            content = content[..MaxContentLength];

        _history.Add(new ChatMessage
        {
            Role = MessageRole.User,
            Content = content,
            Timestamp = DateTimeOffset.UtcNow
        });
        TrimHistory();
    }

    public void AddAssistantMessage(string content)
    {
        _history.Add(new ChatMessage
        {
            Role = MessageRole.Assistant,
            Content = content,
            Timestamp = DateTimeOffset.UtcNow
        });
        TrimHistory();
    }

    public ChatRequest BuildRequest(string? systemPrompt = null)
    {
        var messages = new List<ChatMessage>();
        if (!string.IsNullOrEmpty(systemPrompt))
        {
            messages.Add(new ChatMessage { Role = MessageRole.System, Content = systemPrompt });
        }
        messages.AddRange(_history);
        return new ChatRequest { Messages = messages };
    }

    public void Clear()
    {
        _history.Clear();
    }

    private void TrimHistory()
    {
        while (_history.Count > MaxHistoryMessages)
        {
            _history.RemoveAt(0);
        }
    }
}

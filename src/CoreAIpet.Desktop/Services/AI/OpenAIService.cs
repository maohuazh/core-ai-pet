using CoreAIpet.Core.Interfaces;
using CoreAIpet.Core.Models.Chat;
using OpenAI;
using OpenAI.Chat;
using CoreChatMessage = CoreAIpet.Core.Models.Chat.ChatMessage;
using OpenAIChatMessage = OpenAI.Chat.ChatMessage;

namespace CoreAIpet.Desktop.Services.AI;

/// <summary>
/// OpenAI Chat Completions API 实现
/// </summary>
public class OpenAIService : IAIService
{
    private readonly CoreAIpet.Core.Models.Settings.OpenAIConfig _config;

    public AIProvider Provider => AIProvider.OpenAI;
    public bool IsConfigured => !string.IsNullOrEmpty(_config.ApiKey);

    public OpenAIService(CoreAIpet.Core.Models.Settings.OpenAIConfig config)
    {
        _config = config;
    }

    public async Task<ChatResponse> SendMessageAsync(ChatRequest request, CancellationToken ct = default)
    {
        if (!IsConfigured) return new ChatResponse { IsSuccess = false, ErrorMessage = "OpenAI API Key not configured" };

        try
        {
            var client = new OpenAIClient(_config.ApiKey);
            var chatClient = client.GetChatClient(_config.Model);

            var messages = ConvertMessages(request.Messages);
            var completion = await chatClient.CompleteChatAsync(messages, cancellationToken: ct);
            var content = completion.Value.Content[0].Text;

            return new ChatResponse
            {
                Content = content,
                Model = _config.Model,
                IsSuccess = true,
                TokensUsed = completion.Value.Usage?.TotalTokenCount ?? 0
            };
        }
        catch (Exception ex)
        {
            return new ChatResponse { IsSuccess = false, ErrorMessage = ex.Message };
        }
    }

    public async IAsyncEnumerable<string> SendMessageStreamAsync(ChatRequest request,
        [System.Runtime.CompilerServices.EnumeratorCancellation] CancellationToken ct = default)
    {
        if (!IsConfigured) yield break;

        var client = new OpenAIClient(_config.ApiKey);
        var chatClient = client.GetChatClient(_config.Model);

        var messages = ConvertMessages(request.Messages);
        var updates = chatClient.CompleteChatStreamingAsync(messages, cancellationToken: ct);

        await foreach (var update in updates)
        {
            if (update.ContentUpdate.Count > 0)
            {
                yield return update.ContentUpdate[0].Text;
            }
        }
    }

    public async Task<bool> TestConnectionAsync(CancellationToken ct = default)
    {
        var testRequest = new ChatRequest
        {
            Messages = new List<CoreChatMessage> { new() { Role = MessageRole.User, Content = "Hi" } },
            MaxTokens = 5
        };
        var response = await SendMessageAsync(testRequest, ct);
        return response.IsSuccess;
    }

    private static List<OpenAIChatMessage> ConvertMessages(IList<CoreChatMessage> messages)
    {
        return messages.Select<CoreChatMessage, OpenAIChatMessage>(m =>
            m.Role switch
            {
                MessageRole.System => OpenAIChatMessage.CreateSystemMessage(m.Content),
                MessageRole.User => OpenAIChatMessage.CreateUserMessage(m.Content),
                MessageRole.Assistant => OpenAIChatMessage.CreateAssistantMessage(m.Content),
                _ => throw new ArgumentException($"Unknown role: {m.Role}")
            }).ToList();
    }
}

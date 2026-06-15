using CoreAIpet.Core.Interfaces;
using CoreAIpet.Core.Models.Chat;
using System.Net.Http;
using System.Net.Http.Json;
using System.Text;
using System.Text.Json;

namespace CoreAIpet.Desktop.Services.AI;

/// <summary>
/// Azure OpenAI 实现
/// </summary>
public class AzureOpenAIService : IAIService
{
    private readonly CoreAIpet.Core.Models.Settings.AzureOpenAIConfig _config;
    private readonly HttpClient _httpClient;

    public AIProvider Provider => AIProvider.AzureOpenAI;
    public bool IsConfigured => !string.IsNullOrEmpty(_config.Endpoint) && !string.IsNullOrEmpty(_config.ApiKey);

    public AzureOpenAIService(CoreAIpet.Core.Models.Settings.AzureOpenAIConfig config)
    {
        _config = config;
        _httpClient = new HttpClient();
        _httpClient.DefaultRequestHeaders.Add("api-key", _config.ApiKey);
    }

    public async Task<ChatResponse> SendMessageAsync(ChatRequest request, CancellationToken ct = default)
    {
        if (!IsConfigured) return new ChatResponse { IsSuccess = false, ErrorMessage = "Azure OpenAI not configured" };

        try
        {
            var url = $"{_config.Endpoint.TrimEnd('/')}/openai/deployments/{_config.DeploymentName}/chat/completions?api-version=2024-02-01";
            var body = new
            {
                messages = request.Messages.Select(m => new { role = m.Role.ToString().ToLower(), content = m.Content }).ToArray(),
                max_tokens = request.MaxTokens,
                temperature = request.Temperature
            };

            var response = await _httpClient.PostAsJsonAsync(url, body, ct);
            response.EnsureSuccessStatusCode();

            var json = await response.Content.ReadAsStringAsync(ct);
            using var doc = JsonDocument.Parse(json);
            var content = doc.RootElement.GetProperty("choices")[0].GetProperty("message").GetProperty("content").GetString() ?? "";
            var model = doc.RootElement.GetProperty("model").GetString() ?? _config.Model;

            return new ChatResponse { Content = content, Model = model, IsSuccess = true };
        }
        catch (Exception ex)
        {
            return new ChatResponse { IsSuccess = false, ErrorMessage = ex.Message };
        }
    }

    public async IAsyncEnumerable<string> SendMessageStreamAsync(ChatRequest request,
        [System.Runtime.CompilerServices.EnumeratorCancellation] CancellationToken ct = default)
    {
        // MVP: 使用非流式实现，后续优化
        var response = await SendMessageAsync(request, ct);
        if (response.IsSuccess)
        {
            // 按字符逐个 yield 模拟流式效果
            foreach (var ch in response.Content)
            {
                yield return ch.ToString();
                await Task.Delay(10, ct);
            }
        }
    }

    public async Task<bool> TestConnectionAsync(CancellationToken ct = default)
    {
        var response = await SendMessageAsync(new ChatRequest
        {
            Messages = new List<ChatMessage> { new() { Role = MessageRole.User, Content = "Hi" } },
            MaxTokens = 5
        }, ct);
        return response.IsSuccess;
    }
}

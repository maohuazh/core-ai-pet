using CoreAIpet.Core.Interfaces;
using CoreAIpet.Core.Models.Chat;
using System.Net.Http;
using System.Net.Http.Json;
using System.Text.Json;

namespace CoreAIpet.Desktop.Services.AI;

/// <summary>
/// Ollama REST API 实现
/// </summary>
public class OllamaService : IAIService
{
    private readonly CoreAIpet.Core.Models.Settings.OllamaConfig _config;
    private readonly HttpClient _httpClient;

    public AIProvider Provider => AIProvider.Ollama;
    public bool IsConfigured => !string.IsNullOrEmpty(_config.Endpoint);

    public OllamaService(CoreAIpet.Core.Models.Settings.OllamaConfig config)
    {
        _config = config;
        _httpClient = new HttpClient { BaseAddress = new Uri(_config.Endpoint) };
    }

    public async Task<ChatResponse> SendMessageAsync(ChatRequest request, CancellationToken ct = default)
    {
        if (!IsConfigured) return new ChatResponse { IsSuccess = false, ErrorMessage = "Ollama endpoint not configured" };

        try
        {
            var body = new
            {
                model = _config.Model,
                messages = request.Messages.Select(m => new { role = m.Role.ToString().ToLower(), content = m.Content }).ToArray(),
                stream = false
            };

            var response = await _httpClient.PostAsJsonAsync("/api/chat", body, ct);
            response.EnsureSuccessStatusCode();

            var json = await response.Content.ReadAsStringAsync(ct);
            using var doc = JsonDocument.Parse(json);
            var content = doc.RootElement.GetProperty("message").GetProperty("content").GetString() ?? "";

            return new ChatResponse { Content = content, Model = _config.Model, IsSuccess = true };
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

        var body = new
        {
            model = _config.Model,
            messages = request.Messages.Select(m => new { role = m.Role.ToString().ToLower(), content = m.Content }).ToArray(),
            stream = true
        };

        var httpRequest = new HttpRequestMessage(HttpMethod.Post, "/api/chat")
        {
            Content = JsonContent.Create(body)
        };

        var response = await _httpClient.SendAsync(httpRequest, HttpCompletionOption.ResponseHeadersRead, ct);
        response.EnsureSuccessStatusCode();

        using var stream = await response.Content.ReadAsStreamAsync(ct);
        using var reader = new System.IO.StreamReader(stream);

        while (!reader.EndOfStream && !ct.IsCancellationRequested)
        {
            var line = await reader.ReadLineAsync(ct);
            if (string.IsNullOrWhiteSpace(line)) continue;

            using var doc = JsonDocument.Parse(line);
            if (doc.RootElement.TryGetProperty("message", out var msg))
            {
                var content = msg.GetProperty("content").GetString();
                if (!string.IsNullOrEmpty(content))
                    yield return content;
            }

            if (doc.RootElement.TryGetProperty("done", out var done) && done.GetBoolean())
                break;
        }
    }

    public async Task<bool> TestConnectionAsync(CancellationToken ct = default)
    {
        try
        {
            var response = await _httpClient.GetAsync("/api/tags", ct);
            return response.IsSuccessStatusCode;
        }
        catch { return false; }
    }
}

using CoreAIpet.Core.Interfaces;
using CoreAIpet.Core.Models.Chat;

namespace CoreAIpet.Desktop.Services.AI;

/// <summary>
/// AI 服务路由 — 策略模式入口，管理多个后端
/// </summary>
public class AIServiceProvider : IAIServiceProvider
{
    private readonly Dictionary<AIProvider, IAIService> _providers;
    private AIProvider _active;

    public IAIService Current => _providers[_active];
    public AIProvider ActiveProvider => _active;

    public AIServiceProvider(
        OpenAIService openAI,
        AzureOpenAIService azureOpenAI,
        OllamaService ollama,
        CoreAIpet.Core.Interfaces.IConfigService config)
    {
        _providers = new Dictionary<AIProvider, IAIService>
        {
            [AIProvider.OpenAI] = openAI,
            [AIProvider.AzureOpenAI] = azureOpenAI,
            [AIProvider.Ollama] = ollama
        };

        _active = Enum.TryParse<AIProvider>(config.AISettings.ActiveProvider, true, out var provider)
            ? provider : AIProvider.OpenAI;
    }

    public void SwitchProvider(AIProvider provider)
    {
        if (!_providers.ContainsKey(provider))
            throw new CoreAIpet.Core.Exceptions.AIServiceException(provider.ToString(), "Provider not registered");
        _active = provider;
    }

    public IReadOnlyList<IAIService> GetAllProviders() => _providers.Values.ToList().AsReadOnly();

    public Task<ChatResponse> SendMessageAsync(ChatRequest request, CancellationToken ct = default)
        => Current.SendMessageAsync(request, ct);

    public IAsyncEnumerable<string> SendMessageStreamAsync(ChatRequest request, CancellationToken ct = default)
        => Current.SendMessageStreamAsync(request, ct);
}

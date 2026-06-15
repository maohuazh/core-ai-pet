using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using CoreAIpet.Core.Interfaces;
using CoreAIpet.Core.Models.Character;
using CoreAIpet.Core.Models.Chat;

namespace CoreAIpet.Desktop.ViewModels;

/// <summary>
/// 聊天 ViewModel — 输入处理、发送命令、流式显示、状态联动
/// </summary>
public partial class ChatViewModel : ObservableObject
{
    private readonly IAIServiceProvider _aiService;
    private readonly ICharacterController _character;
    private readonly Services.AI.ChatSessionManager _session;

    [ObservableProperty] private string _inputText = string.Empty;
    [ObservableProperty] private string _responseText = string.Empty;
    [ObservableProperty] private bool _isResponding;

    public ChatViewModel(IAIServiceProvider aiService, ICharacterController character)
    {
        _aiService = aiService;
        _character = character;
        _session = new Services.AI.ChatSessionManager();
    }

    [RelayCommand]
    private async Task SendAsync(CancellationToken ct)
    {
        if (string.IsNullOrWhiteSpace(InputText) || IsResponding) return;

        var userMessage = InputText;
        InputText = string.Empty;
        ResponseText = string.Empty;
        IsResponding = true;

        _session.AddUserMessage(userMessage);
        _character.SetState(CharacterState.Thinking);

        try
        {
            var systemPrompt = Services.AI.SystemPromptBuilder.BuildDefault();
            var request = _session.BuildRequest(systemPrompt);

            _character.SetState(CharacterState.Talking);

            await foreach (var token in _aiService.SendMessageStreamAsync(request, ct))
            {
                ResponseText += token;
            }

            _session.AddAssistantMessage(ResponseText);
        }
        catch (Exception ex)
        {
            ResponseText = $"Error: {ex.Message}";
        }
        finally
        {
            IsResponding = false;
            _character.SetState(CharacterState.Idle);
        }
    }
}

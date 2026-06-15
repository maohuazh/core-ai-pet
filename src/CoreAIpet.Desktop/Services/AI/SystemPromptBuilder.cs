namespace CoreAIpet.Desktop.Services.AI;

/// <summary>
/// 系统提示词构建
/// </summary>
public static class SystemPromptBuilder
{
    public static string BuildDefault()
    {
        return """
            你是一个桌面 AI 助手，名叫 CoreAIpet。你友好、高效、简洁。
            用户通过桌面悬浮窗口与你对话。请保持回复简短（除非用户要求详细回答）。
            你可以使用中文和英文回复。
            """;
    }
}

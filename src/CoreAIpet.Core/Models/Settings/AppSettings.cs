using System.Text.Json.Serialization;

namespace CoreAIpet.Core.Models.Settings;

/// <summary>
/// 应用配置根对象
/// </summary>
public class AppSettings
{
    [JsonPropertyName("appearance")]
    public AppearanceSettings Appearance { get; set; } = new();

    [JsonPropertyName("system")]
    public SystemSettings System { get; set; } = new();

    [JsonPropertyName("ai")]
    public AISettings AI { get; set; } = new();

    [JsonPropertyName("position")]
    public PositionConfig Position { get; set; } = new();

    [JsonPropertyName("plugins")]
    public PluginsConfig Plugins { get; set; } = new();

    [JsonPropertyName("debug")]
    public DebugSettings Debug { get; set; } = new();
}

/// <summary>
/// 外观设置
/// </summary>
public class AppearanceSettings
{
    [JsonPropertyName("scale")]
    public double Scale { get; set; } = 1.0;

    [JsonPropertyName("opacity")]
    public double Opacity { get; set; } = 1.0;

    [JsonPropertyName("theme")]
    public string Theme { get; set; } = "dark";
}

/// <summary>
/// 系统设置
/// </summary>
public class SystemSettings
{
    [JsonPropertyName("autoStart")]
    public bool AutoStart { get; set; } = false;

    [JsonPropertyName("alwaysOnTop")]
    public bool AlwaysOnTop { get; set; } = true;

    [JsonPropertyName("clickThrough")]
    public bool ClickThrough { get; set; } = false;
}

/// <summary>
/// AI 设置
/// </summary>
public class AISettings
{
    [JsonPropertyName("activeProvider")]
    public string ActiveProvider { get; set; } = "openai";

    [JsonPropertyName("openai")]
    public OpenAIConfig OpenAI { get; set; } = new();

    [JsonPropertyName("azureOpenAI")]
    public AzureOpenAIConfig AzureOpenAI { get; set; } = new();

    [JsonPropertyName("ollama")]
    public OllamaConfig Ollama { get; set; } = new();
}

/// <summary>
/// OpenAI 配置
/// </summary>
public class OpenAIConfig
{
    [JsonPropertyName("endpoint")]
    public string Endpoint { get; set; } = "https://api.openai.com/v1";

    [JsonPropertyName("apiKey")]
    public string ApiKey { get; set; } = string.Empty;

    [JsonPropertyName("model")]
    public string Model { get; set; } = "gpt-4";
}

/// <summary>
/// Azure OpenAI 配置
/// </summary>
public class AzureOpenAIConfig
{
    [JsonPropertyName("endpoint")]
    public string Endpoint { get; set; } = string.Empty;

    [JsonPropertyName("apiKey")]
    public string ApiKey { get; set; } = string.Empty;

    [JsonPropertyName("deploymentName")]
    public string DeploymentName { get; set; } = string.Empty;

    [JsonPropertyName("model")]
    public string Model { get; set; } = string.Empty;
}

/// <summary>
/// Ollama 配置
/// </summary>
public class OllamaConfig
{
    [JsonPropertyName("endpoint")]
    public string Endpoint { get; set; } = "http://localhost:11434";

    [JsonPropertyName("model")]
    public string Model { get; set; } = "llama3";
}

/// <summary>
/// 位置配置
/// </summary>
public class PositionConfig
{
    [JsonPropertyName("x")]
    public double X { get; set; } = 1600;

    [JsonPropertyName("y")]
    public double Y { get; set; } = 820;
}

/// <summary>
/// 插件总配置
/// </summary>
public class PluginsConfig
{
    [JsonPropertyName("jira")]
    public JiraPluginConfig Jira { get; set; } = new();

    [JsonPropertyName("email")]
    public EmailPluginConfig Email { get; set; } = new();

    [JsonPropertyName("message")]
    public MessagePluginConfig Message { get; set; } = new();
}

/// <summary>
/// Jira 插件配置
/// </summary>
public class JiraPluginConfig
{
    [JsonPropertyName("enabled")]
    public bool Enabled { get; set; } = true;

    [JsonPropertyName("baseUrl")]
    public string BaseUrl { get; set; } = string.Empty;

    [JsonPropertyName("username")]
    public string Username { get; set; } = string.Empty;

    [JsonPropertyName("apiToken")]
    public string ApiToken { get; set; } = string.Empty;
}

/// <summary>
/// Email 插件配置
/// </summary>
public class EmailPluginConfig
{
    [JsonPropertyName("enabled")]
    public bool Enabled { get; set; } = true;

    [JsonPropertyName("provider")]
    public string Provider { get; set; } = "outlook";
}

/// <summary>
/// Message 插件配置
/// </summary>
public class MessagePluginConfig
{
    [JsonPropertyName("enabled")]
    public bool Enabled { get; set; } = true;
}

/// <summary>
/// 调试设置
/// </summary>
public class DebugSettings
{
    [JsonPropertyName("logLevel")]
    public string LogLevel { get; set; } = "Information";

    [JsonPropertyName("showFps")]
    public bool ShowFps { get; set; } = false;
}

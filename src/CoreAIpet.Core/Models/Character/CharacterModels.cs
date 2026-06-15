using System.Text.Json.Serialization;

namespace CoreAIpet.Core.Models.Character;

/// <summary>
/// 角色状态枚举
/// </summary>
[JsonConverter(typeof(JsonStringEnumConverter))]
public enum CharacterState
{
    /// <summary>待机</summary>
    Idle,

    /// <summary>Hover (鼠标悬停)</summary>
    Happy,

    /// <summary>AI 处理中</summary>
    Thinking,

    /// <summary>AI 回复中</summary>
    Talking
}

/// <summary>
/// 眼球追踪坐标
/// </summary>
public class EyePosition
{
    /// <summary>水平角度 (-1.0 ~ 1.0, 对应 ±30°)</summary>
    public double X { get; set; }

    /// <summary>垂直角度 (-1.0 ~ 1.0, 对应 ±15°)</summary>
    public double Y { get; set; }
}

using CoreAIpet.Core.Models.Character;

namespace CoreAIpet.Desktop.Live2D.Animation;

/// <summary>
/// 角色状态 → 动画组映射
/// 碧蓝航线模型所有动作都在空字符串组 "" 中，统一返回 ""，由 Bridge 随机选取动作
/// </summary>
public static class StateAnimationMapper
{
    private static readonly Dictionary<CharacterState, (string Group, string Name)> StateMap = new()
    {
        [CharacterState.Idle] = ("", ""),
        [CharacterState.Happy] = ("", ""),
        [CharacterState.Thinking] = ("", ""),
        [CharacterState.Talking] = ("", ""),
    };

    public static (string Group, string Name) GetAnimation(CharacterState state)
    {
        return StateMap.TryGetValue(state, out var anim) ? anim : ("", "");
    }
}

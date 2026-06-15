using CoreAIpet.Core.Models.Character;

namespace CoreAIpet.Desktop.Live2D.Animation;

/// <summary>
/// 角色状态 → 动画组映射
/// </summary>
public static class StateAnimationMapper
{
    private static readonly Dictionary<CharacterState, (string Group, string Name)> StateMap = new()
    {
        [CharacterState.Idle] = ("idle", "idle_01"),
        [CharacterState.Happy] = ("happy", "happy_01"),
        [CharacterState.Thinking] = ("thinking", "think_01"),
        [CharacterState.Talking] = ("talking", "talk_01"),
    };

    public static (string Group, string Name) GetAnimation(CharacterState state)
    {
        return StateMap.TryGetValue(state, out var anim) ? anim : ("idle", "idle_01");
    }
}

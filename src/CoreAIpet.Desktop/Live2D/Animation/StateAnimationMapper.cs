using CoreAIpet.Core.Models.Character;

namespace CoreAIpet.Desktop.Live2D.Animation;

/// <summary>
/// 角色状态 → 动画组映射
/// 适配 Hiyori 模型的 motion group：Idle（9 个动作）和 TapBody（1 个动作）
/// </summary>
public static class StateAnimationMapper
{
    private static readonly Dictionary<CharacterState, (string Group, string Name)> StateMap = new()
    {
        [CharacterState.Idle] = ("Idle", ""),       // 随机选择 Idle 组动作
        [CharacterState.Happy] = ("TapBody", ""),   // TapBody 作为开心反应
        [CharacterState.Thinking] = ("Idle", ""),   // 复用 Idle 组的不同 variant
        [CharacterState.Talking] = ("Idle", ""),    // 复用 Idle 组（后续可加 LipSync）
    };

    public static (string Group, string Name) GetAnimation(CharacterState state)
    {
        return StateMap.TryGetValue(state, out var anim) ? anim : ("Idle", "");
    }
}

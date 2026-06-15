using CoreAIpet.Core.Models.Character;

namespace CoreAIpet.Core.Interfaces;

/// <summary>
/// 角色状态变更事件参数
/// </summary>
public class CharacterStateChangedEventArgs : EventArgs
{
    public CharacterState OldState { get; }
    public CharacterState NewState { get; }

    public CharacterStateChangedEventArgs(CharacterState oldState, CharacterState newState)
    {
        OldState = oldState;
        NewState = newState;
    }
}

/// <summary>
/// 角色控制器 — 管理 Live2D 角色的状态、动画、眼球追踪
/// </summary>
public interface ICharacterController
{
    /// <summary>当前角色状态</summary>
    CharacterState CurrentState { get; }

    /// <summary>切换到指定状态</summary>
    void SetState(CharacterState newState);

    /// <summary>切换到指定状态, 并设置最短持续时间</summary>
    void SetState(CharacterState newState, TimeSpan minDuration);

    /// <summary>更新眼球追踪 (鼠标坐标 → 眼球角度)</summary>
    void UpdateEyeTracking(double mouseX, double mouseY);

    /// <summary>播放指定动画</summary>
    void PlayAnimation(string animationGroup, string animationName);

    /// <summary>状态变更事件</summary>
    event EventHandler<CharacterStateChangedEventArgs>? StateChanged;
}

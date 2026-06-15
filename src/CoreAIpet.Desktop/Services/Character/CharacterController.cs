using CoreAIpet.Core.Events;
using CoreAIpet.Core.Interfaces;
using CoreAIpet.Core.Models.Character;
using CoreAIpet.Desktop.Live2D.Animation;
using CoreAIpet.Desktop.Live2D.Rendering;

namespace CoreAIpet.Desktop.Services.Character;

/// <summary>
/// 角色控制器 — 状态机 + 动画控制 + 眼球追踪
/// </summary>
public class CharacterController : ICharacterController
{
    private readonly Live2DRenderHost _renderHost;
    private readonly EyeFollowController _eyeFollow;
    private CharacterState _state = CharacterState.Idle;
    private DateTime _stateEnteredAt = DateTime.MinValue;
    private TimeSpan _minDuration = TimeSpan.Zero;

    public CharacterState CurrentState => _state;

    public event EventHandler<CharacterStateChangedEventArgs>? StateChanged;

    private static readonly HashSet<(CharacterState From, CharacterState To)> ValidTransitions = new()
    {
        (CharacterState.Idle, CharacterState.Happy),
        (CharacterState.Idle, CharacterState.Thinking),
        (CharacterState.Happy, CharacterState.Idle),
        (CharacterState.Happy, CharacterState.Thinking),
        (CharacterState.Thinking, CharacterState.Talking),
        (CharacterState.Thinking, CharacterState.Idle),
        (CharacterState.Talking, CharacterState.Idle),
        (CharacterState.Talking, CharacterState.Thinking),
    };

    public CharacterController(Live2DRenderHost renderHost)
    {
        _renderHost = renderHost;
        _eyeFollow = new EyeFollowController(renderHost);
    }

    public void SetState(CharacterState newState)
    {
        if (_state == newState) return;
        if (DateTime.Now - _stateEnteredAt < _minDuration) return;
        if (!ValidTransitions.Contains((_state, newState))) return;

        var oldState = _state;
        _state = newState;
        _stateEnteredAt = DateTime.Now;
        _minDuration = TimeSpan.Zero;

        // 触发 Live2D 动画切换
        var (group, name) = StateAnimationMapper.GetAnimation(newState);
        _renderHost.SetMotion(group, name);

        StateChanged?.Invoke(this, new CharacterStateChangedEventArgs(oldState, newState));
    }

    public void SetState(CharacterState newState, TimeSpan minDuration)
    {
        SetState(newState);
        _minDuration = minDuration;
    }

    public void UpdateEyeTracking(double mouseX, double mouseY)
    {
        _eyeFollow.Update();
    }

    public void PlayAnimation(string animationGroup, string animationName)
    {
        _renderHost.SetMotion(animationGroup, animationName);
    }
}

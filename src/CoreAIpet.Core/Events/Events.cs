using CoreAIpet.Core.Models.Character;
using CoreAIpet.Core.Models.Chat;

namespace CoreAIpet.Core.Events;

/// <summary>
/// 事件标记接口
/// </summary>
public interface IEvent { }

/// <summary>
/// 聊天消息事件
/// </summary>
public class ChatMessageEvent : IEvent
{
    public ChatMessage Message { get; init; } = null!;
}

/// <summary>
/// 角色状态变更事件
/// </summary>
public class CharacterStateChangedEvent : IEvent
{
    public CharacterState OldState { get; init; }
    public CharacterState NewState { get; init; }
}

/// <summary>
/// 插件生命周期事件
/// </summary>
public class PluginLifecycleEvent : IEvent
{
    public string PluginId { get; init; } = string.Empty;
    public string Action { get; init; } = string.Empty; // Loaded/Activated/Deactivated/Unloaded/Error
    public string? ErrorMessage { get; init; }
}

/// <summary>
/// 设置变更事件
/// </summary>
public class SettingsChangedEvent : IEvent
{
    public string Section { get; init; } = string.Empty;
}

/// <summary>
/// 菜单项点击事件
/// </summary>
public class MenuActionEvent : IEvent
{
    public string MenuItemId { get; init; } = string.Empty;
    public string PluginId { get; init; } = string.Empty;
}

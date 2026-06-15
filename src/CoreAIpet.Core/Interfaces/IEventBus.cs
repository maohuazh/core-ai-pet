using CoreAIpet.Core.Events;

namespace CoreAIpet.Core.Interfaces;

/// <summary>
/// 轻量级内存事件总线 — 解耦模块间通信
/// </summary>
public interface IEventBus
{
    /// <summary>发布事件</summary>
    void Publish<TEvent>(TEvent @event) where TEvent : IEvent;

    /// <summary>同步订阅</summary>
    IDisposable Subscribe<TEvent>(Action<TEvent> handler) where TEvent : IEvent;

    /// <summary>异步订阅</summary>
    IDisposable Subscribe<TEvent>(Func<TEvent, Task> handler) where TEvent : IEvent;
}

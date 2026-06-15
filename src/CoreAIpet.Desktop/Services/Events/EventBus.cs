using System.Collections.Concurrent;
using CoreAIpet.Core.Events;
using CoreAIpet.Core.Interfaces;

namespace CoreAIpet.Desktop.Services.Events;

/// <summary>
/// 轻量级内存事件总线 — 线程安全的发布-订阅实现
/// </summary>
public class EventBus : IEventBus
{
    private readonly ConcurrentDictionary<Type, List<Delegate>> _subscriptions = new();
    private readonly object _lock = new();

    public void Publish<TEvent>(TEvent @event) where TEvent : IEvent
    {
        if (@event == null) return;

        var eventType = typeof(TEvent);
        if (!_subscriptions.TryGetValue(eventType, out var handlers))
            return;

        List<Delegate> handlersCopy;
        lock (_lock)
        {
            handlersCopy = new List<Delegate>(handlers);
        }

        foreach (var handler in handlersCopy)
        {
            try
            {
                if (handler is Func<TEvent, Task> asyncHandler)
                {
                    _ = asyncHandler(@event);
                }
                else if (handler is Action<TEvent> syncHandler)
                {
                    syncHandler(@event);
                }
            }
            catch
            {
                // 事件处理异常不应影响发布者
            }
        }
    }

    public IDisposable Subscribe<TEvent>(Action<TEvent> handler) where TEvent : IEvent
    {
        return AddSubscription(typeof(TEvent), handler);
    }

    public IDisposable Subscribe<TEvent>(Func<TEvent, Task> handler) where TEvent : IEvent
    {
        return AddSubscription(typeof(TEvent), handler);
    }

    private IDisposable AddSubscription(Type eventType, Delegate handler)
    {
        var handlers = _subscriptions.GetOrAdd(eventType, _ => new List<Delegate>());
        lock (_lock)
        {
            handlers.Add(handler);
        }
        return new Subscription(() => RemoveSubscription(eventType, handler));
    }

    private void RemoveSubscription(Type eventType, Delegate handler)
    {
        if (!_subscriptions.TryGetValue(eventType, out var handlers))
            return;

        lock (_lock)
        {
            handlers.Remove(handler);
        }
    }

    private class Subscription : IDisposable
    {
        private readonly Action _unsubscribe;

        public Subscription(Action unsubscribe)
        {
            _unsubscribe = unsubscribe;
        }

        public void Dispose()
        {
            _unsubscribe();
        }
    }
}

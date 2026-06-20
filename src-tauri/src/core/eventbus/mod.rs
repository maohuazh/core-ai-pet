use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// 标准事件类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventType {
    UserChat,
    LLMResponse,
    EmailReceived,
    SlackMessage,
    JiraUpdated,
    MeetingStarted,
    TaskCompleted,
    StateChanged,
    Custom(String),
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::UserChat => write!(f, "UserChat"),
            EventType::LLMResponse => write!(f, "LLMResponse"),
            EventType::EmailReceived => write!(f, "EmailReceived"),
            EventType::SlackMessage => write!(f, "SlackMessage"),
            EventType::JiraUpdated => write!(f, "JiraUpdated"),
            EventType::MeetingStarted => write!(f, "MeetingStarted"),
            EventType::TaskCompleted => write!(f, "TaskCompleted"),
            EventType::StateChanged => write!(f, "StateChanged"),
            EventType::Custom(s) => write!(f, "Custom({})", s),
        }
    }
}

/// 事件结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event_type: String,
    pub source: String,
    pub timestamp: u64,
    pub payload: serde_json::Value,
}

impl Event {
    pub fn new(event_type: impl Into<String>, source: impl Into<String>, payload: serde_json::Value) -> Self {
        Event {
            event_type: event_type.into(),
            source: source.into(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            payload,
        }
    }
}

/// 事件处理器
type EventHandler = Sender<Event>;

/// 事件去重记录
struct DedupEntry {
    event_type: String,
    payload_hash: u64,
    timestamp: u64,
}

/// 发布-订阅事件总线
pub struct EventBus {
    subscribers: Arc<Mutex<HashMap<String, Vec<EventHandler>>>>,
    dedup_window_ms: u64,
    recent_events: Arc<Mutex<Vec<DedupEntry>>>,
}

impl EventBus {
    pub fn new() -> Self {
        EventBus {
            subscribers: Arc::new(Mutex::new(HashMap::new())),
            dedup_window_ms: 100,
            recent_events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// 订阅特定类型的事件
    pub fn subscribe(&self, event_type: &str) -> Receiver<Event> {
        let (tx, rx) = mpsc::channel();
        let mut subs = self.subscribers.lock().unwrap();
        subs.entry(event_type.to_string())
            .or_insert_with(Vec::new)
            .push(tx);
        log::debug!("Subscribed to event type: {}", event_type);
        rx
    }

    /// 发布事件
    pub fn publish(&self, event: Event) {
        log::debug!(
            "Event published: type={}, source={}",
            event.event_type,
            event.source
        );

        // Deduplication check
        if self.is_duplicate(&event) {
            log::debug!("Duplicate event dropped: {}", event.event_type);
            return;
        }

        // Record event for future dedup
        self.record_event(&event);

        // Deliver to subscribers
        let subs = self.subscribers.lock().unwrap();
        if let Some(handlers) = subs.get(&event.event_type) {
            let mut dead_indices = Vec::new();
            for (i, handler) in handlers.iter().enumerate() {
                if handler.send(event.clone()).is_err() {
                    dead_indices.push(i);
                }
            }
            // Note: can't remove dead handlers while holding immutable ref
            // This is acceptable for now - dead channels will be cleaned on next publish
            drop(subs);

            // Clean up dead handlers
            if !dead_indices.is_empty() {
                let mut subs = self.subscribers.lock().unwrap();
                if let Some(handlers) = subs.get_mut(&event.event_type) {
                    for i in dead_indices.into_iter().rev() {
                        handlers.remove(i);
                    }
                }
            }
        }
    }

    /// 取消订阅（通过移除所有该类型的处理器）
    pub fn unsubscribe(&self, event_type: &str) {
        let mut subs = self.subscribers.lock().unwrap();
        subs.remove(event_type);
        log::debug!("Unsubscribed from event type: {}", event_type);
    }

    /// 检查事件是否重复
    fn is_duplicate(&self, event: &Event) -> bool {
        let recent = self.recent_events.lock().unwrap();
        let now = event.timestamp;
        let payload_hash = self.hash_payload(&event.payload);

        for entry in recent.iter() {
            if entry.event_type == event.event_type
                && entry.payload_hash == payload_hash
                && now.saturating_sub(entry.timestamp) < self.dedup_window_ms
            {
                return true;
            }
        }
        false
    }

    /// 记录事件用于去重
    fn record_event(&self, event: &Event) {
        let mut recent = self.recent_events.lock().unwrap();
        let payload_hash = self.hash_payload(&event.payload);

        recent.push(DedupEntry {
            event_type: event.event_type.clone(),
            payload_hash,
            timestamp: event.timestamp,
        });

        // Keep only recent events (last 100)
        let len = recent.len();
        if len > 100 {
            recent.drain(0..len - 100);
        }
    }

    /// 简单 payload hash
    fn hash_payload(&self, payload: &serde_json::Value) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        payload.to_string().hash(&mut hasher);
        hasher.finish()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for EventBus {
    fn clone(&self) -> Self {
        EventBus {
            subscribers: Arc::clone(&self.subscribers),
            dedup_window_ms: self.dedup_window_ms,
            recent_events: Arc::clone(&self.recent_events),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publish_subscribe() {
        let bus = EventBus::new();
        let rx = bus.subscribe("test_event");

        let event = Event::new("test_event", "test", serde_json::json!({"key": "value"}));
        bus.publish(event);

        let received = rx.recv().unwrap();
        assert_eq!(received.event_type, "test_event");
        assert_eq!(received.source, "test");
    }

    #[test]
    fn test_no_receive_for_other_events() {
        let bus = EventBus::new();
        let rx = bus.subscribe("type_a");

        let event = Event::new("type_b", "test", serde_json::json!({}));
        bus.publish(event);

        assert!(rx.try_recv().is_err());
    }

    #[test]
    fn test_deduplication() {
        let bus = EventBus::new();
        let rx = bus.subscribe("test");

        let event1 = Event::new("test", "src", serde_json::json!({"a": 1}));
        let event2 = Event {
            timestamp: event1.timestamp, // same timestamp
            ..event1.clone()
        };

        bus.publish(event1);
        bus.publish(event2);

        // Only one should be received
        assert!(rx.try_recv().is_ok());
        assert!(rx.try_recv().is_err());
    }

    #[test]
    fn test_unsubscribe() {
        let bus = EventBus::new();
        let _rx = bus.subscribe("test");
        bus.unsubscribe("test");

        let event = Event::new("test", "src", serde_json::json!({}));
        bus.publish(event);
        // After unsubscribe, no handlers exist
    }

    #[test]
    fn test_event_creation() {
        let event = Event::new("UserChat", "frontend", serde_json::json!({"message": "hello"}));
        assert_eq!(event.event_type, "UserChat");
        assert_eq!(event.source, "frontend");
        assert!(event.timestamp > 0);
    }

    #[test]
    fn test_event_type_display() {
        assert_eq!(format!("{}", EventType::UserChat), "UserChat");
        assert_eq!(format!("{}", EventType::Custom("foo".into())), "Custom(foo)");
    }
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

/// 角色状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PetState {
    Idle,
    Walking,
    Thinking,
    Talking,
    Working,
    Meeting,
    Sleeping,
    Alert,
}

impl std::fmt::Display for PetState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PetState::Idle => write!(f, "Idle"),
            PetState::Walking => write!(f, "Walking"),
            PetState::Thinking => write!(f, "Thinking"),
            PetState::Talking => write!(f, "Talking"),
            PetState::Working => write!(f, "Working"),
            PetState::Meeting => write!(f, "Meeting"),
            PetState::Sleeping => write!(f, "Sleeping"),
            PetState::Alert => write!(f, "Alert"),
        }
    }
}

/// 触发状态转换的事件
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StateEvent {
    UserChat,
    LLMResponse,
    ResponseComplete,
    EmailReceived,
    SlackMessage,
    JiraUpdated,
    MeetingStarted,
    MeetingEnded,
    TaskCompleted,
    WakeUp,
    GoToSleep,
    UserInteract,
    Idle,
}

/// 状态变化事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChanged {
    pub old_state: PetState,
    pub new_state: PetState,
    pub event: StateEvent,
    pub timestamp: u64,
}

/// 状态转换错误
#[derive(Debug)]
pub enum StateError {
    InvalidTransition { from: PetState, event: StateEvent },
    MinDurationNotMet { remaining_ms: u64 },
}

impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateError::InvalidTransition { from, event } => {
                write!(f, "Invalid transition: {} + {:?}", from, event)
            }
            StateError::MinDurationNotMet { remaining_ms } => {
                write!(f, "Minimum duration not met, {}ms remaining", remaining_ms)
            }
        }
    }
}

/// 状态机
pub struct StateMachine {
    current_state: PetState,
    transitions: HashMap<(PetState, StateEvent), PetState>,
    entered_at: Instant,
    min_duration_ms: HashMap<PetState, u64>,
}

impl StateMachine {
    /// 创建新的状态机，初始状态为 Idle
    pub fn new() -> Self {
        let mut transitions = HashMap::new();

        // Idle transitions
        transitions.insert((PetState::Idle, StateEvent::UserChat), PetState::Thinking);
        transitions.insert((PetState::Idle, StateEvent::UserInteract), PetState::Walking);
        transitions.insert((PetState::Idle, StateEvent::EmailReceived), PetState::Alert);
        transitions.insert((PetState::Idle, StateEvent::SlackMessage), PetState::Alert);
        transitions.insert((PetState::Idle, StateEvent::JiraUpdated), PetState::Alert);
        transitions.insert((PetState::Idle, StateEvent::MeetingStarted), PetState::Meeting);
        transitions.insert((PetState::Idle, StateEvent::GoToSleep), PetState::Sleeping);

        // Walking transitions
        transitions.insert((PetState::Walking, StateEvent::Idle), PetState::Idle);
        transitions.insert((PetState::Walking, StateEvent::UserChat), PetState::Thinking);

        // Thinking transitions
        transitions.insert((PetState::Thinking, StateEvent::LLMResponse), PetState::Talking);
        transitions.insert((PetState::Thinking, StateEvent::UserChat), PetState::Thinking); // self-loop

        // Talking transitions
        transitions.insert((PetState::Talking, StateEvent::ResponseComplete), PetState::Idle);
        transitions.insert((PetState::Talking, StateEvent::UserChat), PetState::Thinking); // interrupt

        // Working transitions
        transitions.insert((PetState::Working, StateEvent::TaskCompleted), PetState::Idle);
        transitions.insert((PetState::Working, StateEvent::UserChat), PetState::Thinking);

        // Meeting transitions
        transitions.insert((PetState::Meeting, StateEvent::MeetingEnded), PetState::Idle);

        // Sleeping transitions
        transitions.insert((PetState::Sleeping, StateEvent::WakeUp), PetState::Idle);
        transitions.insert((PetState::Sleeping, StateEvent::UserInteract), PetState::Idle);

        // Alert transitions
        transitions.insert((PetState::Alert, StateEvent::Idle), PetState::Idle);
        transitions.insert((PetState::Alert, StateEvent::UserInteract), PetState::Idle);

        StateMachine {
            current_state: PetState::Idle,
            transitions,
            entered_at: Instant::now(),
            min_duration_ms: HashMap::new(),
        }
    }

    /// 根据事件触发状态转换
    pub fn transition(&mut self, event: StateEvent) -> Result<PetState, StateError> {
        // Check minimum duration
        if let Some(&min_ms) = self.min_duration_ms.get(&self.current_state) {
            let elapsed = self.entered_at.elapsed().as_millis() as u64;
            if elapsed < min_ms {
                return Err(StateError::MinDurationNotMet {
                    remaining_ms: min_ms - elapsed,
                });
            }
        }

        let key = (self.current_state, event.clone());
        match self.transitions.get(&key) {
            Some(&new_state) => {
                let old_state = self.current_state;
                self.current_state = new_state;
                self.entered_at = Instant::now();
                log::debug!(
                    "State transition: {} -> {} (event: {:?})",
                    old_state,
                    new_state,
                    event
                );
                Ok(new_state)
            }
            None => {
                log::warn!(
                    "Invalid transition: {} + {:?}, staying in current state",
                    self.current_state,
                    event
                );
                Err(StateError::InvalidTransition {
                    from: self.current_state,
                    event,
                })
            }
        }
    }

    /// 获取当前状态
    pub fn get_state(&self) -> PetState {
        self.current_state
    }

    /// 强制设置状态（用于调试和特殊场景）
    pub fn set_state(&mut self, state: PetState) {
        let old_state = self.current_state;
        self.current_state = state;
        self.entered_at = Instant::now();
        log::info!("Force state set: {} -> {}", old_state, state);
    }

    /// 设置状态的最小持续时间
    pub fn set_min_duration(&mut self, state: PetState, duration_ms: u64) {
        self.min_duration_ms.insert(state, duration_ms);
    }

    /// 创建 StateChanged 事件
    pub fn create_changed_event(&self, old_state: PetState, event: StateEvent) -> StateChanged {
        StateChanged {
            old_state,
            new_state: self.current_state,
            event,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state_is_idle() {
        let sm = StateMachine::new();
        assert_eq!(sm.get_state(), PetState::Idle);
    }

    #[test]
    fn test_valid_transition() {
        let mut sm = StateMachine::new();
        let result = sm.transition(StateEvent::UserChat);
        assert!(result.is_ok());
        assert_eq!(sm.get_state(), PetState::Thinking);
    }

    #[test]
    fn test_invalid_transition() {
        let mut sm = StateMachine::new();
        // Sleeping state doesn't accept UserChat without WakeUp first
        sm.set_state(PetState::Sleeping);
        let result = sm.transition(StateEvent::UserChat);
        assert!(result.is_err());
        assert_eq!(sm.get_state(), PetState::Sleeping);
    }

    #[test]
    fn test_force_set_state() {
        let mut sm = StateMachine::new();
        sm.set_state(PetState::Alert);
        assert_eq!(sm.get_state(), PetState::Alert);
    }

    #[test]
    fn test_thinking_to_talking() {
        let mut sm = StateMachine::new();
        sm.set_state(PetState::Thinking);
        let result = sm.transition(StateEvent::LLMResponse);
        assert!(result.is_ok());
        assert_eq!(sm.get_state(), PetState::Talking);
    }

    #[test]
    fn test_min_duration_blocks_transition() {
        let mut sm = StateMachine::new();
        sm.set_min_duration(PetState::Idle, 5000); // 5 second minimum
        // Immediately try to transition - should fail
        let result = sm.transition(StateEvent::UserChat);
        assert!(result.is_err());
        assert_eq!(sm.get_state(), PetState::Idle);
    }

    #[test]
    fn test_self_loop() {
        let mut sm = StateMachine::new();
        sm.set_state(PetState::Thinking);
        let result = sm.transition(StateEvent::UserChat);
        assert!(result.is_ok());
        assert_eq!(sm.get_state(), PetState::Thinking);
    }

    #[test]
    fn test_state_display() {
        assert_eq!(format!("{}", PetState::Idle), "Idle");
        assert_eq!(format!("{}", PetState::Thinking), "Thinking");
    }
}

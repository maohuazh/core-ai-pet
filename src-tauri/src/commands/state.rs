use tauri::{AppHandle, Manager, State};

use crate::core::state::{PetState, StateMachine};

#[tauri::command]
pub async fn get_state(state_machine: State<'_, tokio::sync::Mutex<StateMachine>>) -> Result<String, String> {
    let sm = state_machine.lock().await;
    Ok(serde_json::to_string(&sm.get_state()).unwrap_or_else(|_| "\"Idle\"".to_string()))
}

#[tauri::command]
pub async fn set_state(
    state_machine: State<'_, tokio::sync::Mutex<StateMachine>>,
    state: String,
) -> Result<String, String> {
    let mut sm = state_machine.lock().await;
    let new_state: PetState = serde_json::from_str(&format!("\"{}\"", state))
        .map_err(|e| format!("Invalid state: {}", e))?;
    sm.set_state(new_state);
    Ok(format!("State set to: {}", new_state))
}

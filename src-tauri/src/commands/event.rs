use serde_json::Value;
use tauri::{AppHandle, Emitter, State};

use crate::core::eventbus::{Event, EventBus};

#[tauri::command]
pub async fn emit_event(
    app: AppHandle,
    event_bus: State<'_, tokio::sync::Mutex<EventBus>>,
    event_type: String,
    source: String,
    payload: Value,
) -> Result<(), String> {
    let event = Event::new(event_type.clone(), source, payload);

    // Publish to internal event bus
    {
        let bus = event_bus.lock().await;
        bus.publish(event.clone());
    }

    // Also emit via Tauri event system for frontend listeners
    app.emit(&event_type, &event)
        .map_err(|e| format!("Failed to emit event: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn subscribe_event(
    event_bus: State<'_, tokio::sync::Mutex<EventBus>>,
    event_type: String,
) -> Result<String, String> {
    let bus = event_bus.lock().await;
    let _rx = bus.subscribe(&event_type);
    // Note: actual event delivery happens via Tauri's emit/listen
    // This command is a placeholder for backend subscriptions
    Ok(format!("Subscribed to: {}", event_type))
}

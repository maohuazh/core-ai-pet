use tauri::State;

use crate::infrastructure::storage::Database;

#[tauri::command]
pub async fn storage_get(
    db: State<'_, tokio::sync::Mutex<Database>>,
    key: String,
) -> Result<Option<String>, String> {
    let db = db.lock().await;
    db.storage_get(&key)
        .map_err(|e| format!("Failed to get config: {}", e))
}

#[tauri::command]
pub async fn storage_set(
    db: State<'_, tokio::sync::Mutex<Database>>,
    key: String,
    value: String,
) -> Result<(), String> {
    let db = db.lock().await;
    db.storage_set(&key, &value)
        .map_err(|e| format!("Failed to set config: {}", e))
}

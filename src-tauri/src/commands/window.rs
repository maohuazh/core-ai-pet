use tauri::{AppHandle, Manager, Window};

/// Start dragging the window (called from frontend on mousedown)
#[tauri::command]
pub async fn start_dragging(window: Window) -> Result<(), String> {
    window
        .start_dragging()
        .map_err(|e| format!("Failed to start dragging: {}", e))
}

/// Set window position programmatically
#[tauri::command]
pub async fn set_window_position(
    app: AppHandle,
    x: f64,
    y: f64,
) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("Main window not found")?;

    window
        .set_position(tauri::PhysicalPosition::new(x as i32, y as i32))
        .map_err(|e| format!("Failed to set position: {}", e))
}

/// Get current window position
#[tauri::command]
pub async fn get_window_position(app: AppHandle) -> Result<(f64, f64), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("Main window not found")?;

    let position = window
        .outer_position()
        .map_err(|e| format!("Failed to get position: {}", e))?;

    Ok((position.x as f64, position.y as f64))
}

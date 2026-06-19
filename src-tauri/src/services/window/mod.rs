use tauri::Window;

/// Window service for managing window operations
pub struct WindowService;

impl WindowService {
    /// Start dragging the window
    pub fn start_dragging(window: &Window) -> Result<(), String> {
        window
            .start_dragging()
            .map_err(|e| format!("Failed to start dragging: {}", e))
    }
}

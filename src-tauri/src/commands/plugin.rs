use tauri::State;

use crate::core::plugin::{PluginInfo, PluginManager};

#[tauri::command]
pub async fn plugin_list(
    plugin_manager: State<'_, tokio::sync::Mutex<PluginManager>>,
) -> Result<Vec<PluginInfo>, String> {
    let pm = plugin_manager.lock().await;
    Ok(pm.list_plugins())
}

#[tauri::command]
pub async fn plugin_enable(
    plugin_manager: State<'_, tokio::sync::Mutex<PluginManager>>,
    plugin_id: String,
) -> Result<(), String> {
    let mut pm = plugin_manager.lock().await;
    pm.enable_plugin(&plugin_id)
}

#[tauri::command]
pub async fn plugin_disable(
    plugin_manager: State<'_, tokio::sync::Mutex<PluginManager>>,
    plugin_id: String,
) -> Result<(), String> {
    let mut pm = plugin_manager.lock().await;
    pm.disable_plugin(&plugin_id)
}

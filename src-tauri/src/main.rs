#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod core;
mod infrastructure;
mod services;

use commands::event::{emit_event, subscribe_event};
use commands::plugin::{plugin_disable, plugin_enable, plugin_list};
use commands::state::{get_state, set_state};
use commands::storage::{storage_get, storage_set};
use commands::window::{get_window_position, set_window_position, start_dragging};

use crate::core::eventbus::EventBus;
use crate::core::plugin::PluginManager;
use crate::core::state::StateMachine;
use crate::infrastructure::storage::{default_db_path, Database};

fn main() {
    // Initialize logger
    env_logger::init();

    // Initialize database
    let db_path = default_db_path();
    let db = Database::open(db_path).expect("Failed to open database");

    // Initialize state machine
    let state_machine = StateMachine::new();

    // Initialize event bus
    let event_bus = EventBus::new();

    // Initialize plugin manager and load plugins
    let plugins_dir = std::env::current_dir()
        .unwrap_or_default()
        .join("plugins");
    let mut plugin_manager = PluginManager::new(plugins_dir);
    if let Err(e) = plugin_manager.load_plugins() {
        log::error!("Failed to load plugins: {}", e);
    }

    // Restore plugin states from database
    if let Err(e) = plugin_manager.restore_states(&db) {
        log::error!("Failed to restore plugin states: {}", e);
    }

    // Restore window position
    if let Ok(Some(pos)) = db.load_window_position() {
        log::info!("Restored window position: ({}, {})", pos.x, pos.y);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(tokio::sync::Mutex::new(state_machine))
        .manage(tokio::sync::Mutex::new(event_bus))
        .manage(tokio::sync::Mutex::new(db))
        .manage(tokio::sync::Mutex::new(plugin_manager))
        .invoke_handler(tauri::generate_handler![
            // Window commands
            start_dragging,
            set_window_position,
            get_window_position,
            // State commands
            get_state,
            set_state,
            // Event commands
            emit_event,
            subscribe_event,
            // Storage commands
            storage_get,
            storage_set,
            // Plugin commands
            plugin_list,
            plugin_enable,
            plugin_disable,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

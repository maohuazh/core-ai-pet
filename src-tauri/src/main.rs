#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod core;
mod services;

// Re-export infrastructure from lib
use core_ai_pet::infrastructure;

use tauri::{AppHandle, Manager};
use commands::auth::{get_current_user, login, logout, register};
use commands::event::{emit_event, subscribe_event};
use commands::llm::{
    llm_delete_secret, llm_get_secret, llm_invoke, llm_load_config, llm_save_config,
    llm_save_secret, llm_test_connection,
};
use commands::plugin::{plugin_disable, plugin_enable, plugin_list};
use commands::settings::{
    chat_create_session, chat_delete_session, chat_get_messages, chat_list_sessions,
    chat_store_message, chat_update_session, delete_action_mapping, delete_chat_platform,
    delete_email_account, delete_jira_connection, delete_model, disconnect_chat_platform,
    get_action_mappings, get_active_model_id, get_available_expressions, get_available_motions,
    get_chat_platforms, get_email_accounts, get_git_branch, get_jira_connections, get_models,
    open_chat_window, open_main_window, open_settings_window, save_action_mapping, set_active_model,
    toggle_chat_platform, toggle_email_account, toggle_jira_connection, update_email_account,
    update_jira_connection, update_model,
};
use commands::state::{get_state, set_state};
use commands::storage::{storage_get, storage_set};
use commands::window::{get_window_position, set_window_position, start_dragging};
use commands::window_native::set_click_through;

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
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .manage(tokio::sync::Mutex::new(state_machine))
        .manage(tokio::sync::Mutex::new(event_bus))
        .manage(tokio::sync::Mutex::new(db))
        .manage(tokio::sync::Mutex::new(plugin_manager))
        .invoke_handler(tauri::generate_handler![
            // Window commands
            start_dragging,
            set_window_position,
            get_window_position,
            set_click_through,
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
            // Settings - Jira commands
            get_jira_connections,
            toggle_jira_connection,
            delete_jira_connection,
            update_jira_connection,
            // Settings - Email commands
            get_email_accounts,
            toggle_email_account,
            delete_email_account,
            update_email_account,
            // Settings - Chat commands
            get_chat_platforms,
            toggle_chat_platform,
            disconnect_chat_platform,
            delete_chat_platform,
            // Settings - Model commands
            get_models,
            set_active_model,
            get_active_model_id,
            delete_model,
            update_model,
            // Settings - Action Mapping commands
            get_action_mappings,
            save_action_mapping,
            delete_action_mapping,
            // Settings - Resource Extraction commands
            get_available_motions,
            get_available_expressions,
            // Settings - Window commands
            open_settings_window,
            open_chat_window,
            open_main_window,
            chat_create_session,
            chat_list_sessions,
            chat_delete_session,
            chat_get_messages,
            chat_store_message,
            chat_update_session,
            get_git_branch,
            // LLM commands
            llm_load_config,
            llm_save_config,
            llm_get_secret,
            llm_save_secret,
            llm_delete_secret,
            llm_test_connection,
            llm_invoke,
            // Auth commands
            login,
            logout,
            get_current_user,
            register,
        ])
        .setup(|app| {
            // Apply native window styles for desktop pet behavior
            let handle = app.handle().clone();
            if let Some(window) = handle.get_webview_window("main") {
                let _ = commands::window_native::apply_layered(&window);
                let _ = commands::window_native::apply_no_activate(&window);
                let _ = window.set_shadow(false);

                // Enable initial click-through (window starts transparent to mouse)
                let _ = commands::window_native::window_set_click_through_pub(&window, true);

                log::info!("Desktop pet window styles applied");
            }

            // Start cursor position monitor for smart click-through
            commands::window_native::start_cursor_monitor(handle.clone());

            // Create system tray icon
            if let Err(e) = services::tray::create_tray(&handle) {
                log::error!("Failed to create tray icon: {}", e);
            }

            // Register global shortcut Ctrl+Alt+N to open chat window
            {
                use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};
                let h = handle.clone();
                let _ = app.global_shortcut().on_shortcut("Ctrl+Alt+N", move |_app, _shortcut, event| {
                    if event.state == ShortcutState::Pressed {
                        let h = h.clone();
                        tauri::async_runtime::spawn(async move {
                            let _ = commands::settings::open_chat_window(h).await;
                        });
                    }
                });
                log::info!("Global shortcut Ctrl+Alt+N registered");
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

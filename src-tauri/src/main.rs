#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod services;

use commands::window::{get_window_position, set_window_position, start_dragging};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            start_dragging,
            set_window_position,
            get_window_position
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

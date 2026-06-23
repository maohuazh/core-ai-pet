use serde::{Deserialize, Serialize};
use tauri::{
    image::Image, AppHandle, Manager, State, WebviewWindowBuilder,
};

use crate::infrastructure::storage::Database;

// === Data Structures ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraConnection {
    pub id: String,
    pub name: String,
    pub url: String,
    pub email: String,
    pub status: String,
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
    pub last_sync_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAccount {
    pub id: String,
    pub name: String,
    pub email: String,
    pub provider: String,
    pub status: String,
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
    pub last_sync_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatPlatform {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub status: String,
    pub enabled: bool,
    pub account_name: Option<String>,
    pub connected_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub model_type: String,
    pub path: String,
    pub manifest_path: Option<String>,
    pub model3_path: Option<String>,
    pub thumbnail: Option<String>,
    pub source: String,
    pub status: String,
    pub author: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub sort_order: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionMapping {
    pub id: String,
    pub model_id: String,
    pub trigger_key: String,
    pub motion_group: Option<String>,
    pub motion_name: Option<String>,
    pub expression_name: Option<String>,
    pub effect_name: Option<String>,
    pub use_default: bool,
    pub created_at: String,
    pub updated_at: String,
}

// === Jira Commands ===

#[tauri::command]
pub async fn get_jira_connections(
    db: State<'_, tokio::sync::Mutex<Database>>,
) -> Result<Vec<JiraConnection>, String> {
    let db = db.lock().await;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, name, url, email, status, enabled, created_at, updated_at, last_sync_at FROM jira_connections ORDER BY created_at DESC")
        .map_err(|e| e.to_string())?;

    let connections = stmt
        .query_map([], |row| {
            let enabled_int: i32 = row.get(5)?;
            Ok(JiraConnection {
                id: row.get(0)?,
                name: row.get(1)?,
                url: row.get(2)?,
                email: row.get(3)?,
                status: row.get(4)?,
                enabled: enabled_int != 0,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
                last_sync_at: row.get(8)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(connections)
}

#[tauri::command]
pub async fn toggle_jira_connection(
    db: State<'_, tokio::sync::Mutex<Database>>,
    id: String,
    enabled: bool,
) -> Result<(), String> {
    let db = db.lock().await;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE jira_connections SET enabled = ?1, updated_at = datetime('now') WHERE id = ?2",
        rusqlite::params![enabled as i32, id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn delete_jira_connection(
    db: State<'_, tokio::sync::Mutex<Database>>,
    id: String,
) -> Result<(), String> {
    let db = db.lock().await;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM jira_connections WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn update_jira_connection(
    db: State<'_, tokio::sync::Mutex<Database>>,
    id: String,
    name: String,
) -> Result<(), String> {
    let db = db.lock().await;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE jira_connections SET name = ?1, updated_at = datetime('now') WHERE id = ?2",
        rusqlite::params![name, id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

// === Email Commands ===

#[tauri::command]
pub async fn get_email_accounts(
    db: State<'_, tokio::sync::Mutex<Database>>,
) -> Result<Vec<EmailAccount>, String> {
    let db = db.lock().await;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, name, email, provider, status, enabled, created_at, updated_at, last_sync_at FROM email_accounts ORDER BY created_at DESC")
        .map_err(|e| e.to_string())?;

    let accounts = stmt
        .query_map([], |row| {
            let enabled_int: i32 = row.get(5)?;
            Ok(EmailAccount {
                id: row.get(0)?,
                name: row.get(1)?,
                email: row.get(2)?,
                provider: row.get(3)?,
                status: row.get(4)?,
                enabled: enabled_int != 0,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
                last_sync_at: row.get(8)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(accounts)
}

#[tauri::command]
pub async fn toggle_email_account(
    db: State<'_, tokio::sync::Mutex<Database>>,
    id: String,
    enabled: bool,
) -> Result<(), String> {
    let db = db.lock().await;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE email_accounts SET enabled = ?1, updated_at = datetime('now') WHERE id = ?2",
        rusqlite::params![enabled as i32, id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn delete_email_account(
    db: State<'_, tokio::sync::Mutex<Database>>,
    id: String,
) -> Result<(), String> {
    let db = db.lock().await;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM email_accounts WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn update_email_account(
    db: State<'_, tokio::sync::Mutex<Database>>,
    id: String,
    name: String,
) -> Result<(), String> {
    let db = db.lock().await;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE email_accounts SET name = ?1, updated_at = datetime('now') WHERE id = ?2",
        rusqlite::params![name, id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

// === Chat Commands ===

#[tauri::command]
pub async fn get_chat_platforms(
    db: State<'_, tokio::sync::Mutex<Database>>,
) -> Result<Vec<ChatPlatform>, String> {
    let db = db.lock().await;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, name, icon, status, enabled, account_name, connected_at, created_at, updated_at FROM chat_platforms ORDER BY enabled DESC, created_at ASC")
        .map_err(|e| e.to_string())?;

    let platforms = stmt
        .query_map([], |row| {
            let enabled_int: i32 = row.get(4)?;
            Ok(ChatPlatform {
                id: row.get(0)?,
                name: row.get(1)?,
                icon: row.get(2)?,
                status: row.get(3)?,
                enabled: enabled_int != 0,
                account_name: row.get(5)?,
                connected_at: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(platforms)
}

#[tauri::command]
pub async fn toggle_chat_platform(
    db: State<'_, tokio::sync::Mutex<Database>>,
    id: String,
    enabled: bool,
) -> Result<(), String> {
    let db = db.lock().await;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE chat_platforms SET enabled = ?1, updated_at = datetime('now') WHERE id = ?2",
        rusqlite::params![enabled as i32, id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn disconnect_chat_platform(
    db: State<'_, tokio::sync::Mutex<Database>>,
    id: String,
) -> Result<(), String> {
    let db = db.lock().await;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE chat_platforms SET status = 'disconnected', enabled = 0, account_name = NULL, connected_at = NULL, updated_at = datetime('now') WHERE id = ?1",
        rusqlite::params![id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn delete_chat_platform(
    db: State<'_, tokio::sync::Mutex<Database>>,
    id: String,
) -> Result<(), String> {
    let db = db.lock().await;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM chat_platforms WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

// === Model Commands ===

#[tauri::command]
pub async fn get_models(
    db: State<'_, tokio::sync::Mutex<Database>>,
) -> Result<Vec<Model>, String> {
    let db = db.lock().await;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, name, type, path, manifest_path, model3_path, thumbnail, source, status, author, version, description, license, sort_order, created_at, updated_at FROM models ORDER BY sort_order ASC, created_at ASC")
        .map_err(|e| e.to_string())?;

    let models = stmt
        .query_map([], |row| {
            Ok(Model {
                id: row.get(0)?,
                name: row.get(1)?,
                model_type: row.get(2)?,
                path: row.get(3)?,
                manifest_path: row.get(4)?,
                model3_path: row.get(5)?,
                thumbnail: row.get(6)?,
                source: row.get(7)?,
                status: row.get(8)?,
                author: row.get(9)?,
                version: row.get(10)?,
                description: row.get(11)?,
                license: row.get(12)?,
                sort_order: row.get(13)?,
                created_at: row.get(14)?,
                updated_at: row.get(15)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(models)
}

#[tauri::command]
pub async fn set_active_model(
    db: State<'_, tokio::sync::Mutex<Database>>,
    id: String,
) -> Result<(), String> {
    let db = db.lock().await;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // First, set all models to inactive
    conn.execute("UPDATE models SET status = 'inactive'", [])
        .map_err(|e| e.to_string())?;

    // Then set the specified model to active
    conn.execute(
        "UPDATE models SET status = 'active', updated_at = datetime('now') WHERE id = ?1",
        rusqlite::params![id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn delete_model(
    db: State<'_, tokio::sync::Mutex<Database>>,
    id: String,
) -> Result<(), String> {
    let db = db.lock().await;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM models WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn update_model(
    db: State<'_, tokio::sync::Mutex<Database>>,
    id: String,
    name: String,
) -> Result<(), String> {
    let db = db.lock().await;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE models SET name = ?1, updated_at = datetime('now') WHERE id = ?2",
        rusqlite::params![name, id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

// === Action Mapping Commands ===

#[tauri::command]
pub async fn get_action_mappings(
    db: State<'_, tokio::sync::Mutex<Database>>,
    model_id: String,
) -> Result<Vec<ActionMapping>, String> {
    let db = db.lock().await;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, model_id, trigger_key, motion_group, motion_name, expression_name, effect_name, use_default, created_at, updated_at FROM model_action_mappings WHERE model_id = ?1 ORDER BY trigger_key ASC")
        .map_err(|e| e.to_string())?;

    let mappings = stmt
        .query_map(rusqlite::params![model_id], |row| {
            let use_default_int: i32 = row.get(7)?;
            Ok(ActionMapping {
                id: row.get(0)?,
                model_id: row.get(1)?,
                trigger_key: row.get(2)?,
                motion_group: row.get(3)?,
                motion_name: row.get(4)?,
                expression_name: row.get(5)?,
                effect_name: row.get(6)?,
                use_default: use_default_int != 0,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(mappings)
}

#[tauri::command]
pub async fn save_action_mapping(
    db: State<'_, tokio::sync::Mutex<Database>>,
    model_id: String,
    trigger_key: String,
    motion_group: Option<String>,
    motion_name: Option<String>,
    expression_name: Option<String>,
    effect_name: Option<String>,
    use_default: bool,
) -> Result<(), String> {
    let db = db.lock().await;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Check if mapping exists
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM model_action_mappings WHERE model_id = ?1 AND trigger_key = ?2",
            rusqlite::params![model_id, trigger_key],
            |row| row.get::<_, i32>(0).map(|c| c > 0),
        )
        .map_err(|e| e.to_string())?;

    if exists {
        // Update existing mapping
        conn.execute(
            "UPDATE model_action_mappings SET motion_group = ?1, motion_name = ?2, expression_name = ?3, effect_name = ?4, use_default = ?5, updated_at = datetime('now') WHERE model_id = ?6 AND trigger_key = ?7",
            rusqlite::params![motion_group, motion_name, expression_name, effect_name, use_default as i32, model_id, trigger_key],
        )
        .map_err(|e| e.to_string())?;
    } else {
        // Insert new mapping
        let id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO model_action_mappings (id, model_id, trigger_key, motion_group, motion_name, expression_name, effect_name, use_default) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![id, model_id, trigger_key, motion_group, motion_name, expression_name, effect_name, use_default as i32],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub async fn delete_action_mapping(
    db: State<'_, tokio::sync::Mutex<Database>>,
    id: String,
) -> Result<(), String> {
    let db = db.lock().await;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM model_action_mappings WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

// === Window Management Commands ===

#[tauri::command]
pub async fn open_settings_window(app: AppHandle) -> Result<(), String> {
    // Check if settings window already exists
    if let Some(window) = app.get_webview_window("settings") {
        // Window exists, show and focus it
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    // Create new settings window
    let window = if let Some(icon) = load_app_icon(&app) {
        WebviewWindowBuilder::new(
            &app,
            "settings",
            tauri::WebviewUrl::App("/settings".into()),
        )
        .title("CoreAIpet - 设置")
        .inner_size(680.0, 720.0)
        .min_inner_size(560.0, 480.0)
        .decorations(false)
        .transparent(true)
        .always_on_top(false)
        .resizable(true)
        .icon(icon)
        .unwrap_or_else(|_| {
            WebviewWindowBuilder::new(
                &app,
                "settings",
                tauri::WebviewUrl::App("/settings".into()),
            )
            .title("CoreAIpet - 设置")
            .inner_size(680.0, 720.0)
            .min_inner_size(560.0, 480.0)
            .decorations(false)
            .transparent(true)
            .always_on_top(false)
            .resizable(true)
        })
        .build()
    } else {
        WebviewWindowBuilder::new(
            &app,
            "settings",
            tauri::WebviewUrl::App("/settings".into()),
        )
        .title("CoreAIpet - 设置")
        .inner_size(680.0, 720.0)
        .min_inner_size(560.0, 480.0)
        .decorations(false)
        .transparent(true)
        .always_on_top(false)
        .resizable(true)
        .build()
    }
    .map_err(|e| e.to_string())?;

    // Set up close event handler to hide instead of destroy
    let window_clone = window.clone();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            // Prevent the window from closing
            api.prevent_close();
            // Hide the window instead
            if let Err(e) = window_clone.hide() {
                log::error!("Failed to hide settings window: {}", e);
            }
        }
    });

    Ok(())
}

/// Load the app icon for use as window/taskbar icon.
fn load_app_icon(app: &AppHandle) -> Option<Image<'static>> {
    // Try resource dir (bundled app)
    if let Ok(resource_dir) = app.path().resource_dir() {
        for name in &["icons/logo.png", "icons/logo_256x256.png", "icons/logo_128x128.png"] {
            let path = resource_dir.join(name);
            if path.exists() {
                if let Ok(icon) = Image::from_path(&path) {
                    return Some(icon.to_owned());
                }
            }
        }
    }

    // Try current working directory (dev mode)
    for name in &["icons/logo.png", "icons/logo_256x256.png", "icons/logo_128x128.png"] {
        let path = std::path::Path::new(name);
        if path.exists() {
            if let Ok(icon) = Image::from_path(path) {
                return Some(icon.to_owned());
            }
        }
    }

    None
}

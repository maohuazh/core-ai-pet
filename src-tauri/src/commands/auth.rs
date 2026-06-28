use serde::{Deserialize, Serialize};
use tauri::{Manager, State};

use crate::infrastructure::storage::Database;

/// 用户信息（不含密码）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub avatar: Option<String>,
}

#[tauri::command]
pub async fn login(
    db: State<'_, tokio::sync::Mutex<Database>>,
    email: String,
    password: String,
) -> Result<UserProfile, String> {
    let db = db.lock().await;
    match db.verify_login(&email, &password) {
        Ok(Some(user)) => {
            // Save current session (user id) to app_settings
            {
                let conn = db.conn.lock().map_err(|e| e.to_string())?;
                conn.execute(
                    "INSERT INTO app_settings (key, value) VALUES ('current_user_id', ?1)
                     ON CONFLICT(key) DO UPDATE SET value = ?1, updated_at = datetime('now')",
                    rusqlite::params![user.id],
                )
                .map_err(|e| e.to_string())?;
            }
            Ok(UserProfile {
                id: user.id,
                first_name: user.first_name,
                last_name: user.last_name,
                email: user.email,
                avatar: user.avatar,
            })
        }
        Ok(None) => Err("邮箱或密码错误".to_string()),
        Err(e) => Err(format!("数据库错误: {}", e)),
    }
}

#[tauri::command]
pub async fn logout(
    db: State<'_, tokio::sync::Mutex<Database>>,
) -> Result<(), String> {
    let db = db.lock().await;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM app_settings WHERE key = 'current_user_id'", [])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_current_user(
    db: State<'_, tokio::sync::Mutex<Database>>,
) -> Result<Option<UserProfile>, String> {
    let db = db.lock().await;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Read current_user_id from app_settings
    let user_id: Option<String> = conn
        .prepare("SELECT value FROM app_settings WHERE key = 'current_user_id'")
        .map_err(|e| e.to_string())?
        .query_row([], |row| row.get(0))
        .ok();

    match user_id {
        Some(id) => {
            match db.find_user_by_id(&id) {
                Ok(Some(user)) => Ok(Some(UserProfile {
                    id: user.id,
                    first_name: user.first_name,
                    last_name: user.last_name,
                    email: user.email,
                    avatar: user.avatar,
                })),
                Ok(None) => Ok(None),
                Err(e) => Err(format!("数据库错误: {}", e)),
            }
        }
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn register(
    db: State<'_, tokio::sync::Mutex<Database>>,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
) -> Result<UserProfile, String> {
    let db = db.lock().await;

    // Check if email already exists
    match db.find_user_by_email(&email) {
        Ok(Some(_)) => return Err("该邮箱已被注册".to_string()),
        Ok(None) => {}
        Err(e) => return Err(format!("数据库错误: {}", e)),
    }

    let id = format!("user-{}", uuid_simple());

    db.create_user(&id, &first_name, &last_name, &email, &password, None)
        .map_err(|e| format!("数据库错误: {}", e))?;

    // Auto-login after registration
    {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT INTO app_settings (key, value) VALUES ('current_user_id', ?1)
             ON CONFLICT(key) DO UPDATE SET value = ?1, updated_at = datetime('now')",
            rusqlite::params![id],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(UserProfile {
        id,
        first_name,
        last_name,
        email,
        avatar: None,
    })
}

/// 生成简单唯一 ID（时间戳 + 随机后缀）
fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    format!("{:x}-{:x}", ts, ts >> 16)
}

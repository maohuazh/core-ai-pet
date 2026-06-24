use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

/// 数据库管理器
pub struct Database {
    pub conn: Mutex<Connection>,
}

/// 配置条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigEntry {
    pub key: String,
    pub value: String,
    pub updated_at: u64,
}

/// 聊天记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: Option<i64>,
    pub role: String,
    pub content: String,
    pub timestamp: u64,
    pub metadata: Option<String>,
}

/// 窗口位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowPosition {
    pub x: i32,
    pub y: i32,
    pub updated_at: u64,
}

/// 插件状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginState {
    pub plugin_id: String,
    pub enabled: bool,
    pub config: Option<String>,
    pub last_active: Option<u64>,
}

impl Database {
    /// 打开数据库连接，若数据库文件不存在则自动创建
    pub fn open(db_path: PathBuf) -> Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                rusqlite::Error::ToSqlConversionFailure(Box::new(e))
            })?;
        }

        let conn = Connection::open(&db_path)?;
        let db = Database {
            conn: Mutex::new(conn),
        };
        db.initialize()?;
        Ok(db)
    }

    /// 初始化数据库表结构
    fn initialize(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute_batch(
            "
            -- 配置存储
            CREATE TABLE IF NOT EXISTS config (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at INTEGER NOT NULL
            );

            -- 聊天记录
            CREATE TABLE IF NOT EXISTS chat_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                metadata TEXT
            );

            -- 窗口位置（单行表）
            CREATE TABLE IF NOT EXISTS window_position (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                x INTEGER NOT NULL,
                y INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );

            -- 插件状态
            CREATE TABLE IF NOT EXISTS plugin_state (
                plugin_id TEXT PRIMARY KEY,
                enabled INTEGER NOT NULL DEFAULT 0,
                config TEXT,
                last_active INTEGER
            );

            -- Jira 连接
            CREATE TABLE IF NOT EXISTS jira_connections (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                url TEXT NOT NULL,
                email TEXT NOT NULL,
                api_token TEXT,
                status TEXT NOT NULL DEFAULT 'connected' CHECK (status IN ('connected', 'expired', 'error')),
                enabled INTEGER NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now')),
                last_sync_at TEXT
            );

            -- 邮箱连接
            CREATE TABLE IF NOT EXISTS email_accounts (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                email TEXT NOT NULL,
                provider TEXT NOT NULL DEFAULT 'imap' CHECK (provider IN ('gmail', 'outlook', 'imap', 'other')),
                auth_type TEXT NOT NULL DEFAULT 'oauth2' CHECK (auth_type IN ('oauth2', 'app_password', 'imap_password')),
                auth_token TEXT,
                imap_host TEXT,
                imap_port INTEGER,
                smtp_host TEXT,
                smtp_port INTEGER,
                status TEXT NOT NULL DEFAULT 'connected' CHECK (status IN ('connected', 'expired', 'error')),
                enabled INTEGER NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now')),
                last_sync_at TEXT
            );

            -- 聊天工具连接
            CREATE TABLE IF NOT EXISTS chat_platforms (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                icon TEXT,
                status TEXT NOT NULL DEFAULT 'disconnected' CHECK (status IN ('connected', 'disconnected', 'error')),
                enabled INTEGER NOT NULL DEFAULT 0,
                account_name TEXT,
                auth_token TEXT,
                connected_at TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            -- 模型注册
            CREATE TABLE IF NOT EXISTS models (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                type TEXT NOT NULL DEFAULT 'live2d' CHECK (type IN ('live2d', 'sprite')),
                path TEXT NOT NULL,
                manifest_path TEXT,
                model3_path TEXT,
                thumbnail TEXT,
                source TEXT NOT NULL DEFAULT 'builtin' CHECK (source IN ('builtin', 'cdn', 'custom')),
                status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'inactive')),
                author TEXT,
                version TEXT,
                description TEXT,
                license TEXT,
                sort_order INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            -- 模型动作/表情映射
            CREATE TABLE IF NOT EXISTS model_action_mappings (
                id TEXT PRIMARY KEY,
                model_id TEXT NOT NULL REFERENCES models(id) ON DELETE CASCADE,
                trigger_key TEXT NOT NULL CHECK (trigger_key IN ('daily_1', 'daily_2', 'daily_3', 'new_message', 'new_task', 'new_email', 'task_in_progress', 'task_completed', 'task_approaching_deadline', 'task_overdue')),
                motion_group TEXT,
                motion_name TEXT,
                expression_name TEXT,
                effect_name TEXT,
                use_default INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now')),
                UNIQUE(model_id, trigger_key)
            );

            -- 应用全局设置（KV 存储）
            CREATE TABLE IF NOT EXISTS app_settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            -- 索引
            CREATE INDEX IF NOT EXISTS idx_jira_enabled ON jira_connections(enabled);
            CREATE INDEX IF NOT EXISTS idx_jira_status ON jira_connections(status);
            CREATE INDEX IF NOT EXISTS idx_email_enabled ON email_accounts(enabled);
            CREATE INDEX IF NOT EXISTS idx_email_status ON email_accounts(status);
            CREATE INDEX IF NOT EXISTS idx_email_provider ON email_accounts(provider);
            CREATE INDEX IF NOT EXISTS idx_chat_enabled ON chat_platforms(enabled);
            CREATE INDEX IF NOT EXISTS idx_chat_status ON chat_platforms(status);
            CREATE INDEX IF NOT EXISTS idx_models_type ON models(type);
            CREATE INDEX IF NOT EXISTS idx_models_status ON models(status);
            CREATE INDEX IF NOT EXISTS idx_models_source ON models(source);
            CREATE INDEX IF NOT EXISTS idx_action_mapping_model ON model_action_mappings(model_id);
            CREATE INDEX IF NOT EXISTS idx_action_mapping_trigger ON model_action_mappings(trigger_key);
            ",
        )?;

        // Migration: add type and manifest_path columns to models table if they don't exist
        // (for databases created before these columns were added)
        Self::run_migrations(&conn)?;

        // Initialize mock data if tables are empty
        Self::initialize_mock_data(&conn)?;

        log::info!("Database initialized with all tables");
        Ok(())
    }

    /// Run migrations for existing databases
    fn run_migrations(conn: &Connection) -> Result<()> {
        // Check if 'type' column exists in models table
        let has_type: bool = conn
            .prepare("PRAGMA table_info(models)")?
            .query_map([], |row| {
                let name: String = row.get(1)?;
                Ok(name == "type")
            })?
            .any(|r| r.unwrap_or(false));

        if !has_type {
            conn.execute_batch(
                "ALTER TABLE models ADD COLUMN type TEXT NOT NULL DEFAULT 'live2d';
                 ALTER TABLE models ADD COLUMN manifest_path TEXT;",
            )?;
            log::info!("Migration: added type and manifest_path columns to models table");
        }

        Ok(())
    }

    /// 初始化 Mock 数据（仅在表为空时插入）
    fn initialize_mock_data(conn: &Connection) -> Result<()> {

        // Check if jira_connections is empty
        let jira_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM jira_connections",
            [],
            |row| row.get(0),
        )?;

        if jira_count == 0 {
            conn.execute_batch(
                "INSERT INTO jira_connections (id, name, url, email, status, enabled, last_sync_at) VALUES
                ('jira-mock-001', '公司项目管理', 'https://mycompany.atlassian.net', 'zhangsan@mycompany.com', 'connected', 1, '2026-06-20T09:30:00Z'),
                ('jira-mock-002', '开源项目追踪', 'https://opensource.atlassian.net', 'zhangsan@gmail.com', 'expired', 0, '2026-05-01T14:00:00Z');"
            )?;
            log::info!("Initialized Jira mock data");
        }

        // Check if email_accounts is empty
        let email_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM email_accounts",
            [],
            |row| row.get(0),
        )?;

        if email_count == 0 {
            conn.execute_batch(
                "INSERT INTO email_accounts (id, name, email, provider, status, enabled, last_sync_at) VALUES
                ('email-mock-001', '工作邮箱', 'zhangsan@mycompany.com', 'outlook', 'connected', 1, '2026-06-20T10:00:00Z'),
                ('email-mock-002', '个人邮箱', 'zhangsan@gmail.com', 'gmail', 'connected', 1, '2026-06-20T09:45:00Z');"
            )?;
            log::info!("Initialized Email mock data");
        }

        // Check if chat_platforms is empty
        let chat_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM chat_platforms",
            [],
            |row| row.get(0),
        )?;

        if chat_count == 0 {
            conn.execute_batch(
                "INSERT INTO chat_platforms (id, name, icon, status, enabled, account_name, connected_at) VALUES
                ('chat-wechat', 'WeChat', '💬', 'connected', 1, '张三', '2026-05-15T10:30:00Z'),
                ('chat-slack', 'Slack', '💼', 'connected', 1, 'zhangsan@company.com', '2026-04-20T14:00:00Z'),
                ('chat-teams', 'Microsoft Teams', '👥', 'disconnected', 0, NULL, NULL),
                ('chat-discord', 'Discord', '🎮', 'disconnected', 0, NULL, NULL);"
            )?;
            log::info!("Initialized Chat mock data");
        }

        Ok(())
    }

    // === Config 操作 ===

    /// 获取配置值
    pub fn storage_get(&self, key: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM config WHERE key = ?1")?;
        let mut rows = stmt.query(params![key])?;

        match rows.next()? {
            Some(row) => Ok(Some(row.get(0)?)),
            None => Ok(None),
        }
    }

    /// 设置配置值
    pub fn storage_set(&self, key: &str, value: &str) -> Result<()> {
        let now = current_timestamp();
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO config (key, value, updated_at) VALUES (?1, ?2, ?3)
             ON CONFLICT(key) DO UPDATE SET value = ?2, updated_at = ?3",
            params![key, value, now],
        )?;
        Ok(())
    }

    // === Chat History 操作 ===

    /// 存储聊天消息
    pub fn chat_store(&self, role: &str, content: &str, metadata: Option<&str>) -> Result<i64> {
        let now = current_timestamp();
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO chat_history (role, content, timestamp, metadata) VALUES (?1, ?2, ?3, ?4)",
            params![role, content, now, metadata],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// 获取聊天记录列表（按时间倒序）
    pub fn chat_list(&self, limit: i64, offset: i64) -> Result<Vec<ChatMessage>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, role, content, timestamp, metadata FROM chat_history
             ORDER BY timestamp DESC LIMIT ?1 OFFSET ?2",
        )?;
        let rows = stmt.query_map(params![limit, offset], |row| {
            Ok(ChatMessage {
                id: Some(row.get(0)?),
                role: row.get(1)?,
                content: row.get(2)?,
                timestamp: row.get(3)?,
                metadata: row.get(4)?,
            })
        })?;

        let mut messages = Vec::new();
        for row in rows {
            messages.push(row?);
        }
        Ok(messages)
    }

    // === Window Position 操作 ===

    /// 保存窗口位置
    pub fn save_window_position(&self, x: i32, y: i32) -> Result<()> {
        let now = current_timestamp();
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO window_position (id, x, y, updated_at) VALUES (1, ?1, ?2, ?3)
             ON CONFLICT(id) DO UPDATE SET x = ?1, y = ?2, updated_at = ?3",
            params![x, y, now],
        )?;
        Ok(())
    }

    /// 恢复窗口位置
    pub fn load_window_position(&self) -> Result<Option<WindowPosition>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT x, y, updated_at FROM window_position WHERE id = 1")?;
        let mut rows = stmt.query(params![])?;

        match rows.next()? {
            Some(row) => Ok(Some(WindowPosition {
                x: row.get(0)?,
                y: row.get(1)?,
                updated_at: row.get(2)?,
            })),
            None => Ok(None),
        }
    }

    // === Plugin State 操作 ===

    /// 保存插件状态
    pub fn save_plugin_state(&self, state: &PluginState) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO plugin_state (plugin_id, enabled, config, last_active)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(plugin_id) DO UPDATE SET enabled = ?2, config = ?3, last_active = ?4",
            params![
                state.plugin_id,
                state.enabled as i32,
                state.config,
                state.last_active,
            ],
        )?;
        Ok(())
    }

    /// 获取插件状态
    pub fn load_plugin_state(&self, plugin_id: &str) -> Result<Option<PluginState>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT plugin_id, enabled, config, last_active FROM plugin_state WHERE plugin_id = ?1",
        )?;
        let mut rows = stmt.query(params![plugin_id])?;

        match rows.next()? {
            Some(row) => {
                let enabled_int: i32 = row.get(1)?;
                Ok(Some(PluginState {
                    plugin_id: row.get(0)?,
                    enabled: enabled_int != 0,
                    config: row.get(2)?,
                    last_active: row.get(3)?,
                }))
            }
            None => Ok(None),
        }
    }

    /// 获取所有插件状态
    pub fn load_all_plugin_states(&self) -> Result<Vec<PluginState>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT plugin_id, enabled, config, last_active FROM plugin_state")?;
        let rows = stmt.query_map(params![], |row| {
            let enabled_int: i32 = row.get(1)?;
            Ok(PluginState {
                plugin_id: row.get(0)?,
                enabled: enabled_int != 0,
                config: row.get(2)?,
                last_active: row.get(3)?,
            })
        })?;

        let mut states = Vec::new();
        for row in rows {
            states.push(row?);
        }
        Ok(states)
    }
}

/// 获取当前 Unix 时间戳（毫秒）
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

/// 获取默认数据库路径
pub fn default_db_path() -> PathBuf {
    let app_data = std::env::var("APPDATA")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());

    PathBuf::from(app_data)
        .join("CoreAIpet")
        .join("data.db")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn test_db() -> (Database, PathBuf) {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let path = PathBuf::from(format!("test_db_{}.sqlite", id));
        let _ = fs::remove_file(&path);
        (Database::open(path.clone()).unwrap(), path)
    }

    #[test]
    fn test_database_creation() {
        let (_db, path) = test_db();
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_storage_get_set() {
        let (db, path) = test_db();
        db.storage_set("theme", "dark").unwrap();
        let value = db.storage_get("theme").unwrap();
        assert_eq!(value, Some("dark".to_string()));

        // Update value
        db.storage_set("theme", "light").unwrap();
        let value = db.storage_get("theme").unwrap();
        assert_eq!(value, Some("light".to_string()));

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_storage_get_nonexistent() {
        let (db, path) = test_db();
        let value = db.storage_get("nonexistent").unwrap();
        assert_eq!(value, None);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_chat_store_and_list() {
        let (db, path) = test_db();
        db.chat_store("user", "Hello", None).unwrap();
        db.chat_store("assistant", "Hi there!", Some("{\"model\":\"gpt4\"}"))
            .unwrap();

        let messages = db.chat_list(10, 0).unwrap();
        assert_eq!(messages.len(), 2);
        // Most recent first
        assert_eq!(messages[0].role, "assistant");
        assert_eq!(messages[1].role, "user");
        assert_eq!(messages[0].metadata, Some("{\"model\":\"gpt4\"}".to_string()));

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_window_position() {
        let (db, path) = test_db();

        // Initially no position
        let pos = db.load_window_position().unwrap();
        assert!(pos.is_none());

        // Save position
        db.save_window_position(100, 200).unwrap();
        let pos = db.load_window_position().unwrap().unwrap();
        assert_eq!(pos.x, 100);
        assert_eq!(pos.y, 200);

        // Update position
        db.save_window_position(300, 400).unwrap();
        let pos = db.load_window_position().unwrap().unwrap();
        assert_eq!(pos.x, 300);
        assert_eq!(pos.y, 400);

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_plugin_state() {
        let (db, path) = test_db();

        let state = PluginState {
            plugin_id: "test.plugin".to_string(),
            enabled: true,
            config: Some("{\"key\":\"value\"}".to_string()),
            last_active: Some(12345),
        };

        db.save_plugin_state(&state).unwrap();

        let loaded = db.load_plugin_state("test.plugin").unwrap().unwrap();
        assert_eq!(loaded.plugin_id, "test.plugin");
        assert!(loaded.enabled);
        assert_eq!(loaded.config, Some("{\"key\":\"value\"}".to_string()));

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_load_all_plugin_states() {
        let (db, path) = test_db();

        let state1 = PluginState {
            plugin_id: "plugin.a".to_string(),
            enabled: true,
            config: None,
            last_active: None,
        };
        let state2 = PluginState {
            plugin_id: "plugin.b".to_string(),
            enabled: false,
            config: None,
            last_active: None,
        };

        db.save_plugin_state(&state1).unwrap();
        db.save_plugin_state(&state2).unwrap();

        let all = db.load_all_plugin_states().unwrap();
        assert_eq!(all.len(), 2);

        let _ = fs::remove_file(&path);
    }
}

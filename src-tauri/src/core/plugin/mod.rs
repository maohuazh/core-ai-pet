use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::infrastructure::storage::{Database, PluginState};

/// 插件配置（plugin.json）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub config: Option<serde_json::Value>,
}

/// 运行时插件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub enabled: bool,
    pub description: Option<String>,
}

/// 插件管理器
pub struct PluginManager {
    plugins: HashMap<String, PluginConfig>,
    plugins_dir: PathBuf,
}

impl PluginManager {
    pub fn new(plugins_dir: PathBuf) -> Self {
        PluginManager {
            plugins: HashMap::new(),
            plugins_dir,
        }
    }

    /// 扫描 plugins/ 目录并加载所有插件配置
    pub fn load_plugins(&mut self) -> Result<(), String> {
        // Create plugins directory if it doesn't exist
        if !self.plugins_dir.exists() {
            std::fs::create_dir_all(&self.plugins_dir)
                .map_err(|e| format!("Failed to create plugins directory: {}", e))?;
            log::info!("Created plugins directory: {:?}", self.plugins_dir);
        }

        let entries = std::fs::read_dir(&self.plugins_dir)
            .map_err(|e| format!("Failed to read plugins directory: {}", e))?;

        for entry in entries {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    log::error!("Failed to read directory entry: {}", e);
                    continue;
                }
            };

            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let plugin_json = path.join("plugin.json");
            if !plugin_json.exists() {
                log::warn!("No plugin.json found in {:?}, skipping", path);
                continue;
            }

            match self.load_plugin_from_file(&plugin_json) {
                Ok(config) => {
                    log::info!("Loaded plugin: {} v{}", config.name, config.version);
                    self.plugins.insert(config.id.clone(), config);
                }
                Err(e) => {
                    log::error!("Failed to load plugin from {:?}: {}", plugin_json, e);
                    // Skip invalid plugin, don't stop loading others
                }
            }
        }

        log::info!("Loaded {} plugins", self.plugins.len());
        Ok(())
    }

    /// 从 plugin.json 文件加载单个插件
    fn load_plugin_from_file(&self, path: &Path) -> Result<PluginConfig, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        let config: PluginConfig = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse plugin.json: {}", e))?;

        // Validate required fields
        if config.id.is_empty() {
            return Err("Plugin id is empty".to_string());
        }
        if config.name.is_empty() {
            return Err("Plugin name is empty".to_string());
        }
        if config.version.is_empty() {
            return Err("Plugin version is empty".to_string());
        }

        Ok(config)
    }

    /// 启用插件
    pub fn enable_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        let plugin = self
            .plugins
            .get_mut(plugin_id)
            .ok_or_else(|| format!("Plugin not found: {}", plugin_id))?;

        plugin.enabled = true;
        log::info!("Plugin enabled: {}", plugin_id);
        Ok(())
    }

    /// 禁用插件
    pub fn disable_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        let plugin = self
            .plugins
            .get_mut(plugin_id)
            .ok_or_else(|| format!("Plugin not found: {}", plugin_id))?;

        plugin.enabled = false;
        log::info!("Plugin disabled: {}", plugin_id);
        Ok(())
    }

    /// 获取所有插件列表
    pub fn list_plugins(&self) -> Vec<PluginInfo> {
        self.plugins
            .values()
            .map(|p| PluginInfo {
                id: p.id.clone(),
                name: p.name.clone(),
                version: p.version.clone(),
                enabled: p.enabled,
                description: p.description.clone(),
            })
            .collect()
    }

    /// 获取单个插件信息
    pub fn get_plugin(&self, plugin_id: &str) -> Option<&PluginConfig> {
        self.plugins.get(plugin_id)
    }

    /// 将插件状态持久化到 SQLite
    pub fn persist_states(&self, db: &Database) -> Result<(), String> {
        for (id, plugin) in &self.plugins {
            let state = PluginState {
                plugin_id: id.clone(),
                enabled: plugin.enabled,
                config: plugin.config.as_ref().map(|c| c.to_string()),
                last_active: None,
            };
            db.save_plugin_state(&state)
                .map_err(|e| format!("Failed to persist plugin state for {}: {}", id, e))?;
        }
        Ok(())
    }

    /// 从 SQLite 恢复插件启用状态
    pub fn restore_states(&mut self, db: &Database) -> Result<(), String> {
        let states = db
            .load_all_plugin_states()
            .map_err(|e| format!("Failed to load plugin states: {}", e))?;

        for state in states {
            if let Some(plugin) = self.plugins.get_mut(&state.plugin_id) {
                plugin.enabled = state.enabled;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn test_plugin_dir() -> PathBuf {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        PathBuf::from(format!("test_plugins_{}", id))
    }

    #[test]
    fn test_load_empty_plugins_dir() {
        let dir = test_plugin_dir();
        let mut pm = PluginManager::new(dir.clone());
        pm.load_plugins().unwrap();
        assert_eq!(pm.list_plugins().len(), 0);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_load_valid_plugin() {
        let dir = test_plugin_dir();
        let plugin_dir = dir.join("test-plugin");
        fs::create_dir_all(&plugin_dir).unwrap();
        fs::write(
            plugin_dir.join("plugin.json"),
            r#"{
                "id": "test.plugin",
                "name": "Test Plugin",
                "version": "1.0.0",
                "enabled": true,
                "description": "A test plugin"
            }"#,
        )
        .unwrap();

        let mut pm = PluginManager::new(dir.clone());
        pm.load_plugins().unwrap();

        let plugins = pm.list_plugins();
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].id, "test.plugin");
        assert_eq!(plugins[0].name, "Test Plugin");
        assert!(plugins[0].enabled);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_skip_invalid_plugin() {
        let dir = test_plugin_dir();
        let plugin_dir = dir.join("bad-plugin");
        fs::create_dir_all(&plugin_dir).unwrap();
        fs::write(plugin_dir.join("plugin.json"), "not json").unwrap();

        let mut pm = PluginManager::new(dir.clone());
        // Should not fail, just skip the bad plugin
        pm.load_plugins().unwrap();
        assert_eq!(pm.list_plugins().len(), 0);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_enable_disable_plugin() {
        let dir = test_plugin_dir();
        let plugin_dir = dir.join("test-plugin");
        fs::create_dir_all(&plugin_dir).unwrap();
        fs::write(
            plugin_dir.join("plugin.json"),
            r#"{
                "id": "test.plugin",
                "name": "Test Plugin",
                "version": "1.0.0",
                "enabled": true
            }"#,
        )
        .unwrap();

        let mut pm = PluginManager::new(dir.clone());
        pm.load_plugins().unwrap();

        // Disable
        pm.disable_plugin("test.plugin").unwrap();
        assert!(!pm.get_plugin("test.plugin").unwrap().enabled);

        // Enable
        pm.enable_plugin("test.plugin").unwrap();
        assert!(pm.get_plugin("test.plugin").unwrap().enabled);

        // Non-existent plugin
        assert!(pm.enable_plugin("nonexistent").is_err());

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_list_plugins() {
        let dir = test_plugin_dir();

        // Create two plugins
        for (name, id) in [("plugin-a", "a"), ("plugin-b", "b")] {
            let plugin_dir = dir.join(name);
            fs::create_dir_all(&plugin_dir).unwrap();
            fs::write(
                plugin_dir.join("plugin.json"),
                format!(
                    r#"{{"id": "{}", "name": "{}", "version": "1.0.0", "enabled": true}}"#,
                    id, name
                ),
            )
            .unwrap();
        }

        let mut pm = PluginManager::new(dir.clone());
        pm.load_plugins().unwrap();
        assert_eq!(pm.list_plugins().len(), 2);

        let _ = fs::remove_dir_all(&dir);
    }
}

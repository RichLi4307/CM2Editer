//! Simple application settings persistence.
//!
//! Stores editor preferences (currently only UI language) in the user's config
//! directory so they survive between runs.

use std::path::PathBuf;

use crate::error::{FlowError, Result};

/// Serializable application settings.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AppSettings {
    #[serde(default)]
    pub language: String,
    /// 最近使用的节点类型，以 NodeType 的 PascalCase 名称存储。
    #[serde(default)]
    pub recent_node_types: Vec<String>,
}

impl AppSettings {
    /// Path to the settings file in the user's config directory.
    pub fn path() -> Option<PathBuf> {
        config_dir().map(|dir| dir.join("CM2Editer").join("settings.json"))
    }

    /// Load settings from disk, returning defaults on any error.
    pub fn load() -> Self {
        Self::load_impl().unwrap_or_default()
    }

    fn load_impl() -> Result<Self> {
        let path = Self::path().ok_or(FlowError::Io("no config directory".into()))?;
        let text = std::fs::read_to_string(&path).map_err(FlowError::from)?;
        serde_json::from_str(&text).map_err(FlowError::from)
    }

    /// Save settings to disk, creating parent directories if needed.
    pub fn save(&self) -> Result<()> {
        let path = Self::path().ok_or(FlowError::Io("no config directory".into()))?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(FlowError::from)?;
        }
        let text = serde_json::to_string_pretty(self).map_err(FlowError::from)?;
        std::fs::write(&path, text).map_err(FlowError::from)
    }
}

/// Returns the user's config directory on Windows, macOS and Linux.
fn config_dir() -> Option<PathBuf> {
    // On Windows, APPDATA is the standard roaming config directory.
    if let Some(app_data) = std::env::var_os("APPDATA") {
        return Some(PathBuf::from(app_data));
    }
    // Fallback to XDG_CONFIG_HOME / ~/.config on Unix-like systems.
    if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(xdg));
    }
    #[cfg(not(target_os = "windows"))]
    if let Some(home) = std::env::var_os("HOME") {
        return Some(PathBuf::from(home).join(".config"));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_roundtrip() {
        let settings = AppSettings {
            language: "zh".to_string(),
            recent_node_types: vec!["Log".to_string(), "If".to_string()],
        };
        // We can't easily write to the real config dir in tests, so just verify
        // serialization/deserialization.
        let json = serde_json::to_string(&settings).unwrap();
        let loaded: AppSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.language, "zh");
        assert_eq!(loaded.recent_node_types, vec!["Log", "If"]);
    }

    #[test]
    fn test_settings_default_language_is_empty() {
        let settings = AppSettings::default();
        assert_eq!(settings.language, "");
    }
}

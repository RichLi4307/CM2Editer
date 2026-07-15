//! Lightweight internationalization (i18n) support for the editor UI.
//!
//! Translations are loaded from JSON files under `assets/i18n/`. Each file is a
//! flat map of `key -> text`. The current language falls back to English (`en`)
//! when a key is missing, and finally returns the key itself for easy debugging.

use std::collections::HashMap;
use std::path::Path;

use crate::error::{FlowError, Result};

/// In-memory translation registry.
#[derive(Debug, Clone, Default)]
pub struct I18n {
    current: String,
    translations: HashMap<String, HashMap<String, String>>,
}

impl I18n {
    /// Create an empty registry with English as the fallback language.
    pub fn new() -> Self {
        Self {
            current: "en".to_string(),
            translations: HashMap::new(),
        }
    }

    /// Load all `.json` translation files from a directory.
    ///
    /// Files are keyed by their stem (e.g. `zh.json` -> language `zh`).
    pub fn load_from_dir(&mut self, dir: &Path) -> Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }

        for entry in std::fs::read_dir(dir).map_err(FlowError::from)? {
            let entry = entry.map_err(FlowError::from)?;
            let path = entry.path();
            if path
                .extension()
                .map(|ext| ext.eq_ignore_ascii_case("json"))
                .unwrap_or(false)
            {
                let lang = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default()
                    .to_string();
                let text = std::fs::read_to_string(&path).map_err(FlowError::from)?;
                let map: HashMap<String, String> =
                    serde_json::from_str(&text).map_err(FlowError::from)?;
                self.translations.insert(lang, map);
            }
        }
        Ok(())
    }

    /// Load bundled translations from the default asset location.
    pub fn load_bundled() -> Self {
        let mut i18n = Self::new();
        for dir in [std::path::PathBuf::from("assets/i18n")] {
            let _ = i18n.load_from_dir(&dir);
        }
        i18n
    }

    /// Switch the active language. If the language is not loaded, fallbacks
    /// will still work through English.
    pub fn set_language(&mut self, lang: &str) {
        self.current = lang.to_string();
    }

    /// Current language code.
    pub fn current_language(&self) -> &str {
        &self.current
    }

    /// Translate a key into the current language, falling back to English and
    /// then to the key itself.
    pub fn text(&self, key: &str) -> String {
        self.text_opt(key).unwrap_or_else(|| key.to_string())
    }

    /// Translate a key, returning `None` if no translation is available.
    fn text_opt(&self, key: &str) -> Option<String> {
        if let Some(map) = self.translations.get(&self.current) {
            if let Some(value) = map.get(key) {
                return Some(value.clone());
            }
        }
        if let Some(map) = self.translations.get("en") {
            if let Some(value) = map.get(key) {
                return Some(value.clone());
            }
        }
        None
    }

    /// Format a translated string by replacing `{}` placeholders in order.
    pub fn format(&self, key: &str, args: &[&str]) -> String {
        let mut s = self.text(key);
        for arg in args {
            s = s.replacen("{}", arg, 1);
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i18n_fallback_to_en() {
        let mut i18n = I18n::new();
        let mut map = HashMap::new();
        map.insert("hello".to_string(), "Hello".to_string());
        i18n.translations.insert("en".to_string(), map);
        i18n.set_language("zh");
        assert_eq!(i18n.text("hello"), "Hello");
    }

    #[test]
    fn test_i18n_current_language_wins() {
        let mut i18n = I18n::new();
        let mut en = HashMap::new();
        en.insert("hello".to_string(), "Hello".to_string());
        let mut zh = HashMap::new();
        zh.insert("hello".to_string(), "你好".to_string());
        i18n.translations.insert("en".to_string(), en);
        i18n.translations.insert("zh".to_string(), zh);
        i18n.set_language("zh");
        assert_eq!(i18n.text("hello"), "你好");
    }

    #[test]
    fn test_i18n_format() {
        let mut i18n = I18n::new();
        let mut map = HashMap::new();
        map.insert("saved".to_string(), "Saved {}".to_string());
        i18n.translations.insert("en".to_string(), map);
        assert_eq!(i18n.format("saved", &["proj"]), "Saved proj");
    }
}

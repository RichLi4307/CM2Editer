//! Namespace registry for game-specific enumerations that are too large or
//! too dynamic to hard-code in [`crate::api::enums`].
//!
//! Namespaces are loaded from JSON files under `assets/namespaces/` at
//! application startup. Each file describes a namespace such as `cosplay`,
//! `adult_toy`, or `vibrator` and contains a list of keys with translated
//! display names.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::error::FlowError;

/// A single entry inside a namespace.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NamespaceEntry {
    pub key: String,
    /// Optional secondary category (e.g. "头部", "上装" for cosplay).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Optional translated display names keyed by language code (`en`, `zh`, ...).
    /// When a requested language is missing, the entry key is shown instead.
    pub names: HashMap<String, String>,
}

impl NamespaceEntry {
    /// Create a simple entry where the key is also the only name.
    pub fn new(key: impl Into<String>) -> Self {
        let key = key.into();
        Self {
            key,
            category: None,
            names: HashMap::new(),
        }
    }

    /// Set the category for this entry.
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Display name for the requested language.
    pub fn display_name(&self, lang: &str) -> String {
        self.names
            .get(lang)
            .cloned()
            .or_else(|| self.names.get("en").cloned())
            .unwrap_or_else(|| self.key.clone())
    }

    /// Add or update a translation.
    pub fn with_name(mut self, lang: impl Into<String>, name: impl Into<String>) -> Self {
        self.names.insert(lang.into(), name.into());
        self
    }
}

/// A namespace groups related entries (e.g. all cosplay items).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Namespace {
    pub name: String,
    #[serde(default)]
    pub entries: Vec<NamespaceEntry>,
}

impl Namespace {
    /// Create a namespace from a list of entries.
    pub fn new(name: impl Into<String>, entries: Vec<NamespaceEntry>) -> Self {
        Self {
            name: name.into(),
            entries,
        }
    }

    /// Find an entry by its key.
    pub fn get(&self, key: &str) -> Option<&NamespaceEntry> {
        self.entries.iter().find(|e| e.key == key)
    }

    /// Search entries by key or translated name (case-insensitive).
    pub fn search(&self, query: &str, lang: &str) -> Vec<&NamespaceEntry> {
        let query = query.to_lowercase();
        self.entries
            .iter()
            .filter(|e| {
                e.key.to_lowercase().contains(&query)
                    || e.display_name(lang).to_lowercase().contains(&query)
            })
            .collect()
    }
}

/// On-disk representation of a namespace file.
#[derive(Debug, Clone, Deserialize)]
struct NamespaceFile {
    name: String,
    #[serde(default)]
    entries: HashMap<String, NamespaceEntryFile>,
}

/// A single entry as it appears in a JSON file.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum NamespaceEntryFile {
    /// Simple form: "key": "Display Name" (stored as English name).
    Simple(String),
    /// Full form with optional category.
    Full {
        #[serde(default)]
        category: Option<String>,
        #[serde(flatten)]
        names: HashMap<String, String>,
    },
}

/// Global registry holding all loaded namespaces.
#[derive(Debug, Clone, Default)]
pub struct NamespaceRegistry {
    namespaces: HashMap<String, Namespace>,
}

impl NamespaceRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a namespace.
    pub fn register(&mut self, namespace: Namespace) {
        self.namespaces.insert(namespace.name.clone(), namespace);
    }

    /// Get a namespace by name.
    pub fn get(&self, name: &str) -> Option<&Namespace> {
        self.namespaces.get(name)
    }

    /// All registered namespace names.
    pub fn namespace_names(&self) -> Vec<&String> {
        self.namespaces.keys().collect()
    }

    /// Load all `.json` namespace files from a directory and register them.
    ///
    /// Errors are logged via [`eprintln!`] and skipped so that a single bad
    /// file does not prevent the editor from starting.
    pub fn load_from_dir(&mut self, dir: &std::path::Path) {
        if !dir.is_dir() {
            return;
        }

        let mut entries: Vec<_> = match std::fs::read_dir(dir) {
            Ok(iter) => iter
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .extension()
                        .map(|ext| ext.eq_ignore_ascii_case("json"))
                        .unwrap_or(false)
                })
                .map(|e| e.path())
                .collect(),
            Err(err) => {
                eprintln!("failed to read namespace dir {:?}: {}", dir, err);
                return;
            }
        };
        entries.sort();

        for path in entries {
            match load_namespace_file(&path) {
                Ok(namespace) => {
                    self.register(namespace);
                }
                Err(err) => eprintln!("failed to load namespace {:?}: {}", path, err),
            }
        }
    }

    /// Load bundled namespaces from the default asset location.
    pub fn load_bundled() -> Self {
        let mut registry = Self::new();
        // Try both the development workspace layout and a sibling-of-executable layout.
        let candidates = [
            std::path::PathBuf::from("assets/namespaces"),
            std::path::PathBuf::from("namespaces"),
        ];
        for dir in &candidates {
            registry.load_from_dir(dir);
        }
        registry
    }

    /// Remove an entry from the namespace identified by `namespace` and persist the
    /// change back to the JSON file under `dir` that owns the namespace.
    pub fn remove_entry(&mut self, namespace: &str, key: &str, dir: &Path) -> crate::error::Result<()> {
        if !dir.is_dir() {
            return Err(FlowError::Io(format!("namespace directory not found: {}", dir.display())));
        }

        let mut removed = false;
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path
                .extension()
                .map(|ext| ext.eq_ignore_ascii_case("json"))
                .unwrap_or(false)
                && remove_namespace_entry_from_file(&path, namespace, key)?
            {
                removed = true;
                break;
            }
        }

        if removed {
            if let Some(ns) = self.namespaces.get_mut(namespace) {
                ns.entries.retain(|e| e.key != key);
                if ns.entries.is_empty() {
                    self.namespaces.remove(namespace);
                }
            }
            Ok(())
        } else {
            Err(FlowError::Validation(format!(
                "namespace entry not found: {}.{}",
                namespace, key
            )))
        }
    }
}

fn remove_namespace_entry_from_file(path: &Path, namespace: &str, key: &str) -> crate::error::Result<bool> {
    let text = std::fs::read_to_string(path)?;
    let mut value: serde_json::Value = serde_json::from_str(&text)?;
    let name = value
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if name != namespace {
        return Ok(false);
    }

    let entries = value
        .get_mut("entries")
        .and_then(|v| v.as_object_mut());
    if let Some(entries) = entries {
        if entries.remove(key).is_some() {
            std::fs::write(path, serde_json::to_string_pretty(&value)?)?;
            return Ok(true);
        }
    }
    Ok(false)
}

fn load_namespace_file(path: &std::path::Path) -> Result<Namespace, Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string(path)?;
    let file: NamespaceFile = serde_json::from_str(&text)?;
    let mut entries = Vec::with_capacity(file.entries.len());
    for (key, entry_file) in file.entries {
        let entry = match entry_file {
            NamespaceEntryFile::Simple(name) => {
                NamespaceEntry::new(key.clone()).with_name("en", name)
            }
            NamespaceEntryFile::Full { category, names } => NamespaceEntry {
                key,
                category,
                names,
            },
        };
        entries.push(entry);
    }
    entries.sort_by(|a, b| a.key.cmp(&b.key));
    Ok(Namespace::new(file.name, entries))
}

/// Load a namespace from a simple JSON object containing only a `selected`
/// array of keys. Each key is used as its own English display name.
#[allow(dead_code)]
pub fn load_simple_selected(
    namespace: &str,
    text: &str,
) -> Result<Namespace, Box<dyn std::error::Error>> {
    #[derive(Deserialize)]
    struct SelectedFile {
        selected: Vec<String>,
    }
    let file: SelectedFile = serde_json::from_str(text)?;
    let entries = file.selected.into_iter().map(NamespaceEntry::new).collect();
    Ok(Namespace::new(namespace, entries))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_entry_display_name() {
        let entry = NamespaceEntry::new("Maid")
            .with_name("en", "Maid Uniform")
            .with_name("zh", "女仆装");
        assert_eq!(entry.display_name("zh"), "女仆装");
        assert_eq!(entry.display_name("fr"), "Maid Uniform");
        assert_eq!(entry.display_name("en"), "Maid Uniform");
    }

    #[test]
    fn test_namespace_search() {
        let ns = Namespace::new(
            "cosplay",
            vec![
                NamespaceEntry::new("Maid").with_name("zh", "女仆装"),
                NamespaceEntry::new("Bunny").with_name("zh", "兔女郎"),
                NamespaceEntry::new("Ninja").with_name("zh", "忍者"),
            ],
        );
        let results = ns.search("女", "zh");
        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|e| e.key == "Maid"));
        assert!(results.iter().any(|e| e.key == "Bunny"));
    }

    #[test]
    fn test_load_simple_selected() {
        let json = r#"{"selected": ["A", "B", "C"]}"#;
        let ns = load_simple_selected("test", json).unwrap();
        assert_eq!(ns.name, "test");
        assert_eq!(ns.entries.len(), 3);
        assert_eq!(ns.get("B").unwrap().key, "B");
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_remove_namespace_entry_persists_to_file() {
        let tmp = std::env::temp_dir().join(format!(
            "cm2_test_namespaces_{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&tmp).unwrap();

        let file_path = tmp.join("test.json");
        let content = serde_json::json!({
            "name": "test",
            "entries": {
                "A": { "en": "Alpha", "zh": "阿尔法" },
                "B": { "en": "Beta", "zh": "贝塔" }
            }
        });
        std::fs::write(&file_path, serde_json::to_string_pretty(&content).unwrap()).unwrap();

        let mut registry = NamespaceRegistry::new();
        registry.load_from_dir(&tmp);
        assert_eq!(registry.get("test").unwrap().entries.len(), 2);

        registry.remove_entry("test", "A", &tmp).unwrap();
        assert_eq!(registry.get("test").unwrap().entries.len(), 1);
        assert!(registry.get("test").unwrap().get("B").is_some());

        let mut reloaded = NamespaceRegistry::new();
        reloaded.load_from_dir(&tmp);
        assert_eq!(reloaded.get("test").unwrap().entries.len(), 1);

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_remove_namespace_entry_missing_returns_error() {
        let tmp = std::env::temp_dir().join(format!(
            "cm2_test_namespaces_missing_{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&tmp).unwrap();
        std::fs::write(
            tmp.join("test.json"),
            serde_json::to_string_pretty(&serde_json::json!({
                "name": "test",
                "entries": { "A": { "en": "Alpha" } }
            }))
            .unwrap(),
        )
        .unwrap();

        let mut registry = NamespaceRegistry::new();
        registry.load_from_dir(&tmp);
        assert!(registry.remove_entry("test", "Missing", &tmp).is_err());

        let _ = std::fs::remove_dir_all(&tmp);
    }
}

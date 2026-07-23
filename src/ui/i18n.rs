//! Lightweight internationalization (i18n) support for the editor UI.
//!
//! Translations are loaded from JSON files under `assets/i18n/`. Each file is a
//! flat map of `key -> text`. The current language falls back to English (`en`)
//! when a key is missing, and finally returns the key itself for easy debugging.

use std::collections::HashMap;
use std::path::Path;

use crate::api::registry::{get_definition, all_node_definitions};
use crate::error::{FlowError, Result};
use crate::graph::types::NodeType;

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
    /// Files that fail to parse are skipped so that a single broken translation
    /// file does not prevent the rest from loading.
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
                if lang.is_empty() || lang.starts_with('_') {
                    continue;
                }
                match std::fs::read_to_string(&path)
                    .map_err(FlowError::from)
                    .and_then(|text| serde_json::from_str::<HashMap<String, String>>(&text).map_err(FlowError::from))
                {
                    Ok(map) => {
                        self.translations.insert(lang, map);
                    }
                    Err(e) => {
                        eprintln!("i18n: failed to load {}: {}", path.display(), e);
                    }
                }
            }
        }
        Ok(())
    }

    /// Load bundled translations from the default asset location.
    ///
    /// Tries the current working directory first, then the executable directory
    /// and its parent, so that running from `cargo run` or a packaged binary both
    /// work.
    pub fn load_bundled() -> Self {
        let mut i18n = Self::new();
        let mut dirs = vec![std::path::PathBuf::from("assets/i18n")];
        if let Ok(exe) = std::env::current_exe() {
            if let Some(exe_dir) = exe.parent() {
                dirs.push(exe_dir.join("assets/i18n"));
                if let Some(parent) = exe_dir.parent() {
                    dirs.push(parent.join("assets/i18n"));
                }
            }
        }
        for dir in dirs {
            if i18n.translations.is_empty() || dir.exists() {
                let _ = i18n.load_from_dir(&dir);
            }
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

    /// Translate a stage key (`stage.{name}`) into the current language.
    /// Falls back to the raw stage name if no translation is available.
    pub fn stage_name(&self, stage: &str) -> String {
        let key = format!("stage.{}", stage);
        self.text_opt(&key).unwrap_or_else(|| stage.to_string())
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

    /// Translate a node display name. Falls back to `NodeDefinition::display_name`
    /// and finally to the `NodeType` debug string.
    pub fn node_display_name(&self, node_type: NodeType) -> String {
        let key = format!("node.{:?}.name", node_type);
        self.text_opt(&key).unwrap_or_else(|| {
            get_definition(node_type)
                .map(|d| d.display_name.clone())
                .unwrap_or_else(|| format!("{:?}", node_type))
        })
    }

    /// Translate a node description. Falls back to `NodeDefinition::description`.
    pub fn node_description(&self, node_type: NodeType) -> String {
        let key = format!("node.{:?}.description", node_type);
        self.text_opt(&key).unwrap_or_else(|| {
            get_definition(node_type)
                .map(|d| d.description.clone())
                .unwrap_or_default()
        })
    }

    /// Translate a port label for a specific node type. Falls back to the port
    /// definition in `NodeDefinition`.
    pub fn port_display_name(&self, node_type: NodeType, port_id: &str) -> String {
        let key = format!("node.{:?}.port.{}", node_type, port_id);
        self.text_opt(&key).unwrap_or_else(|| {
            get_definition(node_type)
                .and_then(|d| {
                    d.inputs
                        .iter()
                        .chain(d.outputs.iter())
                        .find(|p| p.id == port_id)
                        .map(|p| p.label.clone())
                })
                .unwrap_or_else(|| port_id.to_string())
        })
    }

    /// Translate a parameter display name for a specific node type. Falls back to
    /// the parameter definition in `NodeDefinition`, then to dynamic parameter templates.
    pub fn param_display_name(&self, node_type: NodeType, param_name: &str) -> String {
        let key = format!("node.{:?}.param.{}", node_type, param_name);
        self.text_opt(&key).unwrap_or_else(|| {
            get_definition(node_type)
                .and_then(|d| {
                    d.params
                        .iter()
                        .find(|p| p.name == param_name)
                        .map(|p| p.display_name.clone())
                        .or_else(|| dynamic_param_display_name(d, param_name))
                })
                .unwrap_or_else(|| param_name.to_string())
        })
    }
}

/// Try to find a display name for a dynamic parameter instance.
fn dynamic_param_display_name(def: &crate::api::definitions::NodeDefinition, param_id: &str) -> Option<String> {
    use crate::graph::types::{DynamicPortKind, DynamicPortTemplate};

    for group in &def.dynamic_ports {
        for member in &group.members {
            if member.kind == DynamicPortKind::Param {
                if let DynamicPortTemplate::Param(p) = &member.template {
                    let suffix = format!("_{}", member.id);
                    if param_id.starts_with(&group.prefix) && param_id.ends_with(&suffix) {
                        return Some(p.display_name.clone());
                    }
                }
            }
        }
    }
    None
}

/// Generate initial translation templates for all registered nodes.
///
/// This is intended as a development helper: run it once to produce
/// `node_zh.json` and `node_en.json`, then merge the keys into the main
/// translation files.
#[allow(dead_code)]
pub fn generate_node_templates() -> (HashMap<String, String>, HashMap<String, String>) {
    let mut zh = HashMap::new();
    let mut en = HashMap::new();
    for def in all_node_definitions() {
        let type_key = format!("{:?}", def.node_type);
        zh.insert(format!("node.{}.name", type_key), def.display_name.clone());
        en.insert(format!("node.{}.name", type_key), type_key.clone());
        zh.insert(
            format!("node.{}.description", type_key),
            def.description.clone(),
        );
        en.insert(
            format!("node.{}.description", type_key),
            format!("{} node", type_key),
        );
        for param in &def.params {
            zh.insert(
                format!("node.{}.param.{}", type_key, param.name),
                param.display_name.clone(),
            );
            en.insert(
                format!("node.{}.param.{}", type_key, param.name),
                param.name.clone(),
            );
        }
        for input in &def.inputs {
            zh.insert(
                format!("node.{}.port.{}", type_key, input.id),
                input.label.clone(),
            );
            en.insert(
                format!("node.{}.port.{}", type_key, input.id),
                input.id.clone(),
            );
        }
        for output in &def.outputs {
            zh.insert(
                format!("node.{}.port.{}", type_key, output.id),
                output.label.clone(),
            );
            en.insert(
                format!("node.{}.port.{}", type_key, output.id),
                output.id.clone(),
            );
        }
    }
    (zh, en)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i18n_fallback_to_en() {
        let mut i18n = I18n::new();
        let mut map = HashMap::new();
        map.insert("label.name".to_string(), "Name".to_string());
        i18n.translations.insert("en".to_string(), map);
        i18n.set_language("zh");
        assert_eq!(i18n.text("label.name"), "Name");
    }

    #[test]
    fn test_i18n_current_language_wins() {
        let mut i18n = I18n::new();
        let mut en = HashMap::new();
        en.insert("label.name".to_string(), "Name".to_string());
        let mut zh = HashMap::new();
        zh.insert("label.name".to_string(), "名称".to_string());
        i18n.translations.insert("en".to_string(), en);
        i18n.translations.insert("zh".to_string(), zh);
        i18n.set_language("zh");
        assert_eq!(i18n.text("label.name"), "名称");
    }

    #[test]
    fn test_i18n_format() {
        let mut i18n = I18n::new();
        let mut map = HashMap::new();
        map.insert("saved".to_string(), "Saved {}".to_string());
        i18n.translations.insert("en".to_string(), map);
        assert_eq!(i18n.format("saved", &["proj"]), "Saved proj");
    }

    /// Ensure the bundled zh.json actually loads and overrides English toolbar
    /// strings when the active language is set to Chinese.
    #[test]
    fn test_bundled_zh_translations_load() {
        let mut i18n = I18n::load_bundled();
        i18n.set_language("zh");
        assert!(
            !i18n.translations.get("zh").expect("zh translations missing").is_empty(),
            "zh.json should contain translations"
        );
        assert_eq!(i18n.text("button.export_project"), "导出工程");
        assert_eq!(i18n.text("canvas.help_text"), "左键拖拽节点 | 中键平移 | 滚轮缩放 | Space 搜索");
    }

    /// 扫描 UI 业务代码中所有静态 i18n key，断言 zh/en/ja 三个翻译文件均存在。
    ///
    /// 动态 key（如 `node.{:?}.name`、由 `format!` 拼接的 key）通过字符串字面量
    /// 检测规则自动排除，仅检查 `i18n.text(KEY)` / `i18n.format(KEY, ...)` 中的
    /// 纯静态 key。
    #[test]
    fn all_static_i18n_keys_exist_in_three_languages() {
        let i18n = I18n::load_bundled();
        let keys = collect_static_i18n_keys();
        let mut missing = Vec::new();
        for key in &keys {
            for lang in ["zh", "en", "ja"] {
                let map = i18n.translations.get(lang);
                if map.map(|m| !m.contains_key(key)).unwrap_or(true) {
                    missing.push(format!("{} missing key: {}", lang, key));
                }
            }
        }
        if !missing.is_empty() {
            panic!(
                "发现以下 i18n key 在三种语言中未全部提供：\n{}",
                missing.join("\n")
            );
        }
    }

    fn collect_static_i18n_keys() -> std::collections::HashSet<String> {
        use std::collections::HashSet;
        use std::path::Path;

        let manifest = env!("CARGO_MANIFEST_DIR");
        let mut files = Vec::new();
        collect_rs_files(&Path::new(manifest).join("src").join("app.rs"), &mut files);
        collect_rs_files(&Path::new(manifest).join("src").join("ui"), &mut files);

        let mut keys = HashSet::new();
        for file in files {
            let content = std::fs::read_to_string(&file).unwrap();
            for line in content.lines() {
                for prefix in ["i18n.text(\"", "i18n.format(\""] {
                    for (idx, _) in line.match_indices(prefix) {
                        let after = &line[idx + prefix.len()..];
                        if let Some(end) = after.find("\")") {
                            let key = &after[..end];
                            if is_static_i18n_key(key) {
                                keys.insert(key.to_string());
                            }
                        }
                    }
                }
            }
        }
        keys
    }

    fn collect_rs_files(path: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(path.to_path_buf());
        } else if path.is_dir() {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    collect_rs_files(&entry.path(), out);
                }
            }
        }
    }

    fn is_static_i18n_key(key: &str) -> bool {
        // 排除空串、含转义或引号的残片、以及明显由 format! 拼接的动态 key。
        if key.is_empty() {
            return false;
        }
        if key.contains('\\') || key.contains('"') {
            return false;
        }
        // 以 node./stage. 开头的 key 由 node/stage 相关 API 动态生成，
        // 其翻译存在性由 generate_node_templates 与 stage 数据保证，不在此处检查。
        if key.starts_with("node.") || key.starts_with("stage.") {
            return false;
        }
        // 只接受点分小写 key，过滤掉 `{}` 等格式残片。
        key.chars().all(|c| c.is_ascii_lowercase() || c == '.' || c == '_' || c.is_ascii_digit())
    }
}

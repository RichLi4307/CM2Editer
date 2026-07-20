//! 坐标预设注册表。
//!
//! 坐标 JSON 文件存放在 `assets/coordinates/`，按场景（stage）分组。
//! 每个坐标条目包含：唯一 ID、显示名、场景名、x/y/z 浮点值。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::error::FlowError;

/// 单个坐标预设条目。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinateEntry {
    pub id: String,
    pub name: String,
    pub stage: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl CoordinateEntry {
    /// 格式化的坐标摘要（用于卡片显示）。
    pub fn coord_text(&self) -> String {
        format!("({:.1}, {:.1}, {:.1})", self.x, self.y, self.z)
    }
}

/// 坐标预设注册表。
#[derive(Debug, Clone, Default)]
pub struct CoordinateRegistry {
    pub entries: Vec<CoordinateEntry>,
}

impl CoordinateRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, entry: CoordinateEntry) {
        self.entries.push(entry);
    }

    /// 按场景名分组返回条目。
    pub fn by_stage(&self) -> HashMap<&str, Vec<&CoordinateEntry>> {
        let mut map: HashMap<&str, Vec<&CoordinateEntry>> = HashMap::new();
        for e in &self.entries {
            map.entry(&e.stage).or_default().push(e);
        }
        map
    }

    /// 所有场景名（去重、排序）。
    pub fn stage_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.entries.iter().map(|e| e.stage.as_str()).collect();
        names.sort();
        names.dedup();
        names
    }

    /// 搜索：匹配 id / name / stage。
    pub fn search(&self, query: &str) -> Vec<&CoordinateEntry> {
        let q = query.to_lowercase();
        self.entries
            .iter()
            .filter(|e| {
                e.id.to_lowercase().contains(&q)
                    || e.name.to_lowercase().contains(&q)
                    || e.stage.to_lowercase().contains(&q)
            })
            .collect()
    }

    pub fn get(&self, id: &str) -> Option<&CoordinateEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    /// 从目录加载所有 `.json` 坐标预设文件。
    pub fn load_from_dir(&mut self, dir: &Path) {
        if !dir.is_dir() {
            return;
        }
        let mut paths: Vec<_> = match std::fs::read_dir(dir) {
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
            Err(_) => return,
        };
        paths.sort();

        for path in paths {
            match load_coordinate_file(&path) {
                Ok(entries) => {
                    for entry in entries {
                        self.register(entry);
                    }
                }
                Err(err) => eprintln!("failed to load coordinates {:?}: {}", path, err),
            }
        }
    }

    /// 加载默认预设。
    pub fn load_bundled() -> Self {
        let mut registry = Self::new();
        registry.load_from_dir(Path::new("assets/coordinates"));
        registry
    }

    /// 从 `dir` 下的 JSON 文件中删除指定 ID 的坐标条目，并同步更新内存注册表。
    pub fn remove_entry(&mut self, id: &str, dir: &Path) -> crate::error::Result<()> {
        if !dir.is_dir() {
            return Err(FlowError::Io(format!(
                "coordinate directory not found: {}",
                dir.display()
            )));
        }

        let mut removed = false;
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path
                .extension()
                .map(|ext| ext.eq_ignore_ascii_case("json"))
                .unwrap_or(false)
                && remove_coordinate_entry_from_file(&path, id)?
            {
                removed = true;
                break;
            }
        }

        if removed {
            self.entries.retain(|e| e.id != id);
            Ok(())
        } else {
            Err(FlowError::Validation(format!("coordinate not found: {}", id)))
        }
    }
}

fn remove_coordinate_entry_from_file(path: &Path, id: &str) -> crate::error::Result<bool> {
    let text = std::fs::read_to_string(path)?;
    let mut value: serde_json::Value = serde_json::from_str(&text)?;
    let Some(array) = value.as_array_mut() else {
        return Ok(false);
    };
    let before = array.len();
    array.retain(|v| v.get("id").and_then(|v| v.as_str()) != Some(id));
    if array.len() < before {
        std::fs::write(path, serde_json::to_string_pretty(&value)?)?;
        return Ok(true);
    }
    Ok(false)
}

fn load_coordinate_file(path: &Path) -> Result<Vec<CoordinateEntry>, Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string(path)?;
    let entries: Vec<CoordinateEntry> = serde_json::from_str(&text)?;
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let mut reg = CoordinateRegistry::new();
        reg.register(CoordinateEntry {
            id: "spawn".into(),
            name: "Spawn".into(),
            stage: "Apart".into(),
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        reg.register(CoordinateEntry {
            id: "bed".into(),
            name: "Bedside".into(),
            stage: "Apart".into(),
            x: 1.5,
            y: 0.0,
            z: -1.5,
        });
        assert_eq!(reg.search("Spawn").len(), 1);
        assert_eq!(reg.search("Apart").len(), 2);
        assert_eq!(reg.by_stage().get("Apart").unwrap().len(), 2);
        assert_eq!(reg.stage_names(), vec!["Apart"]);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_remove_coordinate_entry_persists_to_file() {
        let tmp = std::env::temp_dir().join(format!(
            "cm2_test_coordinates_{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&tmp).unwrap();

        let file_path = tmp.join("default.json");
        let content = serde_json::json!([
            { "id": "spawn", "name": "Spawn", "stage": "Apart", "x": 0.0, "y": 0.0, "z": 0.0 },
            { "id": "bed", "name": "Bedside", "stage": "Apart", "x": 1.5, "y": 0.0, "z": -1.5 }
        ]);
        std::fs::write(&file_path, serde_json::to_string_pretty(&content).unwrap()).unwrap();

        let mut registry = CoordinateRegistry::new();
        registry.load_from_dir(&tmp);
        assert_eq!(registry.entries.len(), 2);

        registry.remove_entry("spawn", &tmp).unwrap();
        assert_eq!(registry.entries.len(), 1);
        assert!(registry.get("bed").is_some());

        let mut reloaded = CoordinateRegistry::new();
        reloaded.load_from_dir(&tmp);
        assert_eq!(reloaded.entries.len(), 1);

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_remove_coordinate_entry_missing_returns_error() {
        let tmp = std::env::temp_dir().join(format!(
            "cm2_test_coordinates_missing_{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&tmp).unwrap();
        std::fs::write(
            tmp.join("default.json"),
            serde_json::to_string_pretty(&serde_json::json!([
                { "id": "spawn", "name": "Spawn", "stage": "Apart", "x": 0.0, "y": 0.0, "z": 0.0 }
            ]))
            .unwrap(),
        )
        .unwrap();

        let mut registry = CoordinateRegistry::new();
        registry.load_from_dir(&tmp);
        assert!(registry.remove_entry("missing", &tmp).is_err());

        let _ = std::fs::remove_dir_all(&tmp);
    }
}

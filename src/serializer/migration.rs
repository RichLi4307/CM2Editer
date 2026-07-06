use serde_json::{Value, json};

use crate::error::{FlowError, Result};

/// 当前支持的 JSON 格式版本
pub const CURRENT_VERSION: &str = "1.3";

/// 将任意历史版本 JSON 升级到当前版本
///
/// 版本链：1.0 → 1.1 → 1.2 → 1.3
/// 缺失字段使用文档约定的默认值补全
pub fn migrate_to_latest(value: Value) -> Result<Value> {
    let version = value
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("1.0")
        .to_string();
    match version.as_str() {
        "1.0" => {
            let value = migrate_1_0_to_1_1(value)?;
            let value = migrate_1_1_to_1_2(value)?;
            migrate_1_2_to_1_3(value)
        }
        "1.1" => {
            let value = migrate_1_1_to_1_2(value)?;
            migrate_1_2_to_1_3(value)
        }
        "1.2" => migrate_1_2_to_1_3(value),
        "1.3" => Ok(value),
        _ => Err(FlowError::VersionMismatch {
            file: version,
            supported: CURRENT_VERSION.to_string(),
        }),
    }
}

fn migrate_1_0_to_1_1(mut value: Value) -> Result<Value> {
    let obj = value
        .as_object_mut()
        .ok_or_else(|| FlowError::Validation("JSON root must be an object".to_string()))?;
    if !obj.contains_key("threads") {
        obj.insert(
            "threads".to_string(),
            json!([{
                "id": "thread_main",
                "name": "main",
                "entry_label": "main",
                "parent": null,
                "auto_start": true
            }]),
        );
    }
    obj.insert("version".to_string(), json!("1.1"));
    Ok(value)
}

fn migrate_1_1_to_1_2(mut value: Value) -> Result<Value> {
    let obj = value
        .as_object_mut()
        .ok_or_else(|| FlowError::Validation("JSON root must be an object".to_string()))?;
    if let Some(nodes) = obj.get_mut("nodes").and_then(|n| n.as_array_mut()) {
        for node in nodes {
            if let Some(node_obj) = node.as_object_mut() {
                if !node_obj.contains_key("size") {
                    node_obj.insert(
                        "size".to_string(),
                        json!({ "width": 180.0, "height": 120.0 }),
                    );
                }
            }
        }
    }
    obj.insert("version".to_string(), json!("1.2"));
    Ok(value)
}

fn migrate_1_2_to_1_3(mut value: Value) -> Result<Value> {
    let obj = value
        .as_object_mut()
        .ok_or_else(|| FlowError::Validation("JSON root must be an object".to_string()))?;
    if let Some(viewport) = obj.get_mut("viewport").and_then(|v| v.as_object_mut()) {
        if !viewport.contains_key("grid_size") {
            viewport.insert("grid_size".to_string(), json!(20.0));
        }
        if !viewport.contains_key("show_grid") {
            viewport.insert("show_grid".to_string(), json!(true));
        }
    } else {
        obj.insert(
            "viewport".to_string(),
            json!({
                "x": 0.0,
                "y": 0.0,
                "zoom": 1.0,
                "grid_size": 20.0,
                "show_grid": true
            }),
        );
    }
    obj.insert("version".to_string(), json!("1.3"));
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrate_1_0_to_latest() -> Result<()> {
        let input = json!({
            "version": "1.0",
            "nodes": []
        });
        let migrated = migrate_to_latest(input)?;
        assert_eq!(migrated["version"].as_str(), Some("1.3"));
        assert!(migrated["threads"].is_array());
        assert_eq!(migrated["viewport"]["grid_size"].as_f64(), Some(20.0));
        assert_eq!(migrated["viewport"]["show_grid"].as_bool(), Some(true));
        Ok(())
    }

    #[test]
    fn test_migrate_1_0_adds_default_thread() -> Result<()> {
        let input = json!({
            "version": "1.0",
            "nodes": []
        });
        let migrated = migrate_to_latest(input)?;
        let threads = migrated["threads"]
            .as_array()
            .ok_or_else(|| FlowError::Validation("threads should be an array".to_string()))?;
        assert_eq!(threads.len(), 1);
        let first = threads
            .first()
            .ok_or_else(|| FlowError::Validation("missing default thread".to_string()))?;
        assert_eq!(first["id"].as_str(), Some("thread_main"));
        assert_eq!(first["entry_label"].as_str(), Some("main"));
        assert_eq!(first["auto_start"].as_bool(), Some(true));
        Ok(())
    }

    #[test]
    fn test_migrate_1_1_adds_size_to_nodes() -> Result<()> {
        let input = json!({
            "version": "1.1",
            "nodes": [
                { "id": "node_1", "type": "Log", "position": { "x": 0.0, "y": 0.0 } }
            ]
        });
        let migrated = migrate_to_latest(input)?;
        let nodes = migrated["nodes"]
            .as_array()
            .ok_or_else(|| FlowError::Validation("nodes should be an array".to_string()))?;
        let node = nodes
            .first()
            .ok_or_else(|| FlowError::Validation("missing migrated node".to_string()))?;
        assert_eq!(node["size"]["width"].as_f64(), Some(180.0));
        assert_eq!(node["size"]["height"].as_f64(), Some(120.0));
        Ok(())
    }

    #[test]
    fn test_migrate_1_2_adds_viewport_fields() -> Result<()> {
        let input = json!({
            "version": "1.2",
            "nodes": [],
            "viewport": { "x": 10.0, "y": 20.0, "zoom": 2.0 }
        });
        let migrated = migrate_to_latest(input)?;
        assert_eq!(migrated["viewport"]["grid_size"].as_f64(), Some(20.0));
        assert_eq!(migrated["viewport"]["show_grid"].as_bool(), Some(true));
        assert_eq!(migrated["viewport"]["zoom"].as_f64(), Some(2.0));
        Ok(())
    }

    #[test]
    fn test_migrate_without_version_assumes_1_0() -> Result<()> {
        let input = json!({ "nodes": [] });
        let migrated = migrate_to_latest(input)?;
        assert_eq!(migrated["version"].as_str(), Some("1.3"));
        assert!(migrated["threads"].is_array());
        assert!(migrated["viewport"].is_object());
        Ok(())
    }

    #[test]
    fn test_migrate_unsupported_version_fails() {
        let input = json!({ "version": "2.0", "nodes": [] });
        let result = migrate_to_latest(input);
        assert!(matches!(
            result,
            Err(FlowError::VersionMismatch {
                file,
                supported
            }) if file == "2.0" && supported == "1.3"
        ));
    }
}

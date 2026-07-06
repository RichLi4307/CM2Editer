use crate::graph::node::{Node, ParamValue};

/// 属性面板。
pub struct PropertiesPanel;

impl PropertiesPanel {
    /// 显示选中节点的可编辑参数，返回发生变更的参数键值（如果有）。
    pub fn show(ui: &mut egui::Ui, node: &Node) -> Option<(String, ParamValue)> {
        ui.heading("属性");
        ui.separator();
        ui.label(format!("节点 ID: {}", node.id));
        ui.label(format!("类型: {:?}", node.node_type));
        ui.label(format!("分类: {}", node.category));
        ui.label(format!(
            "折叠: {}",
            if node.collapsed { "是" } else { "否" }
        ));

        ui.separator();
        ui.label("参数");

        let mut changed = None;
        for (key, value) in &node.params {
            ui.horizontal(|ui| {
                ui.label(key);
                let mut text = param_value_to_string(value);
                if ui.text_edit_singleline(&mut text).changed() {
                    if let Some(new_value) = parse_param_value(&text, value) {
                        changed = Some((key.clone(), new_value));
                    }
                }
            });
        }

        if node.params.is_empty() {
            ui.label("(无参数)");
        }

        ui.separator();
        ui.label(format!(
            "位置: ({:.1}, {:.1})",
            node.position.x, node.position.y
        ));
        ui.label(format!("尺寸: {:.0} x {:.0}", node.size.x, node.size.y));

        changed
    }
}

/// 将参数值转换为可编辑字符串。
fn param_value_to_string(value: &ParamValue) -> String {
    match value {
        ParamValue::Null => String::from("(null)"),
        ParamValue::Ref { node, port } => format!("ref:{}/{}", node, port),
        ParamValue::Literal(v) => v.to_string(),
    }
}

/// 尝试将编辑后的字符串解析回参数值。
fn parse_param_value(text: &str, original: &ParamValue) -> Option<ParamValue> {
    if text == "(null)" || text.is_empty() {
        return Some(ParamValue::Null);
    }
    if let Some(rest) = text.strip_prefix("ref:") {
        let parts: Vec<&str> = rest.split('/').collect();
        if parts.len() == 2 {
            return Some(ParamValue::from_ref(parts[0], parts[1]));
        }
        return None;
    }

    // 根据原值类型推断解析方式
    if let ParamValue::Literal(original_value) = original {
        if let Some(v) = parse_json_literal(text, original_value) {
            return Some(ParamValue::Literal(v));
        }
    }

    // 默认按 JSON 解析
    serde_json::from_str(text).ok().map(ParamValue::Literal)
}

/// 根据原 JSON 类型解析新字符串。
fn parse_json_literal(text: &str, original: &serde_json::Value) -> Option<serde_json::Value> {
    match original {
        serde_json::Value::Bool(_) => match text {
            "true" => Some(serde_json::json!(true)),
            "false" => Some(serde_json::json!(false)),
            _ => None,
        },
        serde_json::Value::Number(_) => text.parse::<f64>().ok().map(|v| serde_json::json!(v)),
        serde_json::Value::String(_) => Some(serde_json::json!(text)),
        _ => serde_json::from_str(text).ok(),
    }
}

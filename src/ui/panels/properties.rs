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
                if let Some((new_key, new_value)) = Self::param_editor(ui, key, value) {
                    changed = Some((new_key, new_value));
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

    /// 为单个参数绘制合适的编辑控件。
    fn param_editor(
        ui: &mut egui::Ui,
        key: &str,
        value: &ParamValue,
    ) -> Option<(String, ParamValue)> {
        match value {
            // 布尔值：使用复选框
            ParamValue::Literal(v) if v.is_boolean() => {
                let mut b = v.as_bool().unwrap_or(false);
                if ui.checkbox(&mut b, "").changed() {
                    return Some((key.to_string(), ParamValue::Literal(serde_json::json!(b))));
                }
                None
            }
            // 数值：使用只允许数字的输入框
            ParamValue::Literal(v) if v.is_number() => {
                let mut num = v.as_f64().unwrap_or(0.0);
                if ui.add(egui::DragValue::new(&mut num)).changed() {
                    return Some((key.to_string(), ParamValue::Literal(serde_json::json!(num))));
                }
                None
            }
            // 引用：使用文本，但带 ref: 前缀
            ParamValue::Ref { node, port } => {
                let mut text = format!("ref:{}/{}", node, port);
                let response = ui.text_edit_singleline(&mut text);
                if response.changed() {
                    if let Some(new_value) = parse_param_value(&text, value) {
                        return Some((key.to_string(), new_value));
                    }
                }
                None
            }
            // 字符串 / Null / 其他 JSON：使用文本编辑，解析失败时静默忽略
            _ => {
                let mut text = param_value_to_string(value);
                if ui.text_edit_singleline(&mut text).changed() {
                    if let Some(new_value) = parse_param_value(&text, value) {
                        return Some((key.to_string(), new_value));
                    }
                }
                None
            }
        }
    }
}

/// 将参数值转换为可编辑字符串。
fn param_value_to_string(value: &ParamValue) -> String {
    match value {
        ParamValue::Null => String::new(),
        ParamValue::Ref { node, port } => format!("ref:{}/{}", node, port),
        ParamValue::Literal(v) => {
            if v.is_string() {
                v.as_str().unwrap_or_default().to_string()
            } else {
                v.to_string()
            }
        }
    }
}

/// 尝试将编辑后的字符串解析回参数值。
fn parse_param_value(text: &str, original: &ParamValue) -> Option<ParamValue> {
    if text.is_empty() {
        return Some(ParamValue::Null);
    }

    // ref: 引用格式
    if let Some(rest) = text.strip_prefix("ref:") {
        let parts: Vec<&str> = rest.split('/').collect();
        if parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty() {
            return Some(ParamValue::from_ref(parts[0], parts[1]));
        }
        return None;
    }

    // 根据原值类型推断
    match original {
        ParamValue::Literal(original_value) => {
            if original_value.is_string() {
                return Some(ParamValue::Literal(serde_json::json!(text)));
            }
            if let Some(v) = parse_json_literal(text, original_value) {
                return Some(ParamValue::Literal(v));
            }
        }
        ParamValue::Null => {
            // 先尝试 JSON 解析，失败则视为字符串
            if let Ok(v) = serde_json::from_str(text) {
                return Some(ParamValue::Literal(v));
            }
            return Some(ParamValue::Literal(serde_json::json!(text)));
        }
        _ => {}
    }

    None
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

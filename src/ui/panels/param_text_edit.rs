use crate::graph::node::ParamValue;

/// 属性面板统一文本输入组件。
///
/// 接受任意文本输入，非 JSON 类型原样存为字符串；
/// Object / List / Array 类型在下行显示校验错误，不吞字。
pub struct ParamTextEdit;

impl ParamTextEdit {
    pub fn show(
        ui: &mut egui::Ui,
        key: &str,
        value: &ParamValue,
        placeholder: &str,
    ) -> Option<(String, ParamValue)> {
        let mut text = val_to_str(value);
        let widget = egui::TextEdit::singleline(&mut text)
            .hint_text(placeholder)
            .desired_width(ui.available_width().max(60.0));
        let resp = ui.add(widget);

        // 校验：仅对 Object / List 类型的原始值做 JSON 合法性检查，
        // 不合法的在下行报错但**不阻止文本输入**。
        let needs_json = matches!(value, ParamValue::Literal(v) if v.is_object() || v.is_array());
        let mut json_ok = true;
        if needs_json && !text.is_empty() {
            if serde_json::from_str::<serde_json::Value>(&text).is_err() {
                json_ok = false;
            }
        }
        if !json_ok {
            ui.label(
                egui::RichText::new("格式错误：请输入合法 JSON")
                    .color(egui::Color32::from_rgb(240, 80, 80))
                    .size(10.0),
            );
        }

        if resp.changed() && (json_ok || !needs_json) {
            Some((key.to_string(), str_to_param(&text, value, json_ok)))
        } else {
            None
        }
    }
}

fn val_to_str(value: &ParamValue) -> String {
    match value {
        ParamValue::Literal(v) if v.is_string() => v.as_str().unwrap_or_default().to_string(),
        ParamValue::Literal(v) => serde_json::to_string(v).unwrap_or_else(|_| "null".to_string()),
        ParamValue::Null => String::new(),
        ParamValue::Ref { node, port } => format!("{}.{} (ref)", node, port),
    }
}

/// Boolean 和 Number 保形解析；其他类型直接存字符串文本；
/// Object / List 类型仅在 JSON 合法时才存为原类型，否则存字符串。
fn str_to_param(text: &str, original: &ParamValue, json_valid: bool) -> ParamValue {
    if let ParamValue::Literal(v) = original {
        if v.is_boolean() {
            return match text {
                "true" => ParamValue::Literal(serde_json::json!(true)),
                "false" => ParamValue::Literal(serde_json::json!(false)),
                _ => ParamValue::Literal(serde_json::json!(text)),
            };
        }
        if v.is_number() {
            if let Ok(n) = text.parse::<f64>() {
                return ParamValue::Literal(serde_json::json!(n));
            }
        }
    }
    // JSON 有效 → 保形
    if json_valid && !text.is_empty() {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(text) {
            return ParamValue::Literal(v);
        }
    }
    // 回退：字符串
    ParamValue::Literal(serde_json::json!(text))
}

use crate::graph::node::ParamValue;

pub struct ParamTextEdit;

impl ParamTextEdit {
    pub fn show(
        ui: &mut egui::Ui,
        key: &str,
        value: &ParamValue,
        placeholder: &str,
    ) -> Option<(String, ParamValue)> {
        let mut text = val_to_str(value);
        let mut widget = egui::TextEdit::singleline(&mut text)
            .hint_text(placeholder)
            .desired_width(ui.available_width().max(60.0));
        if ui.add(widget).changed() {
            str_to_param(&text, value)
                .map(|v| (key.to_string(), v))
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

fn str_to_param(text: &str, original: &ParamValue) -> Option<ParamValue> {
    match original {
        ParamValue::Literal(v) if v.is_boolean() => match text {
            "true" => Some(ParamValue::Literal(serde_json::json!(true))),
            "false" => Some(ParamValue::Literal(serde_json::json!(false))),
            _ => None,
        },
        ParamValue::Literal(v) if v.is_number() => {
            text.parse::<f64>().ok().map(|v| ParamValue::Literal(serde_json::json!(v)))
        }
        ParamValue::Literal(v) if v.is_string() => {
            Some(ParamValue::Literal(serde_json::json!(text)))
        }
        ParamValue::Literal(_) => serde_json::from_str(text).ok().map(ParamValue::Literal),
        ParamValue::Null => serde_json::from_str(text).ok().map(ParamValue::Literal),
        ParamValue::Ref { .. } => serde_json::from_str(text).ok().map(ParamValue::Literal),
    }
}

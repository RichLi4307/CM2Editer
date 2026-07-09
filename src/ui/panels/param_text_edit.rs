use crate::graph::node::ParamValue;
use std::collections::HashMap;

/// 持久编辑缓冲区：在失焦/回车时才提交文本到 ParamValue。
pub type EditBuffers = HashMap<String, String>;

/// 属性面板统一文本输入组件。
///
/// - 传入持久缓冲区 `buffers`，文本框始终从中读写
/// - 缓冲区未初始化时从 `ParamValue` 取值
/// - 失焦 / 按回车 → 提交校验 → 成功则写入 ParamValue 并清空缓冲区
/// - 校验失败 → 文本保留在缓冲区，红色错误提示，不吞字
pub struct ParamTextEdit;

impl ParamTextEdit {
    pub fn show(
        ui: &mut egui::Ui,
        buf_key: &str,       // 缓冲区 key（如 "{node_id}.{param_name}"）
        value: &ParamValue,  // 当前值（仅用于初始化缓冲区）
        buffers: &mut EditBuffers,
        placeholder: &str,
    ) -> Option<(String, ParamValue)> {
        // 初始化或读取缓冲区
        let buf = buffers
            .entry(buf_key.to_string())
            .or_insert_with(|| val_to_str(value));

        let mut text = buf.clone();
        let widget = egui::TextEdit::singleline(&mut text)
            .hint_text(placeholder)
            .desired_width(ui.available_width().max(60.0));
        let resp = ui.add(widget);

        // 判断是否需要校验的类型
        let needs_json = matches!(value, ParamValue::Literal(v) if v.is_object() || v.is_array());

        // 实时更新缓冲区（不校验，不吞字）
        if resp.changed() {
            *buf = text.clone();
        }

        // 校验 + 错误提示
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

        // 提交条件：失焦或回车，且文本有修改
        let committed = resp.changed()
            && (resp.lost_focus()
                || ui.input(|i| i.key_pressed(egui::Key::Enter)));
        if committed && (json_ok || !needs_json) {
            Some((buf_key.split('.').last().unwrap_or(buf_key).to_string(), str_to_param(&text, value, json_ok)))
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
    if json_valid && !text.is_empty() {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(text) {
            return ParamValue::Literal(v);
        }
    }
    ParamValue::Literal(serde_json::json!(text))
}

/// JSON 预览面板。使用 ScrollArea 防止长 JSON 撑高父容器。
pub struct JsonPreviewPanel;

impl JsonPreviewPanel {
    /// 显示当前图的 JSON 序列化预览。高度由父级 `allocate_new_ui` 控制。
    pub fn show(ui: &mut egui::Ui, json: &str) {
        ui.heading("JSON");
        let mut text = json.to_string();
        let rows = (ui.available_height() / 16.0).max(4.0) as usize;
        egui::ScrollArea::vertical()
            .id_salt("json_preview_scroll")
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut text)
                        .desired_rows(rows)
                        .interactive(false)
                        .font(egui::TextStyle::Monospace),
                );
            });
    }
}

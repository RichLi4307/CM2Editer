/// JSON 预览面板。
pub struct JsonPreviewPanel;

impl JsonPreviewPanel {
    /// 显示当前图的 JSON 序列化预览。
    pub fn show(ui: &mut egui::Ui, json: &str) {
        ui.horizontal(|ui| {
            ui.heading("JSON 预览");
        });

        egui::ScrollArea::vertical()
            .id_salt("json_preview_scroll")
            .show(ui, |ui| {
            let mut text = json.to_string();
            ui.add(
                egui::TextEdit::multiline(&mut text)
                    .desired_rows(6)
                    .interactive(false)
                    .font(egui::TextStyle::Monospace),
            );
        });
    }
}

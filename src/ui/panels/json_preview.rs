/// JSON 预览面板。
pub struct JsonPreviewPanel;

impl JsonPreviewPanel {
    /// 显示当前图的 JSON 序列化预览，自动填满父级分配的空间。
    pub fn show(ui: &mut egui::Ui, json: &str) {
        let available = ui.available_size();
        ui.heading("JSON 预览");

        let text_height = (available.y - 24.0).max(60.0);
        let mut text = json.to_string();
        ui.add(
            egui::TextEdit::multiline(&mut text)
                .desired_width(available.x)
                .desired_rows((text_height / 16.0) as usize)
                .interactive(false)
                .font(egui::TextStyle::Monospace),
        );
    }
}

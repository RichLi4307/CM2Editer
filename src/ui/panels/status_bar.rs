use egui::Pos2;

/// 状态栏面板。
pub struct StatusBarPanel;

impl StatusBarPanel {
    /// 显示状态栏信息。
    pub fn show(
        ui: &mut egui::Ui,
        status_message: &str,
        error_count: usize,
        world_pos: Option<Pos2>,
        zoom: f32,
    ) {
        ui.horizontal(|ui| {
            ui.label(status_message);
            ui.separator();
            if error_count > 0 {
                ui.colored_label(
                    egui::Color32::from_rgb(244, 67, 54),
                    format!("错误: {}", error_count),
                );
            } else {
                ui.label("无错误");
            }
            ui.separator();
            if let Some(pos) = world_pos {
                ui.label(format!("World: ({:.1}, {:.1})", pos.x, pos.y));
            } else {
                ui.label("World: -");
            }
            ui.separator();
            ui.label(format!("Zoom: {:.2}x", zoom));
        });
    }
}

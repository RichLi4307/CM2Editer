use egui::Pos2;
use crate::error::FlowError;

/// 状态栏面板。
pub struct StatusBarPanel;

impl StatusBarPanel {
    /// 显示状态栏信息。
    pub fn show(
        ui: &mut egui::Ui,
        status_message: &str,
        errors: &[FlowError],
        world_pos: Option<Pos2>,
        zoom: f32,
    ) {
        ui.horizontal(|ui| {
            ui.label(status_message);
            ui.separator();
            if errors.is_empty() {
                ui.label("无错误");
            } else {
                ui.colored_label(
                    egui::Color32::from_rgb(244, 67, 54),
                    format!("错误: {}", errors.len()),
                );
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

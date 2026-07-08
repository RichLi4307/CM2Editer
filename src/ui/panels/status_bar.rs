use crate::error::FlowError;
use egui::Pos2;

/// 状态栏面板。
pub struct StatusBarPanel;

/// 状态栏点击事件。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusBarEvent {
    None,
    /// 点击了错误计数，请求打开详情面板。
    OpenErrorDetails,
}

impl StatusBarPanel {
    /// 显示状态栏信息，返回用户交互事件。
    pub fn show(
        ui: &mut egui::Ui,
        status_message: &str,
        errors: &[FlowError],
        world_pos: Option<Pos2>,
        zoom: f32,
    ) -> StatusBarEvent {
        let mut event = StatusBarEvent::None;

        ui.horizontal(|ui| {
            ui.label(status_message);
            ui.separator();
            if errors.is_empty() {
                ui.label("无错误");
            } else {
                let block_count = errors.iter().filter(|e| e.is_blocking()).count();
                let warn_count = errors.iter().filter(|e| e.is_warning()).count();
                let label = if warn_count > 0 {
                    format!("错误: {} ⚠ {}", block_count, warn_count)
                } else {
                    format!("错误: {}", block_count)
                };
                if ui
                    .link(
                        egui::RichText::new(label)
                            .color(egui::Color32::from_rgb(244, 67, 54)),
                    )
                    .clicked()
                {
                    event = StatusBarEvent::OpenErrorDetails;
                }
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

        event
    }
}

/// 错误详情悬浮窗口。
pub struct ErrorDetailWindow;

impl ErrorDetailWindow {
    /// 显示错误详情窗口。返回 `true` 表示窗口仍应保持打开。
    pub fn show(open: &mut bool, errors: &[FlowError], ctx: &egui::Context) {
        egui::Window::new("错误详情")
            .id(egui::Id::new("error_detail_window"))
            .collapsible(false)
            .open(open)
            .resizable(true)
            .default_size([400.0, 250.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for err in errors {
                        let severity = if err.is_warning() { "⚠" } else { "❌" };
                        let color = if err.is_warning() {
                            egui::Color32::from_rgb(255, 180, 0)
                        } else {
                            egui::Color32::from_rgb(244, 67, 54)
                        };
                        ui.colored_label(color, format!("{} {}", severity, err));
                    }
                    if errors.is_empty() {
                        ui.label("当前无错误或警告");
                    }
                });
            });
    }
}

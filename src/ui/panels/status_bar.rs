use crate::error::FlowError;
use crate::ui::i18n::I18n;
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
        i18n: &I18n,
    ) -> StatusBarEvent {
        let mut event = StatusBarEvent::None;

        ui.horizontal(|ui| {
            ui.label(status_message);
            ui.separator();
            if errors.is_empty() {
                ui.label(i18n.text("status_bar.no_errors"));
            } else {
                let block_count = errors.iter().filter(|e| e.is_blocking()).count();
                let warn_count = errors.iter().filter(|e| e.is_warning()).count();
                let label = if warn_count > 0 {
                    i18n.format("status_bar.error", &[&block_count.to_string(), &warn_count.to_string()])
                } else {
                    i18n.format("status_bar.error_only", &[&block_count.to_string()])
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
                ui.label(i18n.format("status_bar.world", &[&format!("{:.1}", pos.x), &format!("{:.1}", pos.y)]));
            } else {
                ui.label(i18n.text("status_bar.world_empty"));
            }
            ui.separator();
            ui.label(i18n.format("status_bar.zoom", &[&format!("{:.2}", zoom)]));
        });

        event
    }
}

/// 错误详情悬浮窗口。
pub struct ErrorDetailWindow;

impl ErrorDetailWindow {
    /// 显示错误详情窗口。返回 `true` 表示窗口仍应保持打开。
    pub fn show(open: &mut bool, errors: &[FlowError], ctx: &egui::Context, i18n: &I18n) {
        egui::Window::new(i18n.text("status_bar.error_detail"))
            .id(egui::Id::new("error_detail_window"))
            .collapsible(false)
            .open(open)
            .resizable(true)
            .default_size([400.0, 250.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("error_detail_scroll")
                    .show(ui, |ui| {
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
                        ui.label(i18n.text("status_bar.no_errors_detail"));
                    }
                });
            });
    }
}

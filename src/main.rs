use CM2Editer::ui::canvas::Canvas;
use CM2Editer::ui::theme::{Theme, category_color};

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "CM2Editer",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(Cm2EditorApp::new()))),
    )
}

struct Cm2EditorApp {
    canvas: Canvas,
}

impl Cm2EditorApp {
    fn new() -> Self {
        Self {
            canvas: Canvas::new(),
        }
    }
}

impl eframe::App for Cm2EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).fill(Theme::BACKGROUND))
            .show(ctx, |ui| {
                let response = self.canvas.update(ui);

                // 左上角信息覆盖层
                let text = if let Some(world_pos) = response.hover_world_pos {
                    format!(
                        "World: ({:.1}, {:.1}) | Zoom: {:.2}x | 中键拖拽平移 | 滚轮缩放",
                        world_pos.x, world_pos.y, self.canvas.viewport.zoom
                    )
                } else {
                    format!(
                        "Zoom: {:.2}x | 中键拖拽平移 | 滚轮缩放",
                        self.canvas.viewport.zoom
                    )
                };
                ui.painter().text(
                    response.canvas_rect.min + egui::vec2(10.0, 10.0),
                    egui::Align2::LEFT_TOP,
                    text,
                    egui::FontId::proportional(14.0),
                    Theme::TEXT,
                );

                // 右下角分类色表预览
                let categories = [
                    "Control",
                    "General Functions",
                    "Game Functions: Stats",
                    "Objects",
                    "Math",
                    "String",
                    "File",
                    "Special",
                ];
                let preview_width = 140.0;
                let preview_height = categories.len() as f32 * 22.0 + 8.0;
                let preview_rect = egui::Rect::from_min_size(
                    response.canvas_rect.max
                        - egui::vec2(preview_width + 10.0, preview_height + 10.0),
                    egui::vec2(preview_width, preview_height),
                );
                ui.painter()
                    .rect_filled(preview_rect, 4.0, Theme::NODE_BACKGROUND);
                for (i, category) in categories.iter().enumerate() {
                    let color = category_color(category);
                    let y = preview_rect.min.y + 4.0 + i as f32 * 22.0;
                    let color_rect = egui::Rect::from_min_size(
                        egui::Pos2::new(preview_rect.min.x + 6.0, y),
                        egui::vec2(16.0, 16.0),
                    );
                    ui.painter().rect_filled(color_rect, 3.0, color);
                    ui.painter().text(
                        egui::Pos2::new(preview_rect.min.x + 28.0, y + 8.0),
                        egui::Align2::LEFT_CENTER,
                        *category,
                        egui::FontId::proportional(12.0),
                        Theme::TEXT,
                    );
                }
            });
    }
}

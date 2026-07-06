use CM2Editer::ui::theme::{Theme, category_color};

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "CM2Editer",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(Cm2EditorApp))),
    )
}

struct Cm2EditorApp;

impl eframe::App for Cm2EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).fill(Theme::BACKGROUND))
            .show(ctx, |ui| {
                ui.heading("CM2Editer");
                ui.label("Phase 3.1 完成：egui 框架已搭建，主题色表已就绪");
                ui.separator();
                ui.label("节点分类颜色预览（来源：docs/node_types.md 第 12 节）：");

                let categories = [
                    "Control",
                    "General Functions",
                    "Game Functions: Items",
                    "Game Functions: Player",
                    "Game Functions: Stats",
                    "Game Functions: Additional",
                    "Objects",
                    "Math",
                    "String",
                    "File",
                    "Special",
                ];

                for category in categories {
                    ui.horizontal(|ui| {
                        ui.label(category);
                        let color = category_color(category);
                        let size = egui::vec2(24.0, 24.0);
                        let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
                        ui.painter().rect_filled(rect, 4.0, color);
                    });
                }
            });
    }
}

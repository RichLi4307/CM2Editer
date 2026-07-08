use CM2Editer::app::App;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "CM2Editer",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            let monitor = cc.egui_ctx.input(|i| i.viewport().monitor_size);
            if let Some(size) = monitor {
                let w = (size.x as f32 * 0.75).max(1024.0);
                let h = (size.y as f32 * 0.75).max(600.0);
                cc.egui_ctx
                    .send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(w, h)));
            }
            Ok(Box::new(App::new(cc)))
        }),
    )
}

use CM2Editer::app::App;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "CM2Editer",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([1440.0, 900.0]),
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

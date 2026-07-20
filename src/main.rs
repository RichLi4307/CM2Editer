use CM2Editer::app::App;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "CM2Editer",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    ) {
        eprintln!("Run failed: {}", e);
    }
}

use CM2Editer::app::App;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "CM2Editer",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}

use CM2Editer::app::App;

fn main() -> Result<(), eframe::Error> {
    // 故意设一个远超绝大多数屏幕的初始尺寸，窗口管理器会将其 clamp 到实际屏幕大小，
    // 从而达到"最大化但可还原"的效果。
    eframe::run_native(
        "CM2Editer",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([3000.0, 2000.0]),
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

use CM2Editer::app::App;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "CM2Editer",
        eframe::NativeOptions {
            vsync: true,
            wgpu_options: eframe::egui_wgpu::WgpuConfiguration {
                desired_maximum_frame_latency: Some(1),
                ..Default::default()
            },
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

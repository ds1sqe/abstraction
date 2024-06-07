use abstaction::ui;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 400.0])
            .with_min_inner_size([400.0, 200.0]),
        ..Default::default()
    };

    eframe::run_native(
        "viewer",
        options,
        Box::new(|cc| Box::new(ui::manager::Manager::new(cc))),
    )
}

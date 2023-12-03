use eframe::egui;

use super::struct_mod::MyApp;

pub fn run_app() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native("Example", options, Box::new(|_cc| Box::<MyApp>::default()))
}

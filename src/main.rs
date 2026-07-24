mod app;
mod config;
mod installer;
mod utils;

use app::SetupApp;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Windows LTSC One-Click Workstation Setup GUI")
            .with_inner_size([960.0, 640.0])
            .with_min_inner_size([800.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Windows LTSC One-Click Setup",
        options,
        Box::new(|cc| Ok(Box::new(SetupApp::new(cc)))),
    )
}

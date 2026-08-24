#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[cfg(target_os = "windows")]
mod app;
#[cfg(any(target_os = "windows", test))]
#[cfg_attr(all(not(target_os = "windows"), test), allow(dead_code))]
mod config;
#[cfg(any(target_os = "windows", test))]
#[cfg_attr(all(not(target_os = "windows"), test), allow(dead_code))]
mod installer;
#[cfg(any(target_os = "windows", test))]
#[cfg_attr(all(not(target_os = "windows"), test), allow(dead_code))]
mod inventory;
#[cfg(any(target_os = "windows", test))]
#[cfg_attr(all(not(target_os = "windows"), test), allow(dead_code))]
mod platform;
#[cfg(any(target_os = "windows", test))]
#[cfg_attr(all(not(target_os = "windows"), test), allow(dead_code))]
mod rollback;
#[cfg(any(target_os = "windows", test))]
#[cfg_attr(all(not(target_os = "windows"), test), allow(dead_code))]
mod utils;

#[cfg(target_os = "windows")]
use app::SetupApp;
#[cfg(target_os = "windows")]
use eframe::egui;

#[cfg(target_os = "windows")]
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("LTSC Workspace")
            .with_inner_size([1280.0, 760.0])
            .with_min_inner_size([1040.0, 640.0]),
        ..Default::default()
    };

    eframe::run_native(
        "LTSC Workspace",
        options,
        Box::new(|cc| Ok(Box::new(SetupApp::new(cc)))),
    )
}

#[cfg(not(target_os = "windows"))]
fn main() {
    eprintln!(
        "LTSC Workspace GUI 仅支持 Windows；macOS 请运行 `cargo run --bin macos_inventory -- <输出文件>` 采集工具清单。"
    );
}

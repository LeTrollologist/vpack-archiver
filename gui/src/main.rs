/*!
VPack Archiver GUI — eframe/egui entry point
*/

// Hide console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("VPack Archiver 2.0 — Universal Archive Manager")
            .with_inner_size([1020.0, 640.0])
            .with_min_inner_size([720.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "VPack Archiver",
        native_options,
        Box::new(|cc| Ok(Box::new(app::VpackApp::new(cc)))),
    )
}

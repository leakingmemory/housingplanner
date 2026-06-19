//! Desktop entry point (Windows / Linux / macOS).
//!
//! `windows_subsystem = "windows"` keeps a console window from popping up
//! behind the GUI on Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> eframe::Result {
    // Print embedded license attribution and exit (no GUI).
    if std::env::args()
        .skip(1)
        .any(|a| a == "--licenses" || a == "licenses")
    {
        housingplanner::licenses::print_to_stdout();
        return Ok(());
    }

    let icon = eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon-256.png"))
        .expect("embedded icon is a valid PNG");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([800.0, 480.0])
            .with_icon(std::sync::Arc::new(icon))
            .with_title("Housing Planner"),
        ..Default::default()
    };

    eframe::run_native(
        "Housing Planner",
        options,
        Box::new(|cc| Ok(Box::new(housingplanner::app::PlannerApp::new(cc)))),
    )
}

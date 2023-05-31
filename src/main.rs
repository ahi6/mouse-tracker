use mouse_stats::app;

fn main() {
    // Run GUI in main thread
    run_app().expect("Failed to set up a graphics context")
}

fn run_app() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Mouse stat tracker",
        native_options,
        Box::new(|cc| Box::new(app::MouseTrackerApp::new(cc))),
    )
}

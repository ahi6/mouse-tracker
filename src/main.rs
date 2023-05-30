use mouse_stats::app;

fn main() {
    println!("Hello, world!");
    let native_options = eframe::NativeOptions::default();
    _ = eframe::run_native(
        "Mouse stat tracker",
        native_options,
        Box::new(|_cc| Box::new(app::MouseTrackerApp::default())),
    );
}

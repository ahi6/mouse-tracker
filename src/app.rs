use eframe::egui;

#[derive(Default)]
pub struct MouseTrackerApp {}

impl eframe::App for MouseTrackerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello world!");
        });
    }
}

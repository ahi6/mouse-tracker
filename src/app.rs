use std::sync::mpsc::Receiver;

use eframe::egui;

use crate::mouse_tracker::MouseStats;

pub struct MouseTrackerApp {
    mouse_tracker_receiver: Receiver<MouseStats>,
}

impl eframe::App for MouseTrackerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mouse_stats = self.mouse_tracker_receiver.recv().unwrap(); // blocking
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello world!");
            ui.label(format!("Mouse position: {:?}", mouse_stats))
        });
    }
}

impl MouseTrackerApp {
    pub fn new(rx: Receiver<MouseStats>) -> Self {
        MouseTrackerApp {
            mouse_tracker_receiver: rx,
        }
    }
}

use std::{
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    thread,
    time::Duration,
};

use eframe::egui;

use crate::mouse_tracker::{MouseStats, MouseTracker};

pub struct MouseTrackerApp {
    mouse_tracker_receiver: Receiver<MouseStats>,
    mouse_stats: MouseStats,
}

impl eframe::App for MouseTrackerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Try to update internal mouse_stats from bg thread
        if let Ok(mouse_stats) = self.mouse_tracker_receiver.try_recv() {
            self.mouse_stats = mouse_stats;
        }
        let mouse_stats = &self.mouse_stats;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mouse tracker");
            ui.separator();
            // Persistent stats
            ui.label(format!(
                "Your mouse has travelled {} pixels in total.",
                mouse_stats.total_distance.round()
            ));
            ui.label(format!(
                "Average move speed: {} px (or {} if you count idling)",
                mouse_stats.avg_nonzero_speed, mouse_stats.avg_speed
            ));
            ui.separator();
            // Volatile stats
            ui.label("Mouse tracker is active ☑");
            ui.label(format!("Current mouse position: {}", mouse_stats.position));
            ui.label(format!(
                "Current speed (delta): {}",
                mouse_stats.delta.round()
            ));
            ui.separator();
            // Debug info
            ui.collapsing("Debug info", |ui| {
                ui.heading("Mouse stats:");
                ui.code(format!("{:#?}", self.mouse_stats));
            });
        });
    }
}

impl MouseTrackerApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        // Spawn tracker thread
        let ctx = cc.egui_ctx.clone();
        let rx = MouseTrackerApp::spawn_tracker(ctx);

        // Create the app
        MouseTrackerApp {
            mouse_tracker_receiver: rx,
            mouse_stats: MouseStats::default(),
        }
    }

    fn spawn_tracker(ctx: egui::Context) -> Receiver<MouseStats> {
        // Initialize background thread to obtain mouse data in the background
        // Channel to communicate between GUI and mouse polling/tracker
        let (tx, rx): (SyncSender<MouseStats>, Receiver<MouseStats>) = sync_channel(1);
        // Spawn bg thread that handles the mouse data
        thread::spawn(move || {
            let mut tracker = MouseTracker::new();
            loop {
                // Get the data
                tracker.update();
                // Send the data
                _ = tx.try_send(tracker.stats.clone());
                // Is window is in focus?
                if ctx.input(|i| i.focused) {
                    // Tell GUI to update immediately
                    ctx.request_repaint();
                } else {
                    // Update eventually (after 2 secs)
                    ctx.request_repaint_after(Duration::from_secs(2));
                }

                // Sleep
                let dur = Duration::from_millis(120);
                thread::sleep(dur)
            }
        });

        rx
    }
}

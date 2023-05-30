use std::sync::mpsc::{sync_channel, Receiver, SyncSender}; // Maybe I should use a Mutex
use std::{thread, time::Duration};

use mouse_stats::app;
use mouse_stats::mouse_tracker::{MouseStats, MouseTracker};

fn main() {
    // Channel to communicate between GUI and mouse polling/tracker
    let (tx, rx): (SyncSender<MouseStats>, Receiver<MouseStats>) = sync_channel(1);
    // Spawn bg thread that handles the mouse data
    thread::spawn(move || {
        let mut tracker = MouseTracker::new();

        loop {
            tracker.update();
            _ = tx.try_send(tracker.stats.clone());

            let dur = Duration::from_millis(100);
            thread::sleep(dur)
        }
    });
    // Run GUI in main thread
    run_app(rx).expect("Failed to set up a graphics context")
}

fn run_app(rx: Receiver<MouseStats>) -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Mouse stat tracker",
        native_options,
        Box::new(|_cc| Box::new(app::MouseTrackerApp::new(rx))),
    )
}

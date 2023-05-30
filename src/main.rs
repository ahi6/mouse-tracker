use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::{thread, time::Duration};

use mouse_stats::app;
use mouse_stats::mouse_tracker::{MouseStats, MouseTracker};

fn main() {
    let (tx, rx): (SyncSender<MouseStats>, Receiver<MouseStats>) = sync_channel(1);
    thread::spawn(move || {
        let mut tracker = MouseTracker::new();

        loop {
            tracker.update();
            tx.send(tracker.stats.clone()).unwrap();

            let dur = Duration::from_secs(1);
            thread::sleep(dur)
        }
    });
    // loop {
    //
    //     thread::sleep(dur);
    // }
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

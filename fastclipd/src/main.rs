mod clipboard;

use log::info;

fn main() {
    env_logger::init();

    info!("Starting daemon");
    let mut tracker = clipboard::Tracker::new();
    tracker.poll();
}

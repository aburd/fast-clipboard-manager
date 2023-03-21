mod clipboard;

use log::info;

fn main() {
    env_logger::init();

    info!("Starting daemon");
    clipboard::paste();
}

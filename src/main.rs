use clipboard_master::Master;
use log::info;
use std::sync::{Arc, Mutex};
use std::thread;
use tiled_clipboard::clipboard;
use tiled_clipboard::config;
use tiled_clipboard::os_clipboard::OsClipboard;
use tiled_clipboard::ui;

fn main() {
    env_logger::init();

    info!("Starting tiled clipboard...");
    let config = config::get_config().unwrap();
    let clipboard = Arc::new(Mutex::new(clipboard::get_clipboard(&config).unwrap()));
    let clipboard_cm = Arc::clone(&clipboard);
    let clipboard_app = Arc::clone(&clipboard);
    let mut handles = vec![];

    let clipboard_master_handle = thread::spawn(move || {
        Master::new(OsClipboard::new(clipboard_cm)).run().unwrap();
    });
    handles.push(clipboard_master_handle);

    let gtk_handle = thread::spawn(move || {
        let exit_code = ui::build_and_run_gtk_app(clipboard_app);
        info!("Exited GTK app with exit code: {:?}", exit_code);
    });
    handles.push(gtk_handle);

    for handle in handles {
        handle.join().unwrap();
    }
}

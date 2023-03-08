use clipboard_master::Master;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use log::info;
use std::sync::{Arc, Mutex};
use std::thread;
use tiled_clipboard::clipboard::{self, Clipboard};
use tiled_clipboard::config;
use tiled_clipboard::os_clipboard_handler::OsClipboardHandler;

const APPLICATION_ID: &str = "com.github.aburd.tiled-clipboard-manager";

fn build_app(clipboard: Clipboard) -> Application {
    let app = Application::builder()
        .application_id(APPLICATION_ID)
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(320)
            .modal(true)
            .decorated(false)
            .build();

        window.show_all();
    });
    app
}

fn main() {
    env_logger::init();

    info!("Starting tiled clipboard...");
    let config = config::get_config().unwrap();
    let clipboard = Arc::new(Mutex::new(clipboard::get_clipboard(&config).unwrap()));
    let clipboard_cm = Arc::clone(&clipboard);

    let handle = thread::spawn(move || {
        let _ = Master::new(OsClipboardHandler::new(clipboard_cm)).run();
    });

    handle.join().unwrap();
}

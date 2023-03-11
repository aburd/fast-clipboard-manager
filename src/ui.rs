use crate::clipboard;
use crate::clipboard::Clipboard;
use crate::config;
use crate::os_clipboard::OsClipboard;
use crate::APPLICATION_ID;
use clipboard_master::Master;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk4 as gtk;
use log::info;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct AppError {}

pub fn build_app() -> Result<Application, AppError> {
    let app = Application::builder()
        .application_id(APPLICATION_ID)
        .build();

    app.connect_activate(|app| {
        build_ui(app);
    });
    Ok(app)
}

fn build_ui(app: &gtk::Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(720)
        .default_height(420)
        .modal(true)
        .decorated(false)
        .build();

    window.set_title(Some("Fast Clipboard"));
    window.set_default_size(260, 40);

    info!("Starting fast clipboard...");
    let config = config::get_config().unwrap();
    let clipboard = Arc::new(Mutex::new(clipboard::get_clipboard(&config).unwrap()));
    let clipboard_cm = Arc::clone(&clipboard);

    let label = gtk::Label::new(None);
    label.set_text(&clipboard.lock().unwrap().entries_text());

    window.set_child(Some(&label));

    window.show();

    thread::spawn(move || {
        Master::new(OsClipboard::new(clipboard_cm)).run().unwrap();
    });

    // we are using a closure to capture the label (else we could also use a normal function)
    let tick = move || {
        label.set_text(&clipboard.lock().unwrap().entries_text());
        // we could return glib::Continue(false) to stop our clock after this tick
        glib::Continue(true)
    };

    // executes the closure once every second
    glib::timeout_add_local(Duration::from_millis(100), tick);
}

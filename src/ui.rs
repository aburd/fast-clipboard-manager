use crate::{
    clipboard,
    composite_templates::{ClipboardEntry, Window},
    config, OsClipboard, APPLICATION_ID,
};
use clipboard_master::Master;
use gtk::gdk::Display;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, CssProvider, StyleContext};
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
    // The CSS "magic" happens here.
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("styles.css"));
    // We give the CssProvided to the default screen so the CSS rules we added
    // can be applied to our window.
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let window = Window::new(&app);

    info!("Starting fast clipboard...");
    let config = config::get_config().unwrap();
    let clipboard = Arc::new(Mutex::new(clipboard::get_clipboard(&config).unwrap()));
    let clipboard_cm = Arc::clone(&clipboard);

    let body = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let title = gtk::Label::new(None);
    title.set_text("Clipboard");
    let label = gtk::Label::new(None);
    label.set_text(&clipboard.lock().unwrap().entries_text());
    let entry = ClipboardEntry::new();

    body.append(&title);
    body.append(&label);
    body.append(&entry);
    window.set_child(Some(&body));

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

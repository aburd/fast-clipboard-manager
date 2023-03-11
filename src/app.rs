use crate::{
    clipboard,
    composite_templates::{ClipboardEntry, Window},
    config, OsClipboard, APPLICATION_ID,
};
use clipboard_master::Master;
use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{gio, glib};
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

fn build_entry(index: usize, cb_entry: &clipboard::Entry) -> ClipboardEntry {
    let keys = vec!["a", "s", "d", "f", "g", "h"];
    let key = keys.get(index % keys.len()).unwrap();
    let index_text = format!("{}:", key);
    let entry = ClipboardEntry::new();
    entry.set_entry_info(&index_text, &cb_entry.content());
    entry
}

fn render_app(clipboard: &Arc<Mutex<clipboard::Clipboard>>) -> gtk::Box {
    let app_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    for (i, e) in clipboard
        .lock()
        .unwrap()
        .list_entries()
        .into_iter()
        .enumerate()
    {
        let entry = build_entry(i, e);
        app_box.append(&entry);
    }
    app_box
}

fn setup_css() {
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
}

fn build_ui(app: &gtk::Application) {
    setup_css();

    let window = Window::new(&app);

    info!("Starting fast clipboard...");
    let config = config::get_config().unwrap();
    let clipboard = Arc::new(Mutex::new(clipboard::get_clipboard(&config).unwrap()));
    let clipboard_cm = Arc::clone(&clipboard);

    let mut app_box = render_app(&clipboard);

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .build();
    scrolled_window.set_child(Some(&app_box));

    window.set_child(Some(&scrolled_window));
    window.show();

    thread::spawn(move || {
        Master::new(OsClipboard::new(clipboard_cm)).run().unwrap();
    });

    // we are using a closure to capture the label (else we could also use a normal function)
    let tick = move || {
        app_box = render_app(&clipboard);
        scrolled_window.set_child(Some(&app_box));
        // we could return glib::Continue(false) to stop our clock after this tick
        glib::Continue(true)
    };

    // executes the closure once every second
    glib::timeout_add_local(Duration::from_millis(100), tick);
}

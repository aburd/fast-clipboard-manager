use crate::clipboard::Clipboard;
use crate::APPLICATION_ID;
use gtk::glib::ExitCode;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use std::sync::{Arc, Mutex};

pub fn build_and_run_gtk_app(clipboard: Arc<Mutex<Clipboard>>) -> ExitCode {
    let app = build_app(clipboard);
    app.run()
}

fn build_app(clipboard: Arc<Mutex<Clipboard>>) -> Application {
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

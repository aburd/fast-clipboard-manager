use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

const APPLICATION_ID: &str = "com.github.aburd.tiled-clipboard-manager";

fn main() {
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

    app.run();
}

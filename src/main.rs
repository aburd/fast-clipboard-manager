use clipboard_master::Master;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use log::{error, info};
use tiled_clipboard::clipboard::{self, Clipboard};
use tiled_clipboard::config;

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
        info!("Starting clipboard master");
        info!("Polling...");
    });
    app
}

fn main() {
    env_logger::init();

    info!("Starting tiled clipboard...");
    let config = config::get_config().unwrap();
    match clipboard::get_clipboard(&config) {
        Ok(clipboard) => {
            let _ = Master::new(clipboard).run();
            // let app = build_app(clipboard);
            // app.run();
        }
        Err(e) => {
            error!("Couldn't get clipboard: {}", e);
        }
    }
}

use log::{error, info};
use std::error::Error;
use std::fs::OpenOptions;

use clipboard_master::Master;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use tiled_clipboard::clipboard::Clipboard;
use tiled_clipboard::config::Config;

const APPLICATION_ID: &str = "com.github.aburd.tiled-clipboard-manager";

fn get_config() -> Result<Config, Box<dyn Error>> {
    let home_path = home::home_dir().unwrap();
    let dir_path = home_path.join(".config/titled_clipboard");
    Config::load(dir_path)
}

fn get_clipboard(config: &Config) -> Result<Clipboard, Box<dyn Error>> {
    let storage = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&config.config_dir.join("entries.json"))
        .unwrap();
    // TODO: setup key gracefully
    let key: &[u8; 32] = b"Thisisakeyof32bytesThisisakeyof3";
    let mut clipboard = Clipboard::new(storage, key.to_owned());
    clipboard.load()?;
    Ok(clipboard)
}

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

// fn main() -> Result<(), Box<dyn Error>> {
fn main() {
    env_logger::init();

    info!("Starting tiled clipboard...");
    let config = get_config().unwrap();
    match get_clipboard(&config) {
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

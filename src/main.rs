use log::info;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use tiled_clipboard::clipboard::Clipboard;
use tiled_clipboard::config::Config;

const APPLICATION_ID: &str = "com.github.aburd.tiled-clipboard-manager";

type ClipboardFromFile<'a> = Clipboard<'a, &'a File, &'a File>;

fn retrieve_config() -> Result<Config, Box<dyn Error>> {
    let home_path = home::home_dir().unwrap();
    let dir_path = home_path.join(".config/titled_clipboard");
    Config::load(dir_path)
}

fn retrieve_clipboard<'a>(f: &'a File) -> Result<ClipboardFromFile<'a>, Box<dyn Error>> {
    // TODO: setup key gracefully
    let key: &[u8; 32] = b"Thisisakeyof32bytesThisisakeyof3";
    let mut clipboard = Clipboard::new(BufReader::new(f), BufWriter::new(f), key);
    clipboard.load()?;
    Ok(clipboard)
}

fn build_app(clipboard: ClipboardFromFile) -> Application {
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

// fn main() -> Result<(), Box<dyn Error>> {
fn main() {
    env_logger::init();

    info!("Starting tiled clipboard...");
    let config = retrieve_config().unwrap();
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&config.config_dir.join("entries.json"))
        .unwrap();
    let clipboard = retrieve_clipboard(&f).unwrap();
    let app = build_app(clipboard);

    app.run();
}

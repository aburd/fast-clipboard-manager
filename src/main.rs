#![warn(clippy::all, rust_2018_idioms)]

use clipboard_master::Master;
use fast_clipboard::app::FCAppModel;
use fast_clipboard::clipboard;
use fast_clipboard::config;
use fast_clipboard::os_clipboard;
use fast_clipboard::os_clipboard::OsClipboard;
use log::debug;
use relm4::RelmApp;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    env_logger::init();
    debug!("logger initiated");

    let config = config::get_config().unwrap();
    let clipboard = Arc::new(Mutex::new(clipboard::get_clipboard(&config).unwrap()));
    debug!("config and clipboard loaded");

    debug!("starting app");
    let app = RelmApp::new("aburd.fast_clipboard_manager");
    app.run::<FCAppModel>(clipboard);
    debug!("app exited");
}

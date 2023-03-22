#![warn(clippy::all, rust_2018_idioms)]

mod gui;

use fast_clipboard::config;
use fast_clipboard::store;
use gui::FCAppModel;
use log::debug;
use relm4::RelmApp;
use std::sync::{Arc, Mutex};

const APPLICATION_ID: &str = "com.github.aburd.fast-clipboard-manager";

fn main() {
    env_logger::init();
    debug!("logger initiated");

    let config_file = config::get_config().unwrap();
    let clipboard = Arc::new(Mutex::new(store::get_clipboard(&config_file.path).unwrap()));
    debug!("config and clipboard loaded");

    debug!("starting app");
    let app = RelmApp::new(APPLICATION_ID);
    app.run::<FCAppModel>(clipboard);
    debug!("app exited");
}

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use fast_clipboard::app;
use log::{error, info};

fn run() -> Result<(), eframe::Error> {
    info!("Starting fast clipboard...");
    let app = app::build_app().unwrap();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    info!("starting egui");
    eframe::run_native("My egui App", options, Box::new(|_cc| Box::new(app)))
}

fn main() {
    env_logger::init();

    match run() {
        Ok(code) => {
            info!("gtk app exited with code: {:?}", code);
        }
        Err(e) => {
            error!("app error: {:?}", e);
        }
    };
}

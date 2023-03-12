use crate::{
    clipboard::{self, Clipboard, Entry, EntryKind},
    config,
};
use ::clipboard::{ClipboardContext, ClipboardProvider};
use clipboard_master::Master;
use clipboard_master::{CallbackResult, ClipboardHandler};
use eframe::egui;
use eframe::egui::*;
use log::{debug, error, info};
use std::error::Error;
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct AppError {}

#[derive(Debug)]
pub struct FCClipboardApp {
    clipboard: Arc<Mutex<Clipboard>>,
}

impl FCClipboardApp {
    pub fn new(clipboard: Arc<Mutex<Clipboard>>) -> FCClipboardApp {
        FCClipboardApp { clipboard }
    }

    fn handle_keypress(&mut self, ctx: &egui::Context) {
        let clipboard = self.clipboard.lock().unwrap();
        let copy_entry_at_idx = |idx: usize| {
            let entry = clipboard.get_entry(idx);
            // TODO: Maybe switch out how we set content
            set_content(&String::from_utf8(entry.content().to_owned()).unwrap()).unwrap();
        };
        let select_row = |idx: i32| {};
        if ctx.input(|i| i.key_pressed(Key::A)) {
            copy_entry_at_idx(1);
            select_row(0);
        }
        if ctx.input(|i| i.key_pressed(Key::S)) {
            copy_entry_at_idx(2);
            select_row(0);
        }
        if ctx.input(|i| i.key_pressed(Key::D)) {
            copy_entry_at_idx(3);
            select_row(0);
        }
        if ctx.input(|i| i.key_pressed(Key::F)) {
            copy_entry_at_idx(4);
            select_row(0);
        }
    }
}

impl eframe::App for FCClipboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.handle_keypress(ctx);
            ui.heading("Fast Clipboard");
            ui.vertical_centered(|ui| {
                let clipboard = self.clipboard.lock().unwrap();
                for (i, entry) in clipboard.list_entries().iter().enumerate() {
                    if i == 0 {
                        ui.label(format!("Current Entry: {}", entry));
                        continue;
                    }
                    ui.label(entry.to_string());
                }
            });
        });
    }
}

pub fn build_app() -> Result<FCClipboardApp, AppError> {
    let config = config::get_config().unwrap();
    let clipboard = Arc::new(Mutex::new(clipboard::get_clipboard(&config).unwrap()));

    let cloned = Arc::clone(&clipboard);
    thread::spawn(move || {
        Master::new(OsClipboard { clipboard: cloned })
            .run()
            .unwrap();
    });
    Ok(FCClipboardApp { clipboard })
}

struct OsClipboard {
    clipboard: Arc<Mutex<Clipboard>>,
}

impl ClipboardHandler for OsClipboard {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        debug!("User copied something into clipboard");
        let content = get_content();
        info!("got content: {}", content);
        let entry = Entry::new(&content.as_bytes().to_vec(), EntryKind::Text);
        let mut clipboard = self.clipboard.lock().unwrap();
        clipboard.add_entry(entry).unwrap();

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        error!("clipboard master error: {}", error);
        CallbackResult::Next
    }
}

fn get_content() -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.get_contents().unwrap()
}

fn set_content(data: &str) -> Result<(), Box<dyn Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(data.to_owned())
}

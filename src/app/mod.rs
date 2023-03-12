mod fonts;
mod spacing;
mod widgets;

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
    pub fn new(cc: &eframe::CreationContext<'_>, clipboard: Arc<Mutex<Clipboard>>) -> Self {
        fonts::configure(&cc.egui_ctx);
        spacing::configure(&cc.egui_ctx);
        FCClipboardApp { clipboard }
    }

    pub fn build_app(cc: &eframe::CreationContext<'_>) -> Result<Self, AppError> {
        let config = config::get_config().unwrap();
        let clipboard = Arc::new(Mutex::new(clipboard::get_clipboard(&config).unwrap()));

        let cloned = Arc::clone(&clipboard);
        thread::spawn(move || {
            Master::new(OsClipboard { clipboard: cloned })
                .run()
                .unwrap();
        });
        Ok(FCClipboardApp::new(cc, clipboard))
    }

    fn handle_keypress(&mut self, ctx: &egui::Context) {
        let mut copy_entry_at_idx = |idx: usize| {
            // TODO: Maybe switch out how we set content
            let entry = self.clipboard.lock().unwrap().get_entry(idx).clone();
            let s = String::from_utf8(entry.content().to_owned()).unwrap();
            set_content(&s).unwrap();
        };
        if ctx.input(|i| i.key_pressed(Key::Enter)) {
            std::process::exit(0);
        }
        if ctx.input(|i| i.key_pressed(Key::A)) {
            copy_entry_at_idx(1);
        }
        if ctx.input(|i| i.key_pressed(Key::S)) {
            copy_entry_at_idx(2);
        }
        if ctx.input(|i| i.key_pressed(Key::D)) {
            copy_entry_at_idx(3);
        }
        if ctx.input(|i| i.key_pressed(Key::F)) {
            copy_entry_at_idx(4);
        }
    }
}

impl eframe::App for FCClipboardApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        widgets::window_frame(ctx, frame, "Fast Clipboard Manager", |ui| {
            // Handle general events
            self.handle_keypress(ctx);
            // UI
            let clipboard = self.clipboard.lock().unwrap();
            widgets::clipboard_items_ui(ui, clipboard.list_entries());
        });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
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

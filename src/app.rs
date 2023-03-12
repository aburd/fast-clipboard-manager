use crate::{
    clipboard::{self, Clipboard, Entry, EntryKind},
    config,
};
use ::clipboard::{ClipboardContext, ClipboardProvider};
use clipboard_master::Master;
use clipboard_master::{CallbackResult, ClipboardHandler};
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
    fn new(clipboard: Arc<Mutex<Clipboard>>) -> FCClipboardApp {
        FCClipboardApp { clipboard }
    }
}

impl eframe::App for FCClipboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let clipboard = self.clipboard.lock().unwrap();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Fast Clipboard");
            ui.vertical_centered(|ui| {
                for entry in clipboard.list_entries().iter() {
                    ui.label(entry.to_string());
                }
            });
        });
    }
}

pub fn build_app() -> Result<FCClipboardApp, AppError> {
    let config = config::get_config().unwrap();
    let clipboard = Arc::new(Mutex::new(clipboard::get_clipboard(&config).unwrap()));

    //     controller.connect_key_pressed(
    //         clone!(@weak listbox, @weak clipboard => @default-return gtk::Inhibit(false), move |_, key, _n, _mod_type| {
    //             let clipboard = clipboard.lock().unwrap();
    //             let copy_entry_at_idx = |idx: usize| {
    //                 let entry = clipboard.get_entry(idx);
    //                 os_clipboard::set_content(&entry.content()).unwrap();
    //             };
    //             let select_row = |idx: i32| {
    //             };
    //             match key {
    //                 Key::Return => {
    //                     let selected = listbox.selected_row();
    //                     if let Some(selected) = selected {
    //                         let idx = selected.index();
    //                         copy_entry_at_idx(idx as usize);
    //                     }
    //                     gtk::Inhibit(true)
    //                 },
    //                 Key::a  => {
    //                     copy_entry_at_idx(0);
    //                     select_row(0);
    //                     gtk::Inhibit(true)
    //                 },
    //                 Key::s  => {
    //                     copy_entry_at_idx(1);
    //                     select_row(1);
    //                     gtk::Inhibit(true)
    //                 },
    //                 Key::d  => {
    //                     copy_entry_at_idx(2);
    //                     select_row(2);
    //                     gtk::Inhibit(true)
    //                 },
    //                 Key::f  => {
    //                     copy_entry_at_idx(3);
    //                     select_row(3);
    //                     gtk::Inhibit(true)
    //                 },
    //                 Key::g  => {
    //                     copy_entry_at_idx(4);
    //                     select_row(4);
    //                     gtk::Inhibit(true)
    //                 },
    //                 Key::j => {
    //                     let selected = listbox.selected_row();
    //                     debug!("selected is {:?}", selected);
    //                     if let Some(selected) = selected {
    //                         let idx = selected.index() + 1;
    //                         select_row(idx);
    //                     }
    //                     gtk::Inhibit(false)
    //                 }
    //                 _ => {
    //                     debug!("Key pressed: {:?}", key);
    //                     gtk::Inhibit(false)
    //                 }
    //             }
    //         }),
    //     );

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

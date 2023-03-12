use crate::clipboard::{Clipboard, Entry, EntryKind};
use clipboard::{ClipboardContext, ClipboardProvider};
use clipboard_master::{CallbackResult, ClipboardHandler};
use gtk::glib::Sender;
use gtk4 as gtk;
use log::{debug, error, info};
use std::error::Error;
use std::io;
use std::sync::{Arc, Mutex};

pub struct OsClipboard {
    sender: Sender<String>,
}

impl OsClipboard {
    pub fn new(sender: Sender<String>) -> Self {
        OsClipboard { sender }
    }
}

impl ClipboardHandler for OsClipboard {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        debug!("User copied something into clipboard");
        let content = get_content();
        info!("got content: {}", content);
        self.sender.send(content).unwrap();

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        error!("clipboard master error: {}", error);
        CallbackResult::Next
    }
}

pub fn get_content() -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.get_contents().unwrap()
}

pub fn set_content(data: &str) -> Result<(), Box<dyn Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(data.to_owned())
}

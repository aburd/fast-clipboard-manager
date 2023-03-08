use crate::clipboard::{Clipboard, Entry, EntryKind};
use clipboard::{ClipboardContext, ClipboardProvider};
use clipboard_master::{CallbackResult, ClipboardHandler};
use log::{debug, error, info};
use std::io;
use std::sync::{Arc, Mutex};

pub struct OsClipboardHandler {
    clipboard: Arc<Mutex<Clipboard>>,
}

impl OsClipboardHandler {
    pub fn new(clipboard: Arc<Mutex<Clipboard>>) -> Self {
        OsClipboardHandler { clipboard }
    }
}

impl ClipboardHandler for OsClipboardHandler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        debug!("User copied something into clipboard");
        let mut clipboard = self.clipboard.lock().unwrap();
        let new_entry = Entry::new(&os_clipboard_content().as_bytes().to_vec(), EntryKind::Text);
        info!("adding new entry: {}", new_entry);
        clipboard
            .add_entry(new_entry)
            .expect("Could not add entry to clipboard");
        clipboard.save().unwrap();

        info!("saved copied value as new entry into clipboard");
        debug!(
            "clipboard entries count: {:?}",
            clipboard.list_entries().len()
        );
        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        error!("clipboard master error: {}", error);
        CallbackResult::Next
    }
}

fn os_clipboard_content() -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.get_contents().unwrap()
}

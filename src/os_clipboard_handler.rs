use crate::clipboard::{Clipboard, Entry, EntryKind};
use clipboard::{ClipboardContext, ClipboardProvider};
use clipboard_master::{CallbackResult, ClipboardHandler};
use std::io;

impl ClipboardHandler for Clipboard {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let contents = ctx.get_contents().unwrap();
        self.add_entry(Entry::new(&contents.as_bytes().to_vec(), EntryKind::Text))
            .expect("Could not add entry to clipboard");
        println!("Clipboard: {:?}", self.list_entries());
        self.save().unwrap();

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        eprintln!("Error: {}", error);
        CallbackResult::Next
    }
}

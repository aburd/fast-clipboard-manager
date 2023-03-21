use crate::app::{AppInput, FCAppModel};
use clipboard_master::{CallbackResult, ClipboardHandler};
use log::{debug, error};
use relm4::ComponentSender;
use std::io;

pub struct OsClipboard {
    pub sender: ComponentSender<FCAppModel>,
}

impl ClipboardHandler for OsClipboard {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        debug!("User copied something into clipboard");
        self.sender.input(AppInput::ClipboardChanged);

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        error!("clipboard master error: {}", error);
        CallbackResult::Next
    }
}

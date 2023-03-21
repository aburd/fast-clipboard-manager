use log::{debug, error, info};
use relm4::gtk::gdk::prelude::*;
use relm4::gtk::gdk::Display;
use relm4::gtk::gio;
use relm4::gtk::glib;

// impl ClipboardHandler for OsClipboard {
//     fn on_clipboard_change(&mut self) -> CallbackResult {
//         debug!("User copied something into clipboard");
//         let content = get_content();
//         info!("got content: {}", content);
//         let entry = Entry::new(&content.as_bytes().to_vec(), EntryKind::Text);
//         let mut clipboard = self.clipboard.lock().unwrap();
//         clipboard.add_entry(entry).unwrap();
//         clipboard.save().unwrap();

//         CallbackResult::Next
//     }

//     fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
//         error!("clipboard master error: {}", error);
//         CallbackResult::Next
//     }
// }

// fn get_content() -> String {
//     let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
//     ctx.get_contents().unwrap()
// }

// fn set_content(data: &str) -> Result<(), Box<dyn Error>> {
//     let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
//     ctx.set_contents(data.to_owned())
// }

pub fn test_display(display: Display) {
    let clipboard = display.clipboard();
    clipboard.connect_changed(|clip| {
        debug!("Content notify: {}", clip);
        clip.read_async(
            &["text/plain"],
            glib::Priority::default(),
            gio::Cancellable::NONE,
            |res| {
                if let Ok((_stream, s)) = res {
                    // stream.read_bytes(, cancellable)
                    debug!("Clipboard content changed: {}", s);
                }
            },
        );
    });
}

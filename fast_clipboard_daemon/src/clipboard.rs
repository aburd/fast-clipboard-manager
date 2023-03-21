use fast_clipboard::store::{Entry, EntryKind};
use std::io::Read;
use wl_clipboard_rs::paste::{get_contents, ClipboardType, Error, MimeType, Seat};

pub struct Tracker {
    current: Option<Entry>,
}

impl Tracker {
    pub fn new() -> Self {
        Tracker { current: None }
    }

    pub fn poll(&mut self) {
        let result = get_contents(ClipboardType::Regular, Seat::Unspecified, MimeType::Text);
        match result {
            Ok((mut pipe, _)) => {
                let mut contents = vec![];
                pipe.read_to_end(&mut contents).unwrap();
                if self.current.is_some() && self.current.unwrap().content() == contents {
                    return;
                }
                let entry = Entry::new(&contents, EntryKind::Text);
                self.current = Some(entry);
            }
            // Err(Error::NoSeats) | Err(Error::ClipboardEmpty) | Err(Error::NoMimeType) => {
            //     // The clipboard is empty or doesn't contain text, nothing to worry about.
            // }
            // Err(err) => {
            //     error!("Err: {}", err);
            // }
            _ => {}
        }
    }
}

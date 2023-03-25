use log::debug;
use std::default::Default;
use std::io::Read;
use std::thread;
use std::time::Duration;
use wl_clipboard_rs::paste::{get_contents, ClipboardType, MimeType, Seat};

pub struct Tracker {
    current: Option<Vec<u8>>,
    poll_interval_ms: u64,
}

impl Default for Tracker {
    fn default() -> Self {
        Tracker {
            current: None,
            poll_interval_ms: 1000,
        }
    }
}

impl Tracker {
    pub fn new() -> Self {
        Tracker::default()
    }

    pub fn poll(&mut self) {
        loop {
            thread::sleep(Duration::from_millis(self.poll_interval_ms));
            self.read();
        }
    }

    fn read(&mut self) {
        if let Some(bytes) = read_clipboard() {
            if let Some(current) = &self.current {
                if &bytes == current {
                    return;
                }
            }
            debug!("Got new bytes: {}", String::from_utf8_lossy(&bytes));
            self.current = Some(bytes);
        }
    }
}

fn read_clipboard() -> Option<Vec<u8>> {
    let result = get_contents(ClipboardType::Regular, Seat::Unspecified, MimeType::Text);
    match result {
        Ok((mut pipe, _)) => {
            let mut contents = vec![];
            pipe.read_to_end(&mut contents).unwrap();
            Some(contents)
        }
        // Err(Error::NoSeats) | Err(Error::ClipboardEmpty) | Err(Error::NoMimeType) => {
        //     // The clipboard is empty or doesn't contain text, nothing to worry about.
        // }
        // Err(err) => {
        //     error!("Err: {}", err);
        // }
        _ => None,
    }
}

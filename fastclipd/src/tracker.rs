use log::debug;
use std::{
    future::Future,
    io::Read,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::{sync::broadcast::Sender, time::Duration};

use wl_clipboard_rs::paste::{get_contents, ClipboardType, MimeType, Seat};

pub type TrackerSender = Sender<Vec<u8>>;

pub struct Tracker {
    current: Option<Vec<u8>>,
    poll_interval_ms: u64,
}

impl Tracker {
    pub fn new() -> Self {
        let mut t = Tracker {
            current: None,
            poll_interval_ms: 1000,
        };
        t.current = t.read();
        t
    }

    fn read(&self) -> Option<Vec<u8>> {
        if let Some(bytes) = read_clipboard() {
            if Some(&bytes) != self.current.as_ref() {
                debug!("Got new bytes: {}", String::from_utf8_lossy(&bytes));
                return Some(bytes);
            }
        }
        None
    }
}

impl Future for Tracker {
    type Output = Vec<u8>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(bytes) = self.read() {
            let mut tracker = self.get_mut();
            tracker.current = Some(bytes.clone());
            Poll::Ready(bytes)
        } else {
            // tokio::time::sleep(Duration::from_millis(self.poll_interval_ms)).await;
            cx.waker().wake_by_ref();
            Poll::Pending
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

use log::debug;
use std::{
    future::Future,
    io::Read,
    pin::Pin,
    task::{Context, Poll},
    thread,
    time::Duration,
};
use tokio::sync::broadcast::Sender;

use wl_clipboard_rs::paste::{get_contents, ClipboardType, MimeType, Seat};

pub type TrackerSender = Sender<Vec<u8>>;

pub struct Tracker {
    current: Option<Vec<u8>>,
    poll_interval_ms: u64,
}

const POLL_INTERVAL: u64 = 1000;

impl Default for Tracker {
    fn default() -> Self {
        Self {
            current: None,
            poll_interval_ms: POLL_INTERVAL,
        }
    }
}

impl Tracker {
    pub fn new() -> Self {
        let mut t = Tracker::default();
        t.current = t.read();
        t
    }

    fn read(&self) -> Option<Vec<u8>> {
        if let Some(bytes) = Self::read_clipboard() {
            if Some(&bytes) != self.current.as_ref() {
                return Some(bytes);
            }
        }
        None
    }

    fn read_clipboard() -> Option<Vec<u8>> {
        debug!("reading clipboard");
        let result = get_contents(ClipboardType::Regular, Seat::Unspecified, MimeType::Text);
        match result {
            Ok((mut pipe, _)) => {
                let mut contents = vec![];
                pipe.read_to_end(&mut contents).unwrap();
                Some(contents)
            }
            _ => None,
        }
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
            let duration = Duration::from_millis(self.poll_interval_ms);
            let waker = cx.waker().clone();
            thread::spawn(move || {
                std::thread::sleep(duration);
                waker.wake();
            });
            Poll::Pending
        }
    }
}

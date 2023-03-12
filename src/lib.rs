pub mod app;
pub mod clipboard;
pub mod config;
pub mod os_clipboard;

pub const APPLICATION_ID: &str = "com.github.aburd.fast-clipboard-manager";
pub use os_clipboard::OsClipboard;

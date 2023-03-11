pub mod clipboard;
pub mod composite_templates;
pub mod config;
pub mod os_clipboard;
pub mod ui;

pub const APPLICATION_ID: &str = "com.github.aburd.fast-clipboard-manager";
pub use os_clipboard::OsClipboard;

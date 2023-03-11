pub mod app;
pub mod app_copy;
pub mod clipboard;
pub mod composite_templates;
pub mod config;
pub mod os_clipboard;

pub const APPLICATION_ID: &str = "com.github.aburd.fast-clipboard-manager";
pub use os_clipboard::OsClipboard;

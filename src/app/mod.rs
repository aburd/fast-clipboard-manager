mod fonts;

use crate::{
    clipboard::{self, Clipboard, Entry, EntryKind},
    config,
};
use ::clipboard::{ClipboardContext, ClipboardProvider};
use clipboard_master::Master;
use clipboard_master::{CallbackResult, ClipboardHandler};
use eframe::egui;
use eframe::egui::*;
use log::{debug, error, info};
use std::error::Error;
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct AppError {}

#[derive(Debug)]
pub struct FCClipboardApp {
    clipboard: Arc<Mutex<Clipboard>>,
}

impl FCClipboardApp {
    pub fn new(cc: &eframe::CreationContext<'_>, clipboard: Arc<Mutex<Clipboard>>) -> Self {
        fonts::configure_text_styles(&cc.egui_ctx);
        FCClipboardApp { clipboard }
    }

    pub fn build_app(cc: &eframe::CreationContext<'_>) -> Result<Self, AppError> {
        let config = config::get_config().unwrap();
        let clipboard = Arc::new(Mutex::new(clipboard::get_clipboard(&config).unwrap()));

        let cloned = Arc::clone(&clipboard);
        thread::spawn(move || {
            Master::new(OsClipboard { clipboard: cloned })
                .run()
                .unwrap();
        });
        Ok(FCClipboardApp::new(cc, clipboard))
    }

    fn handle_keypress(&mut self, ctx: &egui::Context) {
        let clipboard = self.clipboard.lock().unwrap();
        let copy_entry_at_idx = |idx: usize| {
            let entry = clipboard.get_entry(idx);
            // TODO: Maybe switch out how we set content
            set_content(&String::from_utf8(entry.content().to_owned()).unwrap()).unwrap();
        };
        let select_row = |idx: i32| {};
        if ctx.input(|i| i.key_pressed(Key::A)) {
            copy_entry_at_idx(1);
            select_row(0);
        }
        if ctx.input(|i| i.key_pressed(Key::S)) {
            copy_entry_at_idx(2);
            select_row(0);
        }
        if ctx.input(|i| i.key_pressed(Key::D)) {
            copy_entry_at_idx(3);
            select_row(0);
        }
        if ctx.input(|i| i.key_pressed(Key::F)) {
            copy_entry_at_idx(4);
            select_row(0);
        }
    }
}

impl eframe::App for FCClipboardApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        custom_window_frame(ctx, frame, "Fast Clipboard Manager", |ui| {
            // Handle general events
            self.handle_keypress(ctx);
            // UI
            let clipboard = self.clipboard.lock().unwrap();
            clipboard_items_ui(ui, clipboard.list_entries());
        });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
}

fn custom_window_frame(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    use eframe::egui::*;

    let panel_frame = egui::Frame {
        fill: ctx.style().visuals.window_fill(),
        rounding: 20.0.into(),
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(), // so the stroke is within the bounds
        ..Default::default()
    };

    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        let app_rect = ui.max_rect();

        let title_bar_height = 32.0;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };
        title_bar_ui(ui, frame, title_bar_rect, title);

        // Add the contents:
        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = title_bar_rect.max.y;
            rect
        }
        .shrink(4.0);
        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        add_contents(&mut content_ui);
    });
}

fn title_bar_ui(
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    title_bar_rect: eframe::epaint::Rect,
    title: &str,
) {
    use egui::*;

    let painter = ui.painter();

    let title_bar_response = ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());

    // Paint the title:
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );

    // Paint the line under the title:
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    // Interact with the title bar (drag to move window):
    if title_bar_response.double_clicked() {
        frame.set_maximized(!frame.info().window_info.maximized);
    } else if title_bar_response.is_pointer_button_down_on() {
        frame.drag_window();
    }

    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            window_controls(ui, frame);
        });
    });
}

/// Show some close for the native window.
fn window_controls(ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    let button_height = 12.0;

    let close_response = ui
        .add(Button::new(RichText::new("❌").size(button_height)))
        .on_hover_text("Close the window");
    if close_response.clicked() {
        frame.close();
    }
}

fn clipboard_items_ui(ui: &mut egui::Ui, entries: &[Entry]) {
    use egui::*;

    ui.vertical(|ui| {
        for (i, entry) in entries.iter().enumerate() {
            if i == 0 {
                ui.label(format!("Current Entry: {}", entry));
                continue;
            }
            ui.label(entry.to_string());
        }
    });
}

struct OsClipboard {
    clipboard: Arc<Mutex<Clipboard>>,
}

impl ClipboardHandler for OsClipboard {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        debug!("User copied something into clipboard");
        let content = get_content();
        info!("got content: {}", content);
        let entry = Entry::new(&content.as_bytes().to_vec(), EntryKind::Text);
        let mut clipboard = self.clipboard.lock().unwrap();
        clipboard.add_entry(entry).unwrap();

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        error!("clipboard master error: {}", error);
        CallbackResult::Next
    }
}

fn get_content() -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.get_contents().unwrap()
}

fn set_content(data: &str) -> Result<(), Box<dyn Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(data.to_owned())
}

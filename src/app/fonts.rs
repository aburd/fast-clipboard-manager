use eframe::egui;
use egui::{FontFamily, FontId, RichText, TextStyle};

#[inline]
pub fn heading2() -> TextStyle {
    TextStyle::Name("Heading2".into())
}

#[inline]
pub fn heading3() -> TextStyle {
    TextStyle::Name("ContextHeading".into())
}

pub fn configure(ctx: &egui::Context) {
    use FontFamily::{Monospace, Proportional};

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(25.0, Proportional)),
        (heading2(), FontId::new(22.0, Proportional)),
        (heading3(), FontId::new(19.0, Proportional)),
        (TextStyle::Body, FontId::new(16.0, Proportional)),
        (TextStyle::Monospace, FontId::new(12.0, Monospace)),
        (TextStyle::Button, FontId::new(12.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}

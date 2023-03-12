use eframe::egui;

pub fn configure(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::Vec2 { x: 20.0, y: 20.0 };
    ctx.set_style(style);
}

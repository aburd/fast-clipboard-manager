use super::fonts;
use crate::clipboard::Entry;
use eframe::egui;
use eframe::egui::*;

pub fn window_frame(
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

pub fn title_bar_ui(
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
pub fn window_controls(ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    let button_height = 12.0;

    let close_response = ui
        .add(Button::new(RichText::new("❌").size(button_height)))
        .on_hover_text("Close the window");
    if close_response.clicked() {
        frame.close();
    }
}

pub fn clipboard_items_ui(ui: &mut egui::Ui, entries: &[Entry]) {
    use egui::*;

    containers::ScrollArea::vertical().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.add_space(10.);
            ui.vertical(|ui| {
                ui.set_width(ui.available_width());
                ui.add_space(10.);
                clipboard_items_inner_ui(ui, entries);
            })
        });
    });
}

pub fn clipboard_items_inner_ui(ui: &mut egui::Ui, entries: &[Entry]) {
    Grid::new("entry-grid").show(ui, |ui| {
        // Present the current clipboard entry
        if let Some(entry) = entries.first() {
            ui.vertical(|ui| {
                ui.label(
                    RichText::new("Current")
                        .text_style(fonts::heading2())
                        .strong(),
                );
                ui.add_space(5.);
                ui.monospace(entry.to_string());
            });
            ui.end_row();
        }
        if entries.len() > 1 {
            ui.vertical(|ui| {
                ui.set_width(ui.available_width());
                for (i, entry) in entries[1..].iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(i.to_string())
                                .text_style(fonts::heading3())
                                .strong(),
                        );
                        ui.add_space(5.);
                        ui.monospace(entry.to_string());
                    });
                }
            });
        }
        ui.end_row();
    });
}

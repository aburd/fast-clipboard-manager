use crate::composite_templates::{list_box_row, model, row_data};
use gtk::{
    glib::{self, clone},
    prelude::*,
    ResponseType,
};
use gtk4 as gtk;
use list_box_row::ListBoxRow;
use row_data::RowData;

#[derive(Debug)]
pub struct AppError {}

pub fn build_app() -> Result<gtk::Application, AppError> {
    let app = gtk::Application::builder()
        .application_id(crate::APPLICATION_ID)
        .build();

    app.connect_activate(|app| {
        build_ui(app);
    });
    Ok(app)
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .default_width(320)
        .default_height(480)
        .application(application)
        .title("Custom Model")
        .build();

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);

    // Create our list store and specify that the type stored in the
    // list should be the RowData GObject we define at the bottom
    let model = model::Model::new();

    // And then create the UI part, the listbox and bind the list store
    // model to it. Whenever the UI needs to show a new row, e.g. because
    // it was notified that the model changed, it will call the callback
    // with the corresponding item from the model and will ask for a new
    // gtk::ListBoxRow that should be displayed.
    //
    // The gtk::ListBoxRow can contain any possible widgets.

    let listbox = gtk::ListBox::new();
    listbox.bind_model(
        Some(&model),
        clone!(@weak window => @default-panic, move |item| {
            ListBoxRow::new(
                item.downcast_ref::<RowData>()
                    .expect("RowData is of wrong type"),
            )
            .upcast::<gtk::Widget>()
        }),
    );

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_height(480)
        .min_content_width(360)
        .build();

    scrolled_window.set_child(Some(&listbox));

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    // Via the delete button we delete the item from the model that
    // is at the index of the selected row. Also deleting from the
    // model is immediately reflected in the listbox.
    let delete_button = gtk::Button::with_label("Delete");
    delete_button.connect_clicked(clone!(@weak model, @weak listbox => move |_| {
        let selected = listbox.selected_row();

        if let Some(selected) = selected {
            let idx = selected.index();
            model.remove(idx as u32);
        }
    }));
    hbox.append(&delete_button);

    vbox.append(&hbox);
    vbox.append(&scrolled_window);

    window.set_child(Some(&vbox));

    for i in 0..10 {
        model.append(&RowData::new(&format!("Name {i}"), "blah"));
    }

    window.show();
}

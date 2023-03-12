use crate::{
    clipboard,
    composite_templates::{model, row_data, ClipboardEntry, Window},
    config, os_clipboard, OsClipboard,
};
use clipboard_master::Master;
use gtk::{
    gdk::{Display, Event, Key},
    glib::{self, clone},
    prelude::*,
    CssProvider, EventControllerKey, StyleContext,
};
use gtk4 as gtk;
use log::{debug, info};
use row_data::RowData;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct AppError {}

fn setup_css() {
    // The CSS "magic" happens here.
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("styles.css"));
    // We give the CssProvided to the default screen so the CSS rules we added
    // can be applied to our window.
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

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
    setup_css();

    info!("Starting fast clipboard...");
    let config = config::get_config().unwrap();
    let clipboard = Arc::new(Mutex::new(clipboard::get_clipboard(&config).unwrap()));
    let clipboard_cm = Arc::clone(&clipboard);

    let window = Window::new(&application);

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
            ClipboardEntry::new(
                item.downcast_ref::<RowData>()
                    .expect("RowData is of wrong type"),
            )
            .upcast::<gtk::Widget>()
        }),
    );

    let controller = EventControllerKey::builder()
        .name("my-event-controller-key")
        .build();

    controller.connect_key_pressed(
        clone!(@weak listbox, @weak clipboard => @default-return gtk::Inhibit(false), move |_, key, n, _mod_type| {
            match key {
                Key::Return => {
                    let selected = listbox.selected_row();

                    if let Some(selected) = selected {
                        let idx = selected.index();
                        let clipboard = clipboard.lock().unwrap();
                        let entry = clipboard.get_entry(idx as usize);
                        os_clipboard::set_content(&entry.content());
                    }
                    gtk::Inhibit(true)
                },
                Key::a  => {
                    let clipboard = clipboard.lock().unwrap();
                    let entry = clipboard.get_entry(0);
                    os_clipboard::set_content(&entry.content());
                    gtk::Inhibit(true)
                },
                Key::s  => {
                    let clipboard = clipboard.lock().unwrap();
                    let entry = clipboard.get_entry(1);
                    os_clipboard::set_content(&entry.content());
                    gtk::Inhibit(true)
                },
                Key::d  => {
                    let clipboard = clipboard.lock().unwrap();
                    let entry = clipboard.get_entry(2);
                    os_clipboard::set_content(&entry.content());
                    gtk::Inhibit(true)
                },
                Key::f  => {
                    let clipboard = clipboard.lock().unwrap();
                    let entry = clipboard.get_entry(3);
                    os_clipboard::set_content(&entry.content());
                    gtk::Inhibit(true)
                },
                Key::g  => {
                    let clipboard = clipboard.lock().unwrap();
                    let entry = clipboard.get_entry(4);
                    os_clipboard::set_content(&entry.content());
                    gtk::Inhibit(true)
                },
                Key::j => {
                    let selected = listbox.selected_row();
                    debug!("selected is {:?}", selected);

                    if let Some(selected) = selected {
                        let idx = selected.index() + 1;
                        debug!("index is {}", idx);
                        let row = listbox.row_at_index(idx);
                        listbox.select_row(row.as_ref());
                    }
                    gtk::Inhibit(false)
                }
                _ => {
                    debug!("Key pressed: {:?}", key);
                    gtk::Inhibit(false)
                }
            }
        }),
    );
    listbox.add_controller(controller);

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_height(480)
        .min_content_width(360)
        .build();
    scrolled_window.set_child(Some(&listbox));
    vbox.append(&scrolled_window);

    window.set_child(Some(&vbox));

    for (i, entry) in clipboard.lock().unwrap().list_entries().iter().enumerate() {
        model.append(&RowData::new(
            &i.to_string(),
            &entry.content(),
            &entry.datetime,
        ));
    }

    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let os_clipboard = OsClipboard::new(tx);
    thread::spawn(move || {
        Master::new(os_clipboard).run().unwrap();
    });

    window.show();

    rx.attach(None, clone!(@weak model => @default-return glib::source::Continue(true), move |action| {
        let mut clipboard = clipboard_cm.lock().unwrap();
        let entry = clipboard::Entry::new(&action.as_bytes().to_vec(), clipboard::EntryKind::Text);
        clipboard.add_entry(entry).unwrap();
        clipboard.save().unwrap();
        let entries: Vec<_> = clipboard.list_entries().iter().enumerate().map(|(i, entry)| {
            RowData::new(
                &i.to_string(),
                &entry.content(),
                &entry.datetime,
            )
        }).collect();
        model.set_all(entries); 

        info!("saved copied value as new entry into clipboard");
        glib::source::Continue(true)
    }));
}

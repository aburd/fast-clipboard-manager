// mod fonts;
// mod spacing;
// mod widgets;

use crate::{
    clipboard::{self, Clipboard, Entry, EntryKind},
    os_clipboard,
};
use ::clipboard::{ClipboardContext, ClipboardProvider};
use log::{debug, error, info};
use relm4::gtk;
use relm4::gtk::gdk;
use relm4::gtk::gdk::Display;
use relm4::gtk::glib::clone;
use relm4::gtk::prelude::*;
use relm4::RelmWidgetExt;
use relm4::{ComponentParts, ComponentSender, SimpleComponent};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct AppError {}

#[derive(Debug)]
pub struct FCAppModel {
    clipboard: Arc<Mutex<Clipboard>>,
}

#[derive(Debug)]
pub enum AppInput {
    AddEntry(String),
    SelectEntry(usize),
    Quit,
}

#[derive(Debug)]
pub struct FCAppWidgets {
    labels: Vec<gtk::Label>,
}

impl SimpleComponent for FCAppModel {
    type Input = AppInput;
    type Output = ();
    type Init = Arc<Mutex<Clipboard>>;
    type Root = gtk::Window;
    type Widgets = FCAppWidgets;

    fn init_root() -> Self::Root {
        gtk::Window::builder()
            .default_width(300)
            .default_height(300)
            .build()
    }

    /// Initialize the UI and model.
    fn init(
        clipboard: Self::Init,
        window: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        debug!("Getting display...");
        let display = Display::default().unwrap();
        debug!("Got display: {:?}", display);
        os_clipboard::test_display(display);

        let clip_clone = clipboard.clone();
        let clip_inner = clip_clone.lock().unwrap();
        let entries = clip_inner.list_entries();
        let model = FCAppModel { clipboard };

        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();

        let labels: Vec<_> = entries
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let label = gtk::Label::new(Some(&format!("{}", entry)));
                label.set_margin_all(5);
                label
            })
            .collect();

        window.set_child(Some(&vbox));
        vbox.set_margin_all(5);
        for label in &labels {
            vbox.append(label);
        }

        let event_controller_key = gtk::EventControllerKey::builder()
            .name("keyboard_handler")
            .build();
        event_controller_key.connect_key_pressed(
            clone!(@strong sender => move |_event_controller, keyval, _keycode, _state| {
                match keyval {
                    gdk::Key::Escape => {
                        sender.input(AppInput::Quit);
                    }
                    gdk::Key::A => {
                        sender.input(AppInput::SelectEntry(0));
                    }
                    gdk::Key::S => {
                        sender.input(AppInput::SelectEntry(1));
                    }
                    gdk::Key::D => {
                        sender.input(AppInput::SelectEntry(2));
                    }
                    gdk::Key::F => {
                        sender.input(AppInput::SelectEntry(3));
                    }
                    _ => {}
                }
                // let mut copy_entry_at_idx = |idx: usize| {
                //     // TODO: Maybe switch out how we set content
                //     let entry = self.clipboard.lock().unwrap().get_entry(idx).clone();
                //     let s = String::from_utf8(entry.content().to_owned()).unwrap();
                //     set_content(&s).unwrap();
                // };
                // if ctx.input(|i| i.key_pressed(Key::Enter)) {
                //     std::process::exit(0);
                // }
                // if ctx.input(|i| i.key_pressed(Key::A)) {
                //     copy_entry_at_idx(1);
                // }
                // if ctx.input(|i| i.key_pressed(Key::S)) {
                //     copy_entry_at_idx(2);
                // }
                // if ctx.input(|i| i.key_pressed(Key::D)) {
                //     copy_entry_at_idx(3);
                // }
                // if ctx.input(|i| i.key_pressed(Key::F)) {
                //     copy_entry_at_idx(4);
                // }
                gtk::Inhibit(true)
            }),
        );

        let widgets = FCAppWidgets { labels };

        ComponentParts { model, widgets }
    }
}

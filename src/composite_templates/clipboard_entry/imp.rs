use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};
use gtk4 as gtk;

#[derive(CompositeTemplate, Default)]
#[template(file = "entry.ui")]
pub struct ClipboardEntry {
    #[template_child]
    pub index_text: TemplateChild<gtk::Label>,
    #[template_child]
    pub content_text: TemplateChild<gtk::Label>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for ClipboardEntry {
    const NAME: &'static str = "FCClipboardEntry";
    type Type = super::ClipboardEntry;
    type ParentType = gtk::Grid;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for ClipboardEntry {}

// Trait shared by all widgets
impl WidgetImpl for ClipboardEntry {}

// Trait shared by all buttons
impl GridImpl for ClipboardEntry {}

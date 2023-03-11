use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::{
    glib::{self, ParamSpec, Properties, Value},
    prelude::*,
};
use gtk4 as gtk;
use std::cell::RefCell;

use crate::composite_templates::row_data::RowData;

#[derive(CompositeTemplate, Default, Properties, Debug)]
#[properties(wrapper_type = super::ClipboardEntry)]
#[template(file = "entry.ui")]
pub struct ClipboardEntry {
    #[template_child]
    pub index_text: TemplateChild<gtk::Label>,
    #[template_child]
    pub content_text: TemplateChild<gtk::Label>,
    #[template_child]
    pub date_text: TemplateChild<gtk::Label>,
    #[property(get, set, construct_only)]
    row_data: RefCell<Option<RowData>>,
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

impl ObjectImpl for ClipboardEntry {
    fn properties() -> &'static [ParamSpec] {
        Self::derived_properties()
    }

    fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
        self.derived_set_property(id, value, pspec)
    }

    fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
        self.derived_property(id, pspec)
    }

    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        let item = self.row_data.borrow();
        let item = item.as_ref().cloned().unwrap();

        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

        // Create the label and spin button that shows the two values
        // of the item. We bind the properties for the two values to the
        // corresponding properties of the widgets so that they are automatically
        // updated whenever the item is changing. By specifying SYNC_CREATE the
        // widget will automatically get the initial value of the item set.
        //
        // In case of the spin button the binding is bidirectional, that is any
        // change of value in the spin button will be automatically reflected in
        // the item.
        item.bind_property("index", &self.index_text.get(), "label")
            .sync_create()
            .build();
        item.bind_property("content", &self.content_text.get(), "label")
            .sync_create()
            .build();
        item.bind_property("date", &self.date_text.get(), "label")
            .sync_create()
            .build();
    }
}

// Trait shared by all widgets
impl WidgetImpl for ClipboardEntry {}

// Trait shared by all buttons
impl GridImpl for ClipboardEntry {}

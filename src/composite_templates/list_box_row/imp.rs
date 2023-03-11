use std::cell::RefCell;

use gtk::{
    glib::{self, clone, ParamSpec, Properties, Value},
    prelude::*,
    subclass::prelude::*,
};
use gtk4 as gtk;

use crate::composite_templates::row_data::RowData;

#[derive(Default, Properties, Debug)]
#[properties(wrapper_type = super::ListBoxRow)]
pub struct ListBoxRow {
    #[property(get, set, construct_only)]
    row_data: RefCell<Option<RowData>>,
}

#[glib::object_subclass]
impl ObjectSubclass for ListBoxRow {
    const NAME: &'static str = "ExListBoxRow";
    type ParentType = gtk::ListBoxRow;
    type Type = super::ListBoxRow;
}

impl ObjectImpl for ListBoxRow {
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
        let label = gtk::Label::new(None);
        item.bind_property("content", &label, "label")
            .sync_create()
            .build();
        hbox.append(&label);
        let label = gtk::Label::new(None);
        item.bind_property("date", &label, "label")
            .sync_create()
            .build();
        hbox.append(&label);
        obj.set_child(Some(&hbox));
    }
}

impl WidgetImpl for ListBoxRow {}
impl ListBoxRowImpl for ListBoxRow {}

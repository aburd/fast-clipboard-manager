use glib::subclass::prelude::*;
use gtk::{
    glib::{self, ParamSpec, Properties, Value},
    prelude::*,
};
use gtk4 as gtk;
use std::cell::RefCell;

// The actual data structure that stores our values. This is not accessible
// directly from the outside.
#[derive(Default, Properties)]
#[properties(wrapper_type = super::RowData)]
pub struct RowData {
    #[property(get, set)]
    index: RefCell<Option<String>>,
    #[property(get, set)]
    content: RefCell<Option<String>>,
    #[property(get, set)]
    date: RefCell<Option<String>>,
}

// Basic declaration of our type for the GObject type system
#[glib::object_subclass]
impl ObjectSubclass for RowData {
    const NAME: &'static str = "RowData";
    type Type = super::RowData;
}

// The ObjectImpl trait provides the setters/getters for GObject properties.
// Here we need to provide the values that are internally stored back to the
// caller, or store whatever new value the caller is providing.
//
// This maps between the GObject properties and our internal storage of the
// corresponding values of the properties.
impl ObjectImpl for RowData {
    fn properties() -> &'static [ParamSpec] {
        Self::derived_properties()
    }

    fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
        self.derived_set_property(id, value, pspec)
    }

    fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
        self.derived_property(id, pspec)
    }
}

use glib::Object;
use gtk::glib;
use gtk4 as gtk;

mod imp;

glib::wrapper! {
    pub struct ClipboardEntry(ObjectSubclass<imp::ClipboardEntry>)
        @extends gtk::Grid, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable,
                    gtk::Buildable, gtk::ConstraintTarget;
}

impl ClipboardEntry {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for ClipboardEntry {
    fn default() -> Self {
        Self::new()
    }
}

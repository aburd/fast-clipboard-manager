use gtk::glib;
use gtk::subclass::prelude::*;
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
        glib::Object::builder().build()
    }

    pub fn set_entry_info(&self, index_text: &str, content_text: &str) {
        let imp = self.imp();
        imp.index_text.set_text(index_text);
        imp.content_text.set_text(content_text);
    }
}

impl Default for ClipboardEntry {
    fn default() -> Self {
        Self::new()
    }
}

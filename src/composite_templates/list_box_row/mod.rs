mod imp;

use gtk::glib;
use gtk4 as gtk;

use crate::composite_templates::row_data::RowData;

glib::wrapper! {
    pub struct ListBoxRow(ObjectSubclass<imp::ListBoxRow>)
        @extends gtk::Widget, gtk::ListBoxRow;
}

impl ListBoxRow {
    pub fn new(row_data: &RowData) -> Self {
        glib::Object::builder()
            .property("row-data", &row_data)
            .build()
    }
}

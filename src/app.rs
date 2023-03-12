use crate::{clipboard, config, OsClipboard};
use clipboard_master::Master;
use log::{debug, info};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct AppError {}

pub fn build_app() -> Result<(), AppError> {
    build_ui();
    Ok(())
}

fn build_ui() {
    info!("Starting fast clipboard...");
    let config = config::get_config().unwrap();
    let clipboard = Arc::new(Mutex::new(clipboard::get_clipboard(&config).unwrap()));
    let clipboard_cm = Arc::clone(&clipboard);

    //     controller.connect_key_pressed(
    //         clone!(@weak listbox, @weak clipboard => @default-return gtk::Inhibit(false), move |_, key, _n, _mod_type| {
    //             let clipboard = clipboard.lock().unwrap();
    //             let copy_entry_at_idx = |idx: usize| {
    //                 let entry = clipboard.get_entry(idx);
    //                 os_clipboard::set_content(&entry.content()).unwrap();
    //             };
    //             let select_row = |idx: i32| {
    //             };
    //             match key {
    //                 Key::Return => {
    //                     let selected = listbox.selected_row();
    //                     if let Some(selected) = selected {
    //                         let idx = selected.index();
    //                         copy_entry_at_idx(idx as usize);
    //                     }
    //                     gtk::Inhibit(true)
    //                 },
    //                 Key::a  => {
    //                     copy_entry_at_idx(0);
    //                     select_row(0);
    //                     gtk::Inhibit(true)
    //                 },
    //                 Key::s  => {
    //                     copy_entry_at_idx(1);
    //                     select_row(1);
    //                     gtk::Inhibit(true)
    //                 },
    //                 Key::d  => {
    //                     copy_entry_at_idx(2);
    //                     select_row(2);
    //                     gtk::Inhibit(true)
    //                 },
    //                 Key::f  => {
    //                     copy_entry_at_idx(3);
    //                     select_row(3);
    //                     gtk::Inhibit(true)
    //                 },
    //                 Key::g  => {
    //                     copy_entry_at_idx(4);
    //                     select_row(4);
    //                     gtk::Inhibit(true)
    //                 },
    //                 Key::j => {
    //                     let selected = listbox.selected_row();
    //                     debug!("selected is {:?}", selected);
    //                     if let Some(selected) = selected {
    //                         let idx = selected.index() + 1;
    //                         select_row(idx);
    //                     }
    //                     gtk::Inhibit(false)
    //                 }
    //                 _ => {
    //                     debug!("Key pressed: {:?}", key);
    //                     gtk::Inhibit(false)
    //                 }
    //             }
    //         }),
    //     );

    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        Master::new(OsClipboard::new(tx)).run().unwrap();
    });
}

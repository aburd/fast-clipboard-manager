use fast_clipboard::ui;
use fast_clipboard::ui::AppError;
use gtk::glib::ExitCode;
use gtk::prelude::*;
use gtk4 as gtk;
use log::{error, info};

fn run() -> Result<ExitCode, AppError> {
    info!("building gtk window");
    let app = ui::build_app()?;
    let exit_code = app.run();
    Ok(exit_code)
}

fn main() {
    env_logger::init();

    match run() {
        Ok(code) => {
            info!("gtk app exited with code: {:?}", code);
        }
        Err(e) => {
            error!("app error: {:?}", e);
        }
    }
}

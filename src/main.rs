use fast_clipboard::app_copy;
use gtk::glib::ExitCode;
use gtk::prelude::*;
use gtk4 as gtk;
use log::{error, info};

fn run() -> Result<ExitCode, app_copy::AppError> {
    info!("building gtk window");
    let application = app_copy::build_app()?;
    let exit_code = application.run();
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

use gtk::glib::ExitCode;
use gtk::prelude::*;
use log::{error, info};
use tiled_clipboard::ui;
use tiled_clipboard::ui::AppError;

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

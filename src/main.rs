use fast_clipboard::app;
use log::{error, info};

fn run() -> Result<(), app::AppError> {
    info!("building gtk window");
    app::build_app()?;
    Ok(())
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

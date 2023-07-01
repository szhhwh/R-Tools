use log::info;
use randapp::conf::AppConf;
use tauri::{App, command};

pub fn init(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!("app setup");

    let app_config = AppConf::read();

    Ok(())
}

use log::info;
use randapp::conf::AppConf;
use tauri::App;

pub fn init(_app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!("app setup");

    let _app_config = AppConf::read().write();

    Ok(())
}

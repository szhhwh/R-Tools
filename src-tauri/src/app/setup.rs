use log::info;
use rtools::conf::AppConf;

pub fn init(_app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    info!("app setup");
    let _app_config = AppConf::read().write();

    Ok(())
}

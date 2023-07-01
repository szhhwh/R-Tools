use log::info;
use randapp::conf::AppConf;
use tauri::{App, command};

pub fn init(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!("app setup");

    let app_config = AppConf::read();

    Ok(())
}

/// close_splashscreen | 关闭 splashscreen
#[command]
fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}
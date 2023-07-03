use log::info;
use rtools::conf::AppConf;
/// tauri命令模块
use tauri::{command, Manager, AppHandle};

/// close_splashscreen | 关闭 splashscreen
#[command]
pub fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

/// 保存配置文件
#[command]
pub fn save_config(_app: AppHandle, data: serde_json::Value) -> Result<(), &'static str> {
    info!("saving config");
    AppConf::read().modify(serde_json::json!(data)).write();
    Ok(())
}
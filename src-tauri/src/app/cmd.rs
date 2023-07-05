use log::{info, error};
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
    match AppConf::read().modify(serde_json::json!(data)).write() {
        Ok(_) => Ok(()),
        Err(v) => {
            error!("{}", v);
            Err("无法保存配置文件")
        }
    }
}

/// 向前端返回config
#[command]
pub fn return_config() -> Result<serde_json::Value, &'static str> {
    let config = match serde_json::to_value(AppConf::read()) {
        Ok(v) => v,
        Err(a) => {
            error!("无法读取本地配置 {}", a);
            return Err("无法读取本地配置")
        }
    };
    Ok(config)
}
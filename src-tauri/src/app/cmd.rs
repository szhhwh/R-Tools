use log::{error, info};
use rtools::conf::AppConf;
/// tauri命令模块
use tauri::{command, AppHandle, Manager, Config};

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
    let config = match AppConf::read().to_json() {
        Ok(v) => v,
        Err(a) => {
            error!("无法读取本地配置 {}", a);
            return Err("无法读取本地配置");
        }
    };
    Ok(config)
}

pub fn get_tauri_conf() -> Option<Config> {
    let config_file = include_str!("../../tauri.conf.json");
    let config = serde_json::from_str(config_file).expect("failed to parse tauri.conf.json");
    Some(config)
}

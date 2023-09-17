use crate::app::readers::calareader;
use log::{error, info};
use rtools::{conf::AppConf, get_tauri_conf};
/// tauri命令模块
use tauri::{command, AppHandle, Manager};

/// 关闭 splashscreen
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

/// 向前端返回工作簿中包含的表名称
/// 
/// 前端以**数组**的方式收到包含的值
#[command]
pub fn return_sheet_names() -> serde_json::Value {
    let config = AppConf::read();
    match calareader::CALA::sheet_names(config.cala_path) {
        Ok(v) => v,
        Err(_) => {
            error!("excel file path unavaliable");
            serde_json::Value::Null
        }
    }
}

/// 向前端返回当前版本号
/// # Return
/// Strings
#[command]
pub fn return_version() -> Result<String, &'static str> {
    match get_tauri_conf() {
        Some(config) => {
            match config.package.version {
                Some(e) => Ok(e),
                None => Err("Can't get application's version")
            }
        },
        None => Err("Can't get application's version")
    }
}
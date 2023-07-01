/// tauri命令模块
use tauri::{command, AppHandle, Manager};

#[command]
pub fn generate_randnum() {}

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
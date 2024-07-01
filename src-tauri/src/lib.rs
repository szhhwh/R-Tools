use error::AppError;
use tauri::Config;
use std::{
    fs,
    path::{Path, PathBuf}, ffi::OsStr,
};

pub mod conf;
pub mod error;

/// 获取应用根目录
///
/// 返回 **PathBuf**
pub fn app_root() -> PathBuf {
    let path = tauri::api::path::home_dir().unwrap().join(".r-tools");
    path
}

/// 判断传入的path是否存在
///
/// ## Return
/// 返回布尔值：为真时路径存在
pub fn exists<P>(path: P) -> bool
where P: AsRef<Path> + AsRef<OsStr>
{
    Path::new(&path).exists()
}

/// 创建文件
pub fn create_file<P>(filepath: P) -> Result<(), AppError>
where
    P: AsRef<Path>,
{
    let file_path = filepath.as_ref();
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::File::create(file_path)?;
    Ok(())
}

/// 读取tauri.conf.json
/// 
/// ## Return
/// 返回 tauri::Config
pub fn get_tauri_conf() -> Option<Config> {
    let config_file = include_str!("../tauri.conf.json");
    let config = serde_json::from_str(config_file).expect("failed to parse tauri.conf.json");
    Some(config)
}
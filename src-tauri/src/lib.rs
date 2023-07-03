use anyhow::Result;
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
/// 返回 **bool**
pub fn exists<P>(path: P) -> bool
where P: AsRef<Path> + AsRef<OsStr>
{
    Path::new(&path).exists()
}

/// 创建文件
pub fn create_file<P>(filename: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let filename = filename.as_ref();
    if let Some(parent) = filename.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::File::create(filename)?;
    Ok(())
}

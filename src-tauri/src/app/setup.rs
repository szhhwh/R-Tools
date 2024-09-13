use std::cmp::Ordering;

use log::info;
use rtools::{conf::AppConf, get_tauri_conf};

pub fn init(_app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    info!("app setup");
    // 初始化配置文件
    let _init_config = AppConf::read().write();

    // 读取软件版本
    let package_version = get_tauri_conf().unwrap().package.version.unwrap();
    info!("package version: {}", &package_version);
    let current_version = semver::Version::parse(package_version.as_str()).unwrap();

    let app_config = AppConf::read();
    match app_config.lastversion.cmp(&current_version) {
        Ordering::Equal => {
            info!("You are running the latest version of the app.");
        },
        Ordering::Greater => {
            info!("You had installed a newer version before.");
        },
        Ordering::Less => {
            info!("You are running the latest version of the app.");
            // 写入当前安装的版本号
            let _ = AppConf::read().modify(serde_json::json!({"lastversion": current_version})).write();
        }
    }
    Ok(())
}

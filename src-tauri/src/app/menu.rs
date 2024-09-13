use log::debug;
use rtools::get_tauri_conf;
use tauri::{CustomMenuItem, Manager, Menu, Submenu, WindowMenuEvent};

use super::windows;

pub fn init() -> Menu {
    // 帮助菜单
    let help = Submenu::new(
        "帮助",
        Menu::new()
            .add_item(CustomMenuItem::new("setting", "首选项"))
            .add_native_item(tauri::MenuItem::Separator)
            .add_item(CustomMenuItem::new("about", "关于"))
            .add_item(CustomMenuItem::new("update_dialog", "更新日志")),
    );

    Menu::new().add_submenu(help)
}

/// 菜单栏切换钩子
pub fn menu_handler(event: WindowMenuEvent<tauri::Wry>) {
    let win = Some(event.window()).unwrap();
    let app = win.app_handle();
    let menu_id = event.menu_item_id();
    // let menu_handle = win.menu_handle();

    match menu_id {
        "about" => {
            let message = get_tauri_conf().unwrap();
            tauri::api::dialog::message(
                app.get_window("core").as_ref(),
                "R-Tools",
                format!(
                    "Made By 狮子耗耗\nCurrent Version: {}",
                    message.package.version.unwrap()
                ),
            )
        }
        "update_dialog" => {
            debug!("open update_dialog window");
            windows::cmd::update_dialog(app)
        }
        "setting" => {
            debug!("open setting window");
            windows::cmd::setting_center(app)
        }
        _ => {}
    }
}

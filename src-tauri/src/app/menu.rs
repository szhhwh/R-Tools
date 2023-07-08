use log::debug;
use tauri::{CustomMenuItem, Manager, Menu, Submenu, WindowMenuEvent};

use super::{cmd::get_tauri_conf, windows};

pub fn init() -> Menu {
    // 设置菜单
    let preferences_menu = Submenu::new(
        "设置",
        Menu::new().add_item(CustomMenuItem::new("setting", "首选项")),
    );

    // 帮助菜单
    let help = Submenu::new(
        "帮助",
        Menu::new().add_item(CustomMenuItem::new("about", "关于")),
    );

    Menu::new().add_submenu(preferences_menu).add_submenu(help)
}

/// 菜单栏切换Handle
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
        "setting" => {
            debug!("opening setting window");
            windows::cmd::setting_center(app, "".into())
        }
        _ => {
        }
    }
}

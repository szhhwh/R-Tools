// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

use tauri_plugin_log::{
    fern::colors::{Color, ColoredLevelConfig},
    LogTarget,
};

mod app;
use app::{cmd, calarand, setup, menu};

fn main() {
    let mut log = tauri_plugin_log::Builder::default()
        .targets([LogTarget::Stdout, LogTarget::Webview])
        .level(log::LevelFilter::Trace);

    if cfg!(debug_assertions) {
        log = log.with_colors(ColoredLevelConfig {
            error: Color::Red,
            warn: Color::Yellow,
            debug: Color::Blue,
            info: Color::BrightGreen,
            trace: Color::Cyan,
        });
    }

    tauri::Builder::default()
        .plugin(log.build())
        .invoke_handler(tauri::generate_handler![
            calarand::generate_randnum,
            calarand::reset,
            calarand::return_list_number,
            calarand::init_list,
            calarand::return_randresult,
            cmd::return_config,
            cmd::save_config,
            cmd::close_splashscreen,
        ])
        .on_menu_event(menu::menu_handler)
        .menu(menu::init())
        .setup(setup::init)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use csvread::{CONF, CSV};
use rand::prelude::*;
use std::process;
use std::sync::atomic::{AtomicU32, Ordering};

static COUNT: AtomicU32 = AtomicU32::new(0);

#[tauri::command]
// 随机数生成
fn generate_randnum() -> String {
    let config = CONF::new().build().unwrap_or_else(|err| {
        println!("err info: {err}");
        process::exit(1)
    });
    let csv = CSV::new().read(config.csv_path).unwrap();
    if COUNT.load(Ordering::Relaxed) <= csv.list.len() as u32 {
        COUNT.fetch_add(1, Ordering::Relaxed);
        randlist(csv)
    } else {
        String::from("123")
    }
}

fn randlist(csv: CSV) -> String {
    let num = rand::thread_rng().gen_range(1..=csv.list.len()) as u32;
    let num = csv.list.get(&num);
    match num {
        Some(n) => String::from(n),
        None => String::from("Out of index"),
    }
}

#[tauri::command]
fn reset() {
    COUNT.store(0, std::sync::atomic::Ordering::Relaxed);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_randnum,reset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

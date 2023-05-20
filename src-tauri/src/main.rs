// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::prelude::*;

#[tauri::command]
// mut condition: Vec<u32>, times: i32 
fn generate_randnum(min: u32, max: u32) -> u32 {
    let randnum: u32 = rand::thread_rng().gen_range(min..=max);
    randnum
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_randnum])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::prelude::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn generate_randnum(min: u32, max:  u32, mut condition:  Vec<u32>) -> u32 {

    let mut randnum: u32 = rand::thread_rng().gen_range(min..=max);
    
    let mut i: i32 = 0;// times count
    loop {
        while condition.contains(&randnum) {
            randnum = rand::thread_rng().gen_range(min..=max);
        }
    
        condition.push(randnum);
        
        if i>=times {
            break;
        }
    }




    randnum
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, generate_randnum])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

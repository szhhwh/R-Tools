// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::prelude::*;

#[tauri::command]
// mut condition: Vec<u32>, times: i32 
fn generate_randnum(min: u32, max: u32) -> u32 {

    let randnum: u32 = rand::thread_rng().gen_range(min..=max);

    randnum
    // let mut randnum: u32 = rand::thread_rng().gen_range(min..=max);
    
    // let mut i: i32 = 0;// times count
    // loop {
    //     while condition.contains(&randnum) {
    //         randnum = rand::thread_rng().gen_range(min..=max);
    //     }

    //     condition.push(randnum);

    //     i = i + 1;
    //     if i>=times {
    //         break;
    //     }
    // }

    // condition
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_randnum])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

use csvread::{CONF, CSV};
use rand::prelude::*;
use std::collections::HashMap;
use std::process;
use std::sync::Mutex;

lazy_static! {
    // define global list object
    static ref LIST: HashMap<u32, String> = {
        let config = CONF::new().build().unwrap_or_else(|err| {
            println!("err info: {err}");
            process::exit(1)
        });
        let csv = CSV::new().read(config.csv_path).unwrap();
        csv.list
    };
    // define global record object
    static ref RECORD: Mutex<Vec<u32>> = Mutex::new(vec![]);
}

#[tauri::command]
// generate_randnum
fn generate_randnum(times: u32) -> String {
    // counter
    let mut count: u32 = 0;
    // 判断record是否为空，空数组则添加一个随机数
    if RECORD.lock().unwrap().is_empty() {
        let num = rand();
        RECORD.lock().unwrap().push(num);
        count += 1;
    }
    // 进入生成循环
    if (LIST.len() > RECORD.lock().unwrap().len()) & (times > count) {
        let mut num; // 定义临时数字
        for _i in 1..=times {
            // 循环times次
            loop {
                num = rand(); // 获取随机数

                // 判断num是否在record中，以及record的长度是否超出list的长度
                let temp1 = LIST.len() < RECORD.lock().unwrap().len();
                let temp2 = RECORD.lock().unwrap().contains(&num);
                if temp1 | temp2 {
                    continue;
                } else {
                    // push num into record
                    RECORD.lock().unwrap().push(num);
                    break;
                }
            }
        }
    }
    // 输出随机数
    output()
}

fn output() -> String {
    let mut result = String::new(); // result 输出Strings
    let lenth = RECORD.lock().unwrap().len(); // 获取record长度
    for i in 0..lenth {
        let value = match LIST.get(match RECORD.lock().unwrap().get(i) {
            Some(e) => e,
            None => process::exit(1),
        }) {
            Some(s) => s,
            None => process::exit(1),
        };
        if i >= 1 {
            result.push_str(",")
        }
        result.push_str(value);
    }
    result
}

fn rand() -> u32 {
    rand::thread_rng().gen_range(0..LIST.len()) as u32
}

#[tauri::command]
// 重置
fn reset() {
    let lenth = RECORD.lock().unwrap().len();
    for _i in 0..lenth {
        RECORD.lock().unwrap().swap_remove(0);
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_randnum, reset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

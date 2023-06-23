// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

use rand::prelude::*;
use std::collections::HashMap;
use std::process;
use std::sync::Mutex;
use tauri::Manager;

// 私有包
use randapp::config::appconfig;
use randapp::freader::csvreader;

// 初始化全局变量
lazy_static! {
    // define global list object
    static ref LIST: Mutex<HashMap<u32, String>> = Mutex::new({
        println!("load list");
        let config = appconfig::CONF::new().build().unwrap_or_else(|err| {
            println!("err create global list object: {err}");
            process::exit(1)
        });
        let csv = csvreader::CSV::new().read(config.csv_path).unwrap();
        csv.list
    });
    // define global record object
    static ref RECORD: Mutex<Vec<u32>> = Mutex::new(vec![]);
}

#[tauri::command]
fn init_list() -> Result<(), String> {
    match appconfig::CONF::new().build() {
        Ok(c) => {
            match csvreader::CSV::new().read(c.csv_path) {
                Ok(_) => Ok(()),
                Err(e) => {
                    println!("init_error:{}", e);
                    Err(String::from("csv 路径无效"))
                }
            }
        },
        Err(e) => {
            println!("init_error:{}", e);
            Err(String::from("config 读取失败"))
        }
    }
}

#[tauri::command]
// generate_randnum
fn generate_randnum(times: u32, app_handle: tauri::AppHandle) -> Result<(), String> {
    let mut record = RECORD.lock().unwrap();
    let list = LIST.lock().unwrap();

    // counter 计数器
    let mut count: u32 = 0;
    // 判断record是否为空，空数组则添加一个随机数
    if record.is_empty() {
        let num = rand::thread_rng().gen_range(1..=list.len()) as u32;
        record.push(num);
        count += 1;
    }
    // 进入生成循环
    if (list.len() > record.len()) & (times > count) {
        let mut num; // 定义临时数字
        for _i in 1..=times {
            // 循环times次
            loop {
                if (times <= count) | (list.len() <= record.len()) {
                    break;
                }
                num = rand::thread_rng().gen_range(1..=list.len()) as u32; // 获取随机数

                // 判断num是否在record中，以及record的长度是否超出list的长度
                if (list.len() > record.len()) & record.contains(&num) {
                    continue;
                } else {
                    // push num into record
                    record.push(num);
                    // counter
                    count += 1;
                    break;
                }
            }
        }
    } else if list.len() == record.len() {
        // 返回抽取完毕消息
        return Err(String::from("列表抽取完毕"));
    }
    // 输出随机结果
    let mut result = String::new(); // result 输出Strings
    let lenth = record.len(); // 获取record长度
    for i in 0..lenth {
        let value = match list.get(match record.get(i) {
            Some(e) => e,
            None => {
                println!("无法获取抽取记录vec");
                process::exit(1);
            }
        }) {
            Some(s) => s,
            None => {
                println!("element con't find in list");
                process::exit(1)
            }
        };
        if i >= 1 {
            result.push_str("，")
        }
        result.push_str(value);
    }

    let _ = app_handle.emit_all(
        // 返回大标题结果
        "titleoutput",
        match list.get(match record.last() {
            Some(e) => e,
            None => {
                println!("无法获取抽取记录vec");
                process::exit(1);
            }
        }) {
            Some(s) => String::from(s),
            None => {
                println!("element con't find in list");
                process::exit(1)
            }
        },
    );
    let _ = app_handle.emit_all("listoutput", result); // 返回下方小字结果
    Ok(())
}

// 重置
#[tauri::command]
fn reset() {
    let mut record = RECORD.lock().unwrap();
    let lenth = record.len();
    for _i in 0..lenth {
        record.swap_remove(0);
    }
}

#[tauri::command]
fn return_list_number() -> u32 {
    let list = LIST.lock().unwrap();
    list.len() as u32
}

// splashscreen
#[tauri::command]
fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_randnum,
            reset,
            return_list_number,
            close_splashscreen,
            init_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

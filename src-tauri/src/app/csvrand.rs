use rand::prelude::*;
use randapp::{conf::AppConf, exists};
use std::collections::HashMap;
use std::process;
use std::sync::Mutex;
use tauri::{Manager, command};
use crate::app::readers::csvreader;

// 初始化全局变量
lazy_static! {
    // define global list object
    static ref LIST: Mutex<HashMap<u32, String>> = Mutex::new({
        println!("初始化全局list变量");
        let config = AppConf::read();
        let csv = csvreader::CSV::new().read(config.csv_path).unwrap();
        csv.content
    });
    // define global record object
    static ref RECORD: Mutex<Vec<u32>> = Mutex::new(vec![]);
}

#[command]
pub fn init_list() -> Result<(), String> {
    if let false = exists(AppConf::read().csv_path) {
        return Err(String::from("CSV 路径无效"));
    }
    Ok(())
}

#[command]
/// 生成随机数
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

/// 重置
#[command]
fn reset() {
    let mut record = RECORD.lock().unwrap();
    let lenth = record.len();
    for _i in 0..lenth {
        record.swap_remove(0);
    }
}

#[command]
fn return_list_number() -> u32 {
    let list = LIST.lock().unwrap();
    list.len() as u32
}



#[command]
fn return_csv_path() -> Result<String, String> {
    match appconfig::CONF::new().build() {
        Ok(c) => Ok(c.csv_path),
        Err(e) => {
            println!("init_error:{}", e);
            Err(String::from("CSV 路径读取失败"))
        }
    }
}

#[command]
fn reload_csv_path() -> Result<String, String> {
    match new_csvpath() {
        Ok(path) => {
            let mut list = LIST.lock().unwrap();
            *list = {
                println!("初始化全局list变量");
                let config = appconfig::CONF::new().build().unwrap_or_else(|err| {
                    println!("err create global list object: {err}");
                    process::exit(1)
                });
                let csv = csvreader::CSV::new().read(config.csv_path).unwrap();
                csv.list
            };
            Ok(path)
        }
        Err(e) => {
            println!("err:{}", e);
            Err(String::from("重新设置CSV文件路径失败"))
        }
    }
}
use log::{debug, error, info};
use rand::prelude::*;
use rtools::{conf::AppConf, exists};
use std::collections::HashMap;
use std::process;
use std::sync::Mutex;
use tauri::{Manager, command};
use crate::app::readers::csvreader;

// 初始化全局变量
lazy_static! {
    // define global list object
    static ref LIST: Mutex<HashMap<u32, String>> = Mutex::new({
        debug!("初始化全局list变量");
        let config = AppConf::read();
        let csv = csvreader::CSV::new().read(config.csv_path).unwrap();
        csv.content
    });
    // define global record object
    static ref RECORD: Mutex<Vec<u32>> = Mutex::new(vec![]);
}

#[command]
pub fn init_list() -> Result<(), &'static str> {
    if let false = exists(AppConf::read().csv_path) {
        return Err("CSV 路径无效");
    }
    Ok(())
}

#[command]
/// 生成随机数
pub fn generate_randnum(times: u32, app_handle: tauri::AppHandle) -> Result<(), &'static str> {
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
        debug!("列表抽取完毕");
        return Err("列表抽取完毕");
    }
    // 输出随机结果
    let mut result = String::new(); // result 输出Strings
    let lenth = record.len(); // 获取record长度
    for i in 0..lenth {
        let value = match list.get(match record.get(i) {
            Some(e) => e,
            None => {
                error!("无法获取抽取记录vec");
                return Err("无法获取抽取记录vec")
            }
        }) {
            Some(s) => s,
            None => {
                error!("element con't find in list");
                return Err("element con't find in list")
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
                error!("无法获取抽取记录vec");
                process::exit(1);
            }
        }) {
            Some(s) => String::from(s),
            None => {
                error!("element con't find in list");
                process::exit(1)
            }
        },
    );
    info!("当前抽取结果: {result}");
    let _ = app_handle.emit_all("listoutput", &result); // 返回下方小字结果
    Ok(())
}

/// 重置计时器
#[command]
pub fn reset() {
    let mut record = RECORD.lock().unwrap();
    let lenth = record.len();
    for _i in 0..lenth {
        record.swap_remove(0);
    }
    debug!("reset CSVRand")
}

#[command]
pub fn return_list_number() -> u32 {
    let list = LIST.lock().unwrap();
    list.len() as u32
}

#[command]
pub fn return_randresult(app_handle: tauri::AppHandle) {
    let list = LIST.lock().unwrap();
    let num = rand::thread_rng().gen_range(1..=list.len()) as u32;

    let _ = app_handle.emit_all(
        // 返回大标题结果
        "titleoutput",
        match list.get(&num) {
            Some(s) => String::from(s),
            None => {
                error!("element con't find in list");
                process::exit(1)
            }
        },
    );
}

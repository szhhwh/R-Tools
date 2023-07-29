use crate::app::readers::calareader;
use log::{debug, error, warn};
use rand::prelude::*;
use rtools::error::AppError;
use rtools::{conf::AppConf, exists};
use std::sync::Mutex;
use std::{collections::HashMap, path::PathBuf};
use tauri::{command, Manager};

// 初始化全局变量
lazy_static! {
    // define global list object
    static ref LIST: Mutex<HashMap<usize, String>> = Mutex::new({
        debug!("初始化全局list变量");
        let config = AppConf::read();
        let xlsx = calareader::CALA::new().read(config.cala_path).unwrap_or_else(|err|{
            println!("{err}");
            calareader::CALA { file_path: PathBuf::new(), content: HashMap::new() }
        });
        xlsx.content
    });
    // define global record object
    static ref RECORD: Mutex<Vec<usize>> = Mutex::new(vec![]);
}

#[command]
pub fn init_list() -> Result<(), &'static str> {
    if let false = exists(AppConf::read().cala_path) {
        return Err("excel 文件路径无效");
    }
    Ok(())
}

fn rand(times: u32) -> Result<(), AppError> {
    let config = AppConf::read();
    let mut record = RECORD.lock().unwrap();
    let list = LIST.lock().unwrap();

    if config.antiduble == true {
        // counter 计数器
        let mut count: u32 = 0;
        // 判断record是否为空，空数组则添加一个随机数
        if record.is_empty() {
            let num = rand::thread_rng().gen_range(0..list.len());
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
                    num = rand::thread_rng().gen_range(0..list.len()); // 获取随机数

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
            return Err(AppError::Err("列表抽取完毕".into()));
        }
    } else if config.antiduble == false {
        for _i in 1..=times {
            record.push(rand::thread_rng().gen_range(0..list.len()));
        }
    }
    Ok(())
}

#[command]
/// 生成随机数
pub fn generate_randnum(times: u32, app_handle: tauri::AppHandle) -> Result<(), String> {
    match rand(times) {
        Ok(_) => (),
        Err(e) => {
            warn!("{e}");
            if let AppError::Err(v) = e {
                return Err(v);
            }
        }
    }
    match titleoutput(&app_handle) {
        Ok(_) => (),
        Err(e) => {
            error!("{e}");
            if let AppError::Err(v) = e {
                return Err(v);
            }
        }
    }
    match listoutput(&app_handle) {
        Ok(_) => (),
        Err(e) => {
            error!("{e}");
            if let AppError::Err(v) = e {
                return Err(v);
            }
        }
    }
    Ok(())
}

/// 返回抽取列表
fn listoutput(app_handle: &tauri::AppHandle) -> Result<(), AppError> {
    let record = RECORD.lock().unwrap();
    let list = LIST.lock().unwrap();
    let mut result = String::new(); // result 输出Strings
    let lenth = record.len(); // 获取record长度
    for i in 0..lenth {
        let value = match list.get(match record.get(i) {
            Some(e) => e,
            None => {
                error!("无法获取抽取记录vec");
                return Err(AppError::Err("无法获取抽取记录vec".into()));
            }
        }) {
            Some(s) => s,
            None => {
                error!("element({}) con't find in list", i);
                return Err(AppError::Err("element con't find in list".into()));
            }
        };
        if i >= 1 {
            result.push_str(", ")
        }
        result.push_str(value);
    }
    debug!("Current Result: {result}");
    let _ = app_handle.emit_all("listoutput", &result); // 返回下方小字结果
    Ok(())
}

/// 返回大标题结果
fn titleoutput(app_handle: &tauri::AppHandle) -> Result<(), AppError> {
    let record = RECORD.lock().unwrap();
    let list = LIST.lock().unwrap();
    let _ = app_handle.emit_all(
        "titleoutput",
        match list.get(match record.last() {
            Some(e) => e,
            None => {
                error!("无法获取抽取记录vec");
                return Err(AppError::Err("无法获取抽取记录vec".into()));
            }
        }) {
            Some(s) => String::from(s),
            None => {
                error!("element con't find in list");
                return Err(AppError::Err("element con't find in list".into()));
            }
        },
    );
    Ok(())
}

/// 重置计数器
#[command]
pub fn reset() {
    let mut record = RECORD.lock().unwrap();
    let lenth = record.len();
    for _i in 0..lenth {
        record.swap_remove(0);
    }
    debug!("reset calarand")
}

#[command]
pub fn return_list_number() -> u32 {
    let list = LIST.lock().unwrap();
    list.len() as u32
}

#[command]
pub fn return_randresult(app_handle: tauri::AppHandle) -> Result<(), &'static str> {
    let list = LIST.lock().unwrap();
    let num = rand::thread_rng().gen_range(0..list.len());

    let _ = app_handle.emit_all(
        // 返回大标题结果
        "titleoutput",
        match list.get(&num) {
            Some(s) => String::from(s),
            None => {
                error!("element con't find in list");
                return Err("element con't find in list");
            }
        },
    );
    Ok(())
}

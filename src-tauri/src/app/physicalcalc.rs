use rtools::error::AppError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpDataItem {
    id: usize,
    value: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeResult {
    mean: f32,
    count: usize,
    std_dev: f32,
    ua: f32,
    ub: f32,
    uc: f32,
    std_error: f32,
}

impl ComputeResult {
    pub fn new(
        mean: f32,
        count: usize,
        std_dev: f32,
        ua: f32,
        ub: f32,
        uc: f32,
        std_error: f32,
    ) -> ComputeResult {
        ComputeResult {
            mean,
            count,
            std_dev,
            ua,
            ub,
            uc,
            std_error,
        }
    }

    pub fn round(mut self) -> Self {
        self.mean = Self::round_half_to_even(self.mean, 1);
        self.std_dev = Self::round_half_to_even(self.std_dev, 1);
        self.ua = Self::round_half_to_even(self.ua, 1);
        self.ub = Self::round_half_to_even(self.ub, 1);
        self.uc = Self::round_half_to_even(self.uc, 1);
        self.std_error = Self::round_half_to_even(self.std_error, 1);
        self
    }

    fn round_half_to_even_int(x: f32) -> f32 {
        let floor_val = x.floor();
        let frac = x - floor_val;

        if frac > 0.5 {
            floor_val + 1.0
        } else if frac < 0.5 {
            floor_val
        } else {
            // frac == 0.5 的情况
            let int_floor = floor_val as i64;
            if int_floor % 2 == 0 {
                floor_val
            } else {
                floor_val + 1.0
            }
        }
    }

    fn round_half_to_even(x: f32, digits: u32) -> f32 {
        if digits == 0 {
            return Self::round_half_to_even_int(x);
        }

        // 根据保留小数位数，生成缩放因子
        let factor = 10f32.powi(digits as i32);
        // 将数值放大
        let scaled = x * factor;

        // 先取向下取整部分
        let floor_val = scaled.floor();
        // 小数部分
        let frac = scaled - floor_val;

        // 如果小数部分大于 0.5，直接向上舍入
        if frac > 0.5 {
            (floor_val + 1.0) / factor
        }
        // 如果小数部分小于 0.5，直接舍去
        else if frac < 0.5 {
            floor_val / factor
        }
        // 如果小数部分恰好等于 0.5，执行 "偶数舍入" 判断
        else {
            // floor_val 转换为整数检查奇偶性
            let int_floor = floor_val as i64;
            if int_floor % 2 == 0 {
                // 偶数则直接舍去
                floor_val / factor
            } else {
                // 奇数则进位
                (floor_val + 1.0) / factor
            }
        }
    }
}

/// 平均数计算
fn mean(data: &Vec<f32>) -> Option<f32> {
    let sum = data.iter().sum::<f32>();
    let count = data.len();

    match count {
        0 => None,
        _ => Some(sum / count as f32),
    }
}

/// 标准误差计算
fn std_deviation(data: &Vec<f32>) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - value;
                    diff * diff
                })
                .sum::<f32>()
                / count as f32;

            Some(variance.sqrt())
        }
        _ => None,
    }
}

// 第一类不确定度计算
fn ua(std_dev: f32, count: usize) -> Option<f32> {
    match std_dev {
        0.0 => None,
        _ => Some((std_dev * count as f32).sqrt()),
    }
}

// 第二类不确定度计算
fn ub(instrument_err: f32) -> Option<f32> {
    match instrument_err {
        0.0 => None,
        _ => Some(((instrument_err * instrument_err) * 3.0 as f32).sqrt()),
    }
}

// 第三类不确定度计算
fn uc(ua: f32, ub: f32) -> Option<f32> {
    match (ua, ub) {
        (0.0, 0.0) => None,
        (ua, ub) => Some(((ua * ua + ub * ub) as f32).sqrt()),
    }
}

// 数据检查并删除异常值
fn data_check(data: &mut Vec<f32>, mean: f32, std_dev: f32) -> Result<bool, AppError> {
    if data.is_empty() {
        return Err(AppError::Err("No valid numeric data provided"));
    }

    let mut removed = false;
    let mut i = 0;

    // 循环检查并移除超出3倍标准差的数据
    while i < data.len() {
        if (data[i] - mean).abs() > 3.0 * std_dev {
            data.remove(i);
            removed = true;
        } else {
            i += 1;
        }
    }

    if data.is_empty() {
        return Err(AppError::Err("All data removed due to exceeding threshold"));
    }

    Ok(removed)
}

fn parse_data(data: &Vec<ExpDataItem>) -> Vec<f32> {
    data.iter()
        .filter_map(|item| item.value.parse::<f32>().ok())
        .collect()
}

fn std_error(uc: f32, mean: f32) -> Option<f32> {
    match uc {
        0.0 => None,
        _ => Some(uc / mean),
    }
}

#[tauri::command]
pub fn compute(data: Vec<ExpDataItem>, instrument_err: f32) -> Result<ComputeResult, &'static str> {
    let mut values = parse_data(&data);

    if values.is_empty() {
        return Err("No valid numeric data provided");
    }

    let mut mean_value;
    let mut stddev_value;
    let mut data_changed = true;

    // 循环计算直到没有异常值
    while data_changed {
        mean_value = mean(&values).ok_or("Failed to calculate mean")?;
        stddev_value = std_deviation(&values).ok_or("Failed to calculate standard deviation")?;
        match data_check(&mut values, mean_value, stddev_value) {
            Ok(removed) => {
                data_changed = removed; // 如果有数据被删除，继续循环
            }
            Err(_) => {
                return Err("Error checking data for outliers");
            }
        }
    }

    let mean_value = mean(&values).ok_or("Failed to calculate mean")?;
    let stddev_value = std_deviation(&values).ok_or("Failed to calculate standard deviation")?;
    let ua_value =
        ua(stddev_value, values.len()).ok_or("Failed to calculate first class uncertainty")?;
    let ub_value = ub(instrument_err).ok_or("Failed to calculate second class uncertainty")?;
    let uc_value = uc(ua_value, ub_value).ok_or("Failed to calculate combined uncertainty")?;
    let stderror_value =
        std_error(uc_value, mean_value).ok_or("Failed to calculate standard error")?;

    let compute_result = ComputeResult::new(
        mean_value,
        values.len(),
        stddev_value,
        ua_value,
        ub_value,
        uc_value,
        stderror_value,
    ).round();

    Ok(compute_result)
}

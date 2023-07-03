use std::collections::HashMap;
use std::fs;
use std::path::Path;
use rtools::error::AppError;

#[derive(Default)]
pub struct CSV<P: AsRef<Path>> {
    pub file_path: P,
    pub content: HashMap<u32, String>,
}

impl<P: AsRef<Path> + Default> CSV<P> {
    pub fn new() -> Self {
        Default::default()
    }

    /// 读取CSV文件内容
    pub fn read(&mut self, path: P) -> Result<Self, AppError> {
        // 从传入的路径中读取csv文件正文
        let binding = fs::read_to_string(&path)?;
        let content = binding.as_bytes();
        // 解析csv
        let mut reader = csv::Reader::from_reader(content);
        let mut list: HashMap<u32, String> = HashMap::new();
        for record in reader.records() {
            let record = record?;
            list.insert(
                match record[0].parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                },
                match record[1].parse() {
                    Ok(name) => name,
                    Err(_) => continue,
                },
            );
        }
        Ok(CSV {
            file_path: path,
            content: list,
        })
    }
}

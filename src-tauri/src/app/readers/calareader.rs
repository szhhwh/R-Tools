use calamine::{open_workbook,  Reader, Xlsx};
use rtools::error::AppError;
use std::collections::HashMap;
use std::path::Path;

#[derive(Default)]
pub struct CALA<P: AsRef<Path>> {
    pub file_path: P,
    pub content: HashMap<usize, String>,
}

impl<P: AsRef<Path> + Default> CALA<P> {
    pub fn new() -> Self {
        Default::default()
    }

    /// 读取excel文件内容
    pub fn read(&mut self, path: P) -> Result<Self, AppError> {
        let mut workbook: Xlsx<_> = open_workbook(&path)?;
        let mut list: HashMap<usize, String> = HashMap::new();
        if let Some(Ok(range)) = workbook.worksheet_range("Sheet1") {
            let cell = range.cells();
            for i in cell {
                list.insert(i.0, i.2.to_string());
            }
        }
        Ok(CALA { file_path: path, content: list })
    }
}

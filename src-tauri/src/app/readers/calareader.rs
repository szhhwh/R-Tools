use calamine::{open_workbook, Reader, Xls, Xlsx};
use log::debug;
use rtools::conf;
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
        let extension = match path.as_ref().extension() {
            Some(v) => v,
            None => return Err(AppError::Err("excel path parse err".into())),
        };
        let mut list: HashMap<usize, String> = HashMap::new();
        let sheetname = conf::AppConf::read().lastsheet;
        if (extension == "xlsx") | (extension == "xlsm") | (extension == "xlam") {
            let mut workbook: Xlsx<_> = open_workbook(&path)?;
            let for_tablename: Xlsx<_> = open_workbook(&path)?;
            let tablename = for_tablename.sheet_names();
            if tablename.contains(&sheetname) {
                if let Some(Ok(range)) = workbook.worksheet_range(&sheetname) {
                    let cell = range.cells();
                    for i in cell {
                        list.insert(i.0, i.2.to_string());
                    }
                }
            } else {
                if let Some(Ok(range)) = workbook.worksheet_range(tablename[0].as_str()) {
                    let cell = range.cells();
                    for i in cell {
                        list.insert(i.0, i.2.to_string());
                    }
                }
            }
        } else if extension == "xls" {
            let mut workbook: Xls<_> = open_workbook(&path)?;
            let for_tablename: Xls<_> = open_workbook(&path)?;
            let tablename = for_tablename.sheet_names();
            if let Some(Ok(range)) = workbook.worksheet_range(tablename[0].as_str()) {
                let cell = range.cells();
                for i in cell {
                    list.insert(i.0, i.2.to_string());
                }
            }
        }
        Ok(CALA {
            file_path: path,
            content: list,
        })
    }

    /// 返回excel workbook中的sheet名称
    pub fn sheet_names(path: P) -> Result<serde_json::Value, AppError> {
        let extension = match path.as_ref().extension() {
            Some(v) => v,
            None => return Err(AppError::Err("excel path parse err".into())),
        };
        let mut names = serde_json::Value::Null;
        if (extension == "xlsx") | (extension == "xlsm") | (extension == "xlam") {
            let workbook: Xlsx<_> = open_workbook(&path)?;
            let tablename = workbook.sheet_names();
            debug!(
                "Sheets' name: {}",
                serde_json::to_value(&tablename).unwrap()
            );
            names = serde_json::to_value(&tablename).unwrap();
        } else if extension == "xls" {
            let workbook: Xls<_> = open_workbook(&path)?;
            let tablename = workbook.sheet_names();
            debug!("Sheets' name: {}", serde_json::to_value(tablename).unwrap());
            names = serde_json::to_value(&tablename).unwrap();
        }
        Ok(names)
    }
}

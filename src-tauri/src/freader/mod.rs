pub mod csvreader {
    use std::collections::HashMap;
    use std::error::Error;
    use std::fs;

    #[derive(Default)]
    pub struct CSV {
        pub path: String,
        pub list: HashMap<u32, String>,
    }

    impl CSV {
        pub fn new() -> Self {
            Default::default()
        }
        pub fn read(&mut self, path: String) -> Result<Self, Box<dyn Error>> {
            // 从传入的路径中读取csv文件正文
            let binding = fs::read_to_string(&path).expect("error read csvfile");
            let content = binding.as_bytes();
            // 使用csv解析器解析csv
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
            Ok(CSV { path, list })
        }
    }
}

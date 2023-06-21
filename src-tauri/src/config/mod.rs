pub mod g_config {
    use ini::Ini;
    use rfd::FileDialog;
    use std::error::Error;
    use std::io;
    use std::process;

    const CONF_NAME: &str = "conf.ini";
    const CONF_CSV_PATH_KEY: &str = "csv_path";
    const CONF_PATH_SECTION: &str = "PATH";
    
    #[derive(Default)]
    pub struct CONF {
        pub csv_path: String,
    }

    impl CONF {
        pub fn new() -> Self {
            Default::default()
        }

        pub fn build(&self) -> Result<CONF, &'static str> {
            let mut conf_init = false;
            let mut count: i8 = 0;
            let mut config: Option<Ini>;

            loop {
                // 尝试从默认文件中读取配置
                config = match Ini::load_from_file(CONF_NAME) {
                    Ok(config) => {
                        conf_init = true;
                        Some(config)
                    }
                    Err(_) => {
                        // 无法读取配置后尝试新建
                        println!("No conf file\nTry to create conf file");
                        self.new_conf().unwrap();
                        None
                    }
                };
                count += 1;
                if conf_init | (count > 2) {
                    break;
                }
            }

            let config = match config {
                // 解出ini对象
                Some(n) => n,
                None => {
                    return Err("Problem resloving conf file");
                }
            };

            // 从配置文件中读取csv路径
            let csv_path = match config.get_from(Some("PATH"), CONF_CSV_PATH_KEY) {
                Some(n) => String::from(n),
                None => {
                    return Err("Error get csvpath from ini file");
                }
            };

            Ok(CONF { csv_path })
        }

        fn new_conf(&self) -> Result<String, Box<dyn Error>> {
            // 调用rfd库获取csv路径
            let csv_path = match FileDialog::new()
                .add_filter("csv file", &["csv"])
                .set_directory("/")
                .pick_file()
            {
                Some(p) => p,
                None => {
                    println!("Error get path");
                    process::exit(1)
                }
            }
            .as_path()
            .display()
            .to_string();

            // 根据路径创建新的conf文件
            let mut conf = Ini::new();
            conf.with_section(Some(CONF_PATH_SECTION))
                .set(CONF_CSV_PATH_KEY, &csv_path);
            conf.write_to(&mut io::stdout()).unwrap();
            conf.write_to_file(CONF_NAME).unwrap();

            Ok(csv_path.to_string())
        }
    }
}

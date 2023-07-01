pub mod appconfig {
    use ini::Ini;
    use rfd::FileDialog;
    use std::error::Error;
    use std::io;

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

        pub fn build(&self) -> Result<CONF, Box<dyn Error>> {
            // 尝试从默认文件中读取配置
            let config = match Ini::load_from_file(CONF_NAME) {
                Ok(config) => config,
                Err(_) => {
                    // 无法读取配置后尝试新建
                    println!("Can't find config file\nTry to create config file");
                    new_csvpath()?;
                    Ini::load_from_file(CONF_NAME)?
                }
            };

            // 从配置文件中读取csv路径
            let csv_path = match config.get_from(Some("PATH"), CONF_CSV_PATH_KEY) {
                Some(n) => String::from(n),
                None => {
                    return Err("Error get csvpath from ini file".into());
                }
            };

            Ok(CONF { csv_path })
        }

        pub fn reload_csvpath(&self) {
            let _ = new_csvpath();
        }
    }

    /// 调取rfd获取选择的csv文件路径并返回该路径
    pub fn new_csvpath() -> Result<String, &'static str> {
        // 调用rfd库获取csv路径
        let csv_path = match FileDialog::new()
            .add_filter("csv file", &["csv"])
            .set_directory("/")
            .pick_file()
        {
            Some(p) => p,
            None => {
                println!("Problem get path");
                return Err("路径为空或未选取文件");
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

        // 获取用户config文件夹
        let user_config_path = match tauri::api::path::config_dir() {
            None => {
                println!("Problem get user's config_path");
                return Err("13");
            }
            Some(path) => path,
        }
        .join("randapp")
        .join("config.ini")
        .as_path()
        .display()
        .to_string();
        dbg!(&user_config_path);
        conf.write_to_file(&user_config_path).unwrap();

        Ok(csv_path.to_string())
    }
}

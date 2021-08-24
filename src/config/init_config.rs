use toml;
use std::fs::File;
use std::io::Read;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct InitConfig {
    pub global: GlobalConfig,
    pub db: DBConfig,
}

#[derive(Deserialize)]
pub struct GlobalConfig {
    pub host: String,
    pub debug: bool,
}

#[derive(Deserialize)]
pub struct DBConfig {
    pub mysql_url: String,
    pub logic_column: String,
    pub logic_un_deleted: i64,
    pub logic_deleted: i64,
}

impl Default for InitConfig {
    fn default() -> Self {
        let file_path = "res/init.toml";
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such file: {}, error: {}", file_path, e)
        };

        let mut file_content = String::new();
        if let Err(e) = file.read_to_string(&mut file_content) {
            panic!("read file context error! file: {} error: {}", file_path, e);
        }

        let init_config = match toml::from_str(&file_content) {
            Ok(c) => c,
            Err(e) => panic!("load init config to cache error, file: {}, error: {}", file_path, e)
        };

        init_config
    }
}
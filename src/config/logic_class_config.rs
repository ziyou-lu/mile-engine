use std::fs::File;
use std::io::Read;
use toml;
use serde_derive::Deserialize;
use anyhow::{Result, Error};
use crate::obj::schema::data_schema::{BoxSchema, GameObjectSchema};

#[derive(Deserialize)]
pub(crate) struct LogicClassConfig {
    pub objects : Vec<LogicClass>,
    pub containers: Vec<LogicClass>,
}

#[derive(Deserialize)]
pub(crate) struct LogicClass {
    pub name: String,
    pub base: String,
    pub design: String,
}

impl Default for LogicClassConfig {
    fn default() -> Self {
        let class_config_file_path = "res/logic/class.toml";

        let mut file = match File::open(class_config_file_path) {
            Ok(f) => f,
            Err(e) => {
                panic!("open file logic class config error! file: {} error: {}", class_config_file_path, e)
            }
        };

        let mut class_config_file_str = String::new();
        if let Ok(e) = file.read_to_string(&mut class_config_file_str) {
            panic!("read file logic class config error! file: {} error: {}", class_config_file_path, e)
        }

        let logic_class_config = match toml::from_str(&class_config_file_str) {
            Ok(config) => config,
            Err(e) => {
                panic!("format  logic class config error! file: {} error: {}", class_config_file_path, e)
            }
        };

        logic_class_config
    }
}
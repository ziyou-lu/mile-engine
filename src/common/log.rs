use std::fs;
use crate::config::init_config::LogConfig;
use std::collections::HashMap;
use crate::common::log::LogType::{LogTypeFile, LogTypeConsole};
use time;

pub struct Log {}

static DEBUG: &'static str = "debug";
static INFO: &'static str = "info";
static WARN: &'static str = "warn";
static ERROR: &'static str = "error";
static DEBUG_LEVEL: i8 = 1;
static INFO_LEVEL: i8 = 2;
static WARN_LEVEL: i8 = 3;
static ERROR_LEVEL: i8 = 4;
lazy_static! {
    pub static ref LEVEL_MAP: HashMap<&'static str, i8> = [
        (DEBUG, DEBUG_LEVEL),
        (DEBUG, INFO_LEVEL),
        (DEBUG, WARN_LEVEL),
        (DEBUG, ERROR_LEVEL)
    ].into();
}
enum LogType<'a> {
    LogTypeConsole(&'a str),
    LogTypeFile(&'a str)
}

impl Log {
    pub fn debug(content: &str) {
        if should_output(LogTypeFile(DEBUG)){
            save_to_log_file(DEBUG, content);
        }

        if should_output(LogTypeConsole(DEBUG)) {
            output_to_console(DEBUG, content);
        }
    }

    pub fn info(content: &str) {
        if should_output(LogTypeFile(INFO)){
            save_to_log_file(INFO, content);
        }

        if should_output(LogTypeConsole(INFO)) {
            output_to_console(INFO, content);
        }
    }

    pub fn warn(content: &str) {
        if should_output(LogTypeFile(WARN)){
            save_to_log_file(WARN, content);
        }

        if should_output(LogTypeConsole(WARN)) {
            output_to_console(WARN, content);
        }
    }

    pub fn error(content: &str) {
        if should_output(LogTypeFile(ERROR)){
            save_to_log_file(ERROR, content);
        }

        if should_output(LogTypeConsole(ERROR)) {
            output_to_console(ERROR, content);
        }
    }
}

fn should_output(log_type: LogType) -> bool {
    let (level, config_log_level) = match log_type {
        LogTypeConsole(level) => (level, crate::CONTEXT.init_config.log.console_level.as_str()),
        LogTypeFile(level) => (level, crate::CONTEXT.init_config.log.file_level.as_str())
    };
    let config_log_level = crate::CONTEXT.init_config.log.console_level.as_str();
    if (!LEVEL_MAP.contains_key(config_log_level) || !crate::CONTEXT.init_config.global.debug)
        && LEVEL_MAP.get(level).unwrap() < &WARN_LEVEL {
        return false;
    }

    if LEVEL_MAP.get(config_log_level).unwrap() > LEVEL_MAP.get(level).unwrap() {
        return false;
    }

    return true;
}

fn save_to_log_file(level: &str, content: &str) {
    if !fs::try_exists("log").unwrap() {
        fs::DirBuilder::new().create("log");
    }
}

fn output_to_console(level: &str, content: &str) {
    println!("[{}] {:?} {}", level, time::Instant::now(), content);
}
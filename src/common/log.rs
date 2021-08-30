use std::fs;
use std::collections::HashMap;
use crate::common::log::LogType::{LogTypeFile, LogTypeConsole};
use std::io::{Read, Write};
use std::ops::{Add};
use uuid;
use std::str::FromStr;
use std::os::unix::fs::MetadataExt;

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
        (INFO, INFO_LEVEL),
        (WARN, WARN_LEVEL),
        (ERROR, ERROR_LEVEL)
    ].into();
}
enum LogType<'a> {
    LogTypeConsole(&'a str),
    LogTypeFile(&'a str)
}

impl Log {
    pub fn debug(&self, content: String) {
        if should_output(LogTypeFile(DEBUG)){
            if let Err(e) = save_to_log_file(DEBUG, content.as_str()) {
                output_to_console(ERROR , format!("[engine] {:?}", e).as_str());
            }
        }

        if should_output(LogTypeConsole(DEBUG)) {
            output_to_console(DEBUG, content.as_str());
        }
    }

    pub fn info(&self, content: String) {
        if should_output(LogTypeFile(INFO)){
            if let Err(e) = save_to_log_file(INFO, content.as_str()) {
                output_to_console(INFO , format!("[engine] {:?}", e).as_str());
            }
        }

        if should_output(LogTypeConsole(INFO)) {
            output_to_console(INFO, content.as_str());
        }
    }

    pub fn warn(&self, content: String) {
        if should_output(LogTypeFile(WARN)){
            if let Err(e) = save_to_log_file(WARN, content.as_str()) {
                output_to_console(WARN , format!("[engine] {:?}", e).as_str());
            }
        }

        if should_output(LogTypeConsole(WARN)) {
            output_to_console(WARN, content.as_str());
        }
    }

    pub fn error(&self, content: String) {
        if should_output(LogTypeFile(ERROR)){
            if let Err(e) = save_to_log_file(ERROR, content.as_str()) {
                output_to_console(ERROR , format!("[engine] {:?}", e).as_str());
            }
        }

        if should_output(LogTypeConsole(ERROR)) {
            output_to_console(ERROR, content.as_str());
        }
    }

    pub fn panic(&self, content: String) -> !{
        if should_output(LogTypeFile(ERROR)){
            if let Err(e) = save_to_log_file(ERROR, content.as_str()) {
                output_to_console(ERROR , format!("[engine] {:?}", e).as_str());
            }
        }

        panic!(content);
    }
}

fn should_output(log_type: LogType) -> bool {
    let (level, config_log_level) = match log_type {
        LogTypeConsole(level) => (level, crate::MILE.get_context().init_config.log.console_level.as_str()),
        LogTypeFile(level) => (level, crate::MILE.get_context().init_config.log.file_level.as_str())
    };

    if (!LEVEL_MAP.contains_key(config_log_level) || !crate::MILE.get_context().init_config.global.debug)
        && LEVEL_MAP.get(level).unwrap() < &WARN_LEVEL {
        return false;
    }

    if LEVEL_MAP.get(config_log_level).unwrap() > LEVEL_MAP.get(level).unwrap() {
        return false;
    }

    return true;
}

fn save_to_log_file(level: &str, content: &str) -> Result<(), std::io::Error>{
    if let Err(_) = fs::read_dir("log") {
        fs::DirBuilder::new().create("log")?;
    }

    let file_split_day = crate::MILE.get_context().init_config.log.file_split_time;
    let server_name = crate::MILE.get_context().init_config.global.server_name.clone();

    let log_content = format!("[{}] {:?} {} \n", level, chrono::Local::now(), content);

    let mut record_str = String::new();
    let create_new_file = match fs::File::open("log/.log") {
        Ok(mut record_file) => {
            record_file.read_to_string(&mut record_str)?;

            let record_date = chrono::DateTime::<chrono::Local>::from_str(record_str.as_str()).unwrap_or_else(|e| {
                output_to_console(ERROR, format!("{:?}", e.to_string()).as_str());
                chrono::Local::now()
            });
            record_date.add(chrono::Duration::days(file_split_day as i64)).lt(&chrono::Local::now())
        }
        Err(_) => {
            fs::File::create("log/.log")?;
            true
        }
    };

    if create_new_file {
        if let Ok(_) = fs::File::open("log/latest.log") {
            fs::rename("log/latest.log",
                       format!("log/{}-{}-{}.log",
                               server_name,
                               record_str,
                               uuid::Uuid::new_v4().to_string()
                       ))?;
        }

        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("log/latest.log")?;
        file.write_all(log_content.as_bytes())?;
        file.flush()?;

        let mut record_file = fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open("log/.log")?;
        record_file.write_all(chrono::Local::today().and_hms(0, 0, 0).to_string().as_bytes())?;
        record_file.flush()?;
    } else {
        if let Ok(_) = fs::read("log/latest.log") {
            if fs::metadata("log/latest.log").unwrap().size() / 1024 > crate::MILE.get_context().init_config.log.file_max_size {
                fs::rename("log/latest.log",
                           format!("log/{}-{}-{}.log",
                                   server_name,
                                   record_str,
                                   uuid::Uuid::new_v4().to_string()
                           ))?;
            }
        }

        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("log/latest.log")?;
        file.write_all(log_content.as_bytes())?;
        file.flush()?;
    }

    Ok(())
}

fn output_to_console(level: &str, content: &str) {
    println!("[{}] {:?} {}", level, chrono::Local::now(), content);
}
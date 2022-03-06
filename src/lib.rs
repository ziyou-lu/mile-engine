#[macro_use]
extern crate lazy_static;

pub mod db;
mod cache;
mod context;
mod config;
mod common;
mod logic;
mod obj;
pub mod mile;

use config::init_config::InitConfig;
use mile::{MILE, Mile};




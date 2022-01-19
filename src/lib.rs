#[macro_use]
extern crate lazy_static;

pub mod db;
mod cache;
mod context;
mod config;
mod common;
mod logic;
mod obj;
mod obj_mgr;
pub mod mile;
mod bin;

use config::init_config::InitConfig;

use std::ops::DerefMut;

use crate::common::error::Error;
use crate::context::Context;
use tokio::net::TcpListener;



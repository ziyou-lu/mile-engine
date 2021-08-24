#[macro_use]
extern crate lazy_static;

pub mod db;
mod cache;
mod rpc;
mod context;
mod config;



use config::init_config::InitConfig;

fn main() {
    println!("Hello, world!");
}

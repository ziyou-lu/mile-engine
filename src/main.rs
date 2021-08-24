#[macro_use]
extern crate lazy_static;

pub mod db;
mod cache;
mod rpc;
mod context;
mod config;

use context::CONTEXT;

use config::init_config::InitConfig;
use db::mapper::*;
use rbatis::crud::{CRUD, Skip};

#[tokio::main]
async fn main() {
        let user = User {
                user_name: Some(String::from("ziyou")),
                password: Some(String::from("123"))
        };
        CONTEXT.rbatis.save::<User>(&user, &[]);
}

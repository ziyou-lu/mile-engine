#![feature(path_try_exists)]
#[macro_use]
extern crate lazy_static;

pub mod db;
mod cache;
mod rpc;
mod context;
mod config;
mod common;

pub use context::CONTEXT;

use config::init_config::InitConfig;
use db::mapper::*;
use rbatis::crud::{CRUD};

#[tokio::main]
async fn main() {
        let user = User {
                user_name: String::from("ziyou"),
                password: String::from("123")
        };
        match CONTEXT.rbatis.save::<User>(&user, &[]).await {
                Ok(_r) => println!("玩家注册成功，用户名为 {}, 密码为{}", user.user_name, user.password),
                Err(e) => panic!("玩家注册失败, 失败原因{}", e)
        }

        let mut login_user = User {
                user_name: String::from("ziyou"),
                password: String::from("123456")
        };
        println!("玩家登录，输入的账号为 {} 密码为 {}", login_user.user_name, login_user.password);
        let user_info = match CONTEXT.rbatis.fetch_by_column::<User, String>("user_name", &login_user.user_name).await {
                Ok(r) => Some(r),
                Err(e) => {
                        println!("查询玩家用户名失败，用户名还未注册 错误 {}", e);
                        None
                }
        };

        if user_info.unwrap().password == login_user.password {
                println!("玩家 {} 登录成功, 正在获取角色信息", login_user.user_name);
        } else {
                println!("玩家 {} 登录失败, 密码错误", login_user.user_name);
        }

        login_user = User {
                user_name: String::from("ziyou"),
                password: String::from("123")
        };

        println!("玩家登录，输入的账号为 {} 密码为 {}", login_user.user_name, login_user.password);
        let user_info = match CONTEXT.rbatis.fetch_by_column::<User, String>("user_name", &login_user.user_name).await {
                Ok(r) => Some(r),
                Err(e) => {
                        println!("查询玩家用户名失败，用户名还未注册 错误 {}", e);
                        None
                }
        };

        if user_info.unwrap().password == login_user.password {
                println!("玩家 {} 登录成功, 正在获取角色信息", login_user.user_name);
        } else {
                println!("玩家 {} 登录失败, 密码错误", login_user.user_name);
        }
}

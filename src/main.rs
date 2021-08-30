#![feature(in_band_lifetimes)]
#[macro_use]
extern crate lazy_static;

pub mod db;
mod cache;
mod rpc;
mod context;
mod config;
mod common;
mod logic;

use config::init_config::InitConfig;
use logic::logic_base::LogicBase;
use crate::db::mapper::User;
use std::ops::DerefMut;
use rbatis::crud::CRUD;
use crate::common::error::Error;
use std::cell::RefCell;
use crate::context::Context;
use std::sync::{Arc, Mutex};

struct Mile {
	context: context::Context,
}

impl Default for Mile {
	fn default() -> Self {
		Mile {
			context: context::Context::default()
		}
	}
}

impl Mile {
	pub fn first<F>(&self, first_fn: F) -> Result<(), Error>
		where F: FnOnce() -> Result<(), Error>
	{
		first_fn()
	}

	pub fn start(&mut self) -> Result<(), Error> {
		self.get_context().rpc.into_inner().server = Some(match self.get_context().rpc.into_inner().server_builder.into_inner().build() {
			Ok(server) => server,
			Err(e) => MILE.get_context().log.panic(format!("start server error, {:?}", e))
		});

		self.init_all_logics()
	}

	pub fn get_context(&self) -> &Context {
		&self.context
	}

	fn init_all_logics(&self) -> Result<(), Error>{
		for logic in MILE.get_context().logics.lock().unwrap().deref_mut() {
			if let Err(e) = logic.init() {
				self.get_context().log.error(format!("init logic modules error, {}", e));
				return Error::engine(format!("init logic modules error, {}", e));
			}
		}

		Ok(())
	}
}


lazy_static! {
    pub static ref MILE: Mile = Mile::default();
}


#[tokio::main]
async fn main() {
	let user = User {
		user_name: String::from("ziyou"),
		password: String::from("123")
	};
	match MILE.get_context().rbatis.save::<User>(&user, &[]).await {
		Ok(_r) => MILE.get_context().log.info(format!("玩家注册成功，用户名为 {}, 密码为{}", user.user_name, user.password)),
		Err(e) => MILE.get_context().log.error(format!("玩家注册失败，失败原因{:?}", e))
	}

	let mut login_user = User {
		user_name: String::from("ziyou"),
		password: String::from("123456")
	};
	MILE.get_context().log.info(format!("玩家登录，输入的账号为 {} 密码为 {}", login_user.user_name, login_user.password));
	let user_info = match MILE.get_context().rbatis.fetch_by_column::<User, String>("user_name", &login_user.user_name).await {
		Ok(r) => Some(r),
		Err(e) => {
			MILE.get_context().log.info(format!("查询玩家用户名失败，用户名还未注册 错误 {}", e));
			None
		}
	};

	if user_info.unwrap().password == login_user.password {
		MILE.get_context().log.info(format!("玩家 {} 登录成功, 正在获取角色信息", login_user.user_name));
	} else {
		MILE.get_context().log.warn(format!("玩家 {} 登录失败, 密码错误", login_user.user_name));
	}

	login_user = User {
		user_name: String::from("ziyou"),
		password: String::from("123")
	};

	MILE.get_context().log.info(format!("玩家登录，输入的账号为 {} 密码为 {}", login_user.user_name, login_user.password));
	let user_info = match MILE.get_context().rbatis.fetch_by_column::<User, String>("user_name", &login_user.user_name).await {
		Ok(r) => Some(r),
		Err(e) => {
			MILE.get_context().log.info(format!("查询玩家用户名失败，用户名还未注册 错误 {}", e));
			None
		}
	};

	if user_info.unwrap().password == login_user.password {
		MILE.get_context().log.debug(format!("玩家 {} 登录成功, 正在获取角色信息", login_user.user_name));
	} else {
		MILE.get_context().log.info(format!("玩家 {} 登录失败, 密码错误", login_user.user_name));
	}
}

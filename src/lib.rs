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

use config::init_config::InitConfig;
use crate::db::mapper::User;
use std::ops::DerefMut;
use rbatis::crud::CRUD;
use crate::common::error::Error;
use crate::context::Context;
use tokio::net::TcpListener;
use obj::ObjType;
pub struct Mile {
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
	/*
		初始话
	 */
	pub fn first<F>(&self, first_fn: F) -> Result<(), Error>
		where F: FnOnce() -> Result<(), Error>
	{
		first_fn()
	}

	pub fn start(&mut self) -> Result<(), Error> {
		self.init_all_logics()
	}

	pub fn get_context(&self) -> &Context {
		&self.context
	}

	async fn init_tcp(&self) {
		let listener = TcpListener::bind(self.context.init_config.global.host.clone()).await.unwrap();

		loop{
			let (mut socket,_) = listener.accept().await.unwrap();
			tokio::spawn(async move {
				let (mut reader, mut writer)=socket.split();
				//let mut b1=Vec::new();
				// reader.read(&mut b1).await;
				// println!("接到客户端消息：{:?}",String::from_utf8_lossy(&b1));

				// writer.write_all(format!("我接到了哦，别点了，你发送的信息为{:?}", String::from_utf8_lossy(&b1)).as_bytes()).await;
			});
		}
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


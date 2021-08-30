use crate::config::init_config::InitConfig;
use grpc::{Server, ServerBuilder};
use crate::common::error::Error;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;

pub struct Rpc {
    pub server_builder: RefCell<ServerBuilder>,
    pub server: Option<Server>
}

pub fn init_rpc(config: &InitConfig) -> Result<Rpc, Error> {
    let mut server_builder = grpc::ServerBuilder::new_unix();
    if let Err(e) = server_builder.http.set_addr(format!("0.0.0.0:{}", &config.global.host)) {
        return Error::engine(format!("init rpc error! {:?}", e));
    }
    Ok(Rpc {
        server_builder: RefCell::new(server_builder),
        server: None
    })
}


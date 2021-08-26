use crate::config::init_config::InitConfig;
use grpc::{Server, ServerBuilder};

pub struct Rpc {
    pub server_builder: ServerBuilder,
    pub server: Option<Server>
}

pub fn init_rpc(config: &InitConfig) -> Rpc {
    let mut server_builder = grpc::ServerBuilder::new_plain();
    server_builder.http.set_addr(format!("0.0.0.0:{}", &config.global.host));
    Rpc {
        server_builder,
        server: None
    }
}


use crate::InitConfig;
use rbatis::rbatis::Rbatis;
use crate::common::log::Log;
use crate::rpc::Rpc;
use grpc::rt::ServerServiceDefinition;
use crate::logic::logic_base::LogicBase;
use std::sync::{Arc,Mutex};
use std::ops::{Deref, DerefMut};
use std::cell::RefCell;

pub struct Context {
    pub init_config: InitConfig,
    pub rbatis: Rbatis,
    pub log: Log,
    pub rpc: RefCell<Rpc>,

    pub(crate) logics: Arc<Mutex<Vec<Box<dyn LogicBase + Send>>>>
}

unsafe impl Sync for Context {

}

impl Default for Context {
    fn default() -> Self {
        let init_config = InitConfig::default();

        Context {
            rbatis: rbatis::core::runtime::task::block_on(async {
                crate::db::init_rbatis(&init_config).await
            }),
            log: Log{},
            rpc: match crate::rpc::init_rpc(&init_config) {
                Ok(rpc) => RefCell::new(rpc),
                Err(e) => panic!("rpc init error! {}", e)
            },
            logics: Arc::new(Mutex::new(vec![])),
            init_config
        }
    }

}

impl Context{
    pub fn registry_logic_module<T>(&mut self, logic: T) where T : LogicBase + Send + 'static {
        self.logics.lock().unwrap().deref_mut().push(Box::new(logic));
    }

    pub fn add_service_for_client(&mut self, service: ServerServiceDefinition) {
        self.rpc.into_inner().server_builder.into_inner().add_service(service);
    }
}

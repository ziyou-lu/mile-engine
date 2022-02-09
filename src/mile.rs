use std::net::TcpListener;
use std::ops::DerefMut;
use crate::common::error::Error;
use crate::context;
use crate::context::Context;

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
        初始化
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
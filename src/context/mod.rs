use crate::InitConfig;
use rbatis::rbatis::Rbatis;
use crate::common::log::Log;

pub struct Context {
    pub init_config: InitConfig,
    pub rbatis: Rbatis,
    pub log: Log
}

impl Default for Context {
    fn default() -> Self {
        let init_config = InitConfig::default();

        Context {
            rbatis: rbatis::core::runtime::task::block_on(async {
                crate::db::init_rbatis(&init_config).await
            }),
            init_config,
            log: Log{}
        }
    }

}

lazy_static! {
    pub static ref CONTEXT: Context = Context::default();
}

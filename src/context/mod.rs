use crate::InitConfig;
use rbatis::rbatis::Rbatis;

pub struct Context {
    pub init_config: InitConfig,
    pub rbatis: Rbatis
}

impl Default for Context {
    fn default() -> Self {
        let init_config = InitConfig::default();

        Context {
            rbatis: rbatis::core::runtime::task::block_on(async {
                crate::db::init_rbatis(&init_config).await
            }),
            init_config
        }
    }

}

lazy_static! {
    pub static ref CONTEXT: Context = Context::default();
}

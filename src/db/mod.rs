use crate::InitConfig;
use rbatis::plugin::logic_delete::{RbatisLogicDeletePlugin};
use rbatis::rbatis::Rbatis;

pub mod mapper;

pub async fn init_rbatis(config: &InitConfig) -> Rbatis {
    let mut rbatis = Rbatis::new();
    //设置逻辑删除插件
    rbatis.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new_opt(
        &config.db.logic_column,
        config.db.logic_deleted as i32,
        config.db.logic_un_deleted as i32,
    )));
    if config.global.debug.eq(&false) && rbatis.is_debug_mode() {
        panic!(
            r#"已使用release模式，但是rbatis仍使用debug模式！请删除 Cargo.toml 中 rbatis的配置 features = ["debug_mode"]"#
        );
    }
    //连接数据库
    rbatis.link(&config.db.mysql_url).await.expect("[abs_admin] rbatis link database fail!");
    println!("[mile-engine] rbatis link database success!");
    return rbatis;
}

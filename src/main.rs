use hubbo_sso::config::init_configuration;
use hubbo_sso::web;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    // init_configuration()
    //     .await
    //     .expect("从nacos初始化系统配置信息失败");
    web::start_up().await
}

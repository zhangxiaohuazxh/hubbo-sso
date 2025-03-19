use hubbo_sso::server;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    // init_configuration()
    //     .await
    //     .expect("从nacos初始化系统配置信息失败");
    server::start_up().await
}

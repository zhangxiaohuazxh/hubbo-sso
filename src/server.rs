use crate::route;
use actix_web::{App, HttpServer};

pub async fn start_up() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(route::test_route_configure)
            .configure(route::ali_oauth2_configure)
    })
    .bind("0.0.0.0:8080")?
    // 主线程数 没上线就设置少一点
    .workers(3)
    // 服务器关闭超时时间,收到关闭信号后最多60秒的等待时长,超时则会丢弃所有的任务强制关闭服务器
    .shutdown_timeout(60)
    .run()
    .await
}

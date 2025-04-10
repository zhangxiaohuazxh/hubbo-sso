use crate::route;
use actix_web::{App, HttpServer};
use anyhow::Result;
use log::LevelFilter;
use log4rs::Config;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;

pub async fn start_up() -> std::io::Result<()> {
    init_logger().unwrap();
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

fn init_logger() -> Result<()> {
    let pattern = "{h({T})}-{d(%Y-%m-%d %H:%M:%S)}-[{h({l}-{M}-{L})}]-{m}{n}";
    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();
    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build("logs/hubbo.log")?;
    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(
            Root::builder()
                .appender("console")
                .appender("file")
                .build(LevelFilter::Info),
        )?;
    log4rs::init_config(config)?;
    Ok(())
}

pub(crate) mod model;
pub(crate) mod value_object_model;

use crate::web::model::ResponseWrapper;
use crate::web::value_object_model::User;
use actix_web::{get, post, web, App, HttpServer, Responder};

#[get("")]
async fn index() -> impl Responder {
    web::Json(ResponseWrapper::<i32>::success())
}

#[post("args")]
async fn resolve_args(user: web::Form<User>) -> impl Responder {
    println!("接收到的用户信息 {:?}", user);
    web::Json(ResponseWrapper::<i32>::success())
}

pub async fn start_up() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/test").service(index).service(resolve_args)))
        .bind("0.0.0.0:8080")?
        // 主线程数 没上线就设置少一点
        .workers(3)
        // 服务器关闭超时时间,收到关闭信号后最多60秒的等待时长,超时则会丢弃所有的任务强制关闭服务器
        .shutdown_timeout(60)
        .run()
        .await
}

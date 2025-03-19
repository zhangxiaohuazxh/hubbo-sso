use crate::model::{request::User, response::ResponseWrapper};
use actix_web::{
    Responder, get, post,
    web::{self, ServiceConfig},
};
#[get("")]
async fn index() -> impl Responder {
    web::Json(ResponseWrapper::<i32>::success())
}

#[post("args")]
async fn resolve_args(user: web::Form<User>) -> impl Responder {
    println!("接收到的用户信息 {:?}", user);
    web::Json(ResponseWrapper::<i32>::success())
}

pub fn configure(config: &mut ServiceConfig) {
    config.service(web::scope("/test").service(index).service(resolve_args));
}

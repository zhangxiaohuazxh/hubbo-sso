use crate::model::request::AliCallbackRequestParam;
use crate::utils::cipher;
use actix_web::web::{self, ServiceConfig};
use actix_web::{HttpResponse, Responder, get};
use chrono::Local;
use reqwest::Client;
use url::Url;

const REDIRECT_URL: &str = "https://openauth.alipay.com/oauth2/publicAppAuthorize.htm?app_id=xxx&scope=auth_user&redirect_uri=http://hubbo.iepose.cn/ali/callback";

const GATEWAY_URL: &str = "https://openapi.alipay.com/gateway.do";

#[get("/login")]
async fn ali_oauth_login() -> HttpResponse {
    HttpResponse::Found()
        .append_header(("location", REDIRECT_URL))
        .finish()
}

#[get("/callback")]
async fn ali_callback(param: web::Query<AliCallbackRequestParam>) -> impl Responder {
    println!("接收到的参数 {:#?}", param);
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let args = format!("{}", 1);
    let app_id = "xx";
    let signedContent = format!(
        "app_id={}&charset=UTF-8&code={}&format=json&grant_type=authorization_code&method=alipay.system.oauth.token&sign_type=RSA2&timestamp={}&version=1.0",
        app_id, &param.auth_code, timestamp
    );
    let sign = cipher::sign(signedContent.as_bytes()).await.unwrap();
    println!("signed content {}", signedContent);
    println!(" ");
    println!("sign {}", sign);
    println!(" ");
    let url = Url::parse(&format!("{}?{}&sign={}", GATEWAY_URL, signedContent, sign))
        .unwrap()
        .to_string();
    println!("url {:#?}", url);
    println!(" ");
    let res = Client::new()
        .get(url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("res {:#?}", res);
    "ok"
}

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        web::scope("/ali")
            .service(ali_oauth_login)
            .service(ali_callback),
    );
}

#[cfg(test)]
mod test {
    #[actix_rt::test]
    async fn test_timestamp() {
        let time = chrono::Utc::now();
        let time = time.timestamp_millis();
        println!("time {:?}", time);
    }
}

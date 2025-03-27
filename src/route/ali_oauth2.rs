use crate::model::request::AliCallbackRequestParam;
use crate::utils::cipher;
use actix_web::web::{self, ServiceConfig};
use actix_web::{get, HttpResponse, Responder};
use anyhow::Context;
use chrono::Local;
use reqwest::Client;

const REDIRECT_URL: &str = "https://openauth.alipay.com/oauth2/publicAppAuthorize.htm?app_id=2021005130670109&scope=auth_user&redirect_uri=http://hubbo.iepose.cn/ali/callback";

const GATEWAY_URL: &str = "https://openapi.alipay.com/gateway.do";

#[get("/login")]
async fn ali_oauth_login() -> HttpResponse {
    HttpResponse::Found()
        .append_header(("location", REDIRECT_URL))
        .finish()
}

#[get("/callback")]
async fn ali_callback(param: web::Query<AliCallbackRequestParam>) -> impl Responder {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let app_id = "2021005130670109";
    let signed_content = format!(
        "app_id={}&charset=UTF-8&code={}&format=json&grant_type=authorization_code&method=alipay.system.oauth.token&sign_type=RSA2&timestamp={}&version=1.0",
        app_id, &param.auth_code, &timestamp
    );
    let sign = cipher::sign(signed_content.as_bytes())
        .await
        .with_context(|| "签名失败")
        .unwrap();
    let sign = url::form_urlencoded::Serializer::new(String::new())
        .append_pair("", &sign)
        .finish()
        .trim_start_matches("=")
        .to_string();
    let url = format!(
        "?charset=UTF-8&method=alipay.system.oauth.token&sign={0}&version=1.0&app_id={1}&sign_type=RSA2&timestamp={2}&format=json",
        sign, app_id, timestamp
    );
    let mut new_url = String::from(GATEWAY_URL);
    new_url.push_str(&url[..]);
    let url = url::Url::parse(&new_url).unwrap().to_string();
    let body_string = format!("grant_type=authorization_code&code={}", param.auth_code);
    let resp = Client::new()
        .post(url)
        .body(body_string)
        .header("Accept", "text/plain,text/xml,text/javascript,text/html")
        .header(
            "Content-Type",
            "application/x-www-form-urlencoded;charset=UTF-8",
        )
        .send()
        .await
        .unwrap();
    let headers = resp.headers();
    println!("headers {:#?}", headers);
    let trace_id = headers.get("trace_id").unwrap();
    println!("trace id  {:?}", trace_id);
    let res = resp.text().await.unwrap();
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

use crate::model::request::ali_pay::{AliCallbackRequestParam, AliPayOauthResponse};
use crate::utils::cipher;
use actix_web::web::{self, ServiceConfig};
use actix_web::{HttpResponse, Responder, get};
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
    let res: AliPayOauthResponse = resp.json().await.unwrap();
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
    use super::*;
    use crate::utils::cipher::sign;
    use chrono::Utc;
    use std::collections::BTreeMap;

    #[actix_rt::test]
    async fn test_timestamp() {
        let time = chrono::Utc::now();
        let time = time.timestamp_millis();
        println!("time {:?}", time);
    }

    #[actix_rt::test]
    async fn test_get_user_info() -> anyhow::Result<()> {
        let url = "https://openapi.alipay.com/gateway.do";
        let token = "";
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let mut args = BTreeMap::new();
        args.insert("charset", "UTF-8");
        args.insert("method", "alipay.user.info.share");
        args.insert("app_id", "2021005130670109");
        args.insert("format", "json");
        args.insert("version", "1.0");
        args.insert("sign_type", "RSA2");
        args.insert("timestamp", &timestamp);
        args.insert("auth_token", token);
        let mut args_string = String::new();
        args.iter().enumerate().for_each(|e| {
            let index: usize = e.0;
            if index == 0_usize {
                args_string.push_str(&format!("{}={}", e.1.0, e.1.1));
            } else {
                args_string.push_str(&format!("&{}={}", e.1.0, e.1.1));
            }
        });
        println!("args {:#?}", args);
        println!("args_string {:#?}", args_string);
        let sign = cipher::sign(args_string.as_bytes()).await?;
        args.remove("auth_token").unwrap();
        args_string.clear();
        args.iter().enumerate().for_each(|e| {
            let index: usize = e.0;
            if index == 0_usize {
                args_string.push_str(&format!("?{}={}", e.1.0, e.1.1));
            } else {
                args_string.push_str(&format!("&{}={}", e.1.0, e.1.1));
            }
        });
        let mut url_string = String::new();
        url_string.push_str(url);
        url_string.push_str(&args_string);
        println!("url string {}", url_string);
        let sign = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("", &sign)
            .finish()
            .trim_start_matches("=")
            .to_string();
        let mut url = url::Url::parse(&url_string)?.to_string();
        url.push_str("&sign=");
        url.push_str(&sign);
        println!("Url {}", url);
        let resp = Client::new()
            .post(url)
            .body(format!("auth_token=${token}"))
            .send()
            .await?
            .text()
            .await?;
        println!("remote response {:#?}", resp);
        Ok(())
    }
}

use crate::model::request::AliCallbackRequestParam;
use actix_web::web::{self, ServiceConfig};
use actix_web::{get, HttpResponse, Responder};
use chrono::Utc;

const REDIRECT_URL: &str = "https://openauth.alipay.com/oauth2/publicAppAuthorize.htm?app_id=${app_id}&scope=auth_user&redirect_uri=http://hubbo.${domain}/ali/callback";

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
    let timestamp = Utc::now().timestamp_millis();
    let args = format!("{}", 1);
    let params = [
        ("charset", "UTF-8"),
        ("method", "alipay.system.oauth.token"),
        ("format", "json"),
        ("sign", "${sign}"),
        ("app_id", "${app_id}"),
        ("version", "1.0"),
        ("sign_type", "RSA2"),
        ("timestamp", &timestamp.to_string()),
        ("refresh_token", &param.auth_code),
        ("code", &param.auth_code),
        ("grant_type", "authorization_code"),
    ];
    // let res: String = Client::new()
    //     .get(GATEWAY_URL)
    //     .form(&params)
    //     .send()
    //     .await
    //     .unwrap()
    //     .json()
    //     .await
    //     .unwrap();
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

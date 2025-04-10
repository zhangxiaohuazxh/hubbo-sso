use crate::model::request::ali_pay::{AliCallbackRequestParam, AliPayOauthResponse};
use crate::model::response::ResponseWrapper;
use crate::utils::request;
use crate::utils::request::MediaType;
use crate::utils::{
    cipher,
    web::{QueryParam, QueryStringBuilder},
};
use actix_web::web::{self, ServiceConfig};
use actix_web::{HttpResponse, get};
use chrono::Local;
use reqwest::Client;
use std::collections::BTreeMap;

const REDIRECT_URL: &str = "https://openauth.alipay.com/oauth2/publicAppAuthorize.htm?app_id=2021005130670109&scope=auth_user&redirect_uri=http://hubbo.iepose.cn/ali/callback";

const GATEWAY_URL: &str = "https://openapi.alipay.com/gateway.do";

fn ali_pay_common_args() -> BTreeMap<&'static str, QueryParam<'static>> {
    let mut map = BTreeMap::new();
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let app_id = "2021005130670109";
    map.insert("app_id", QueryParam::Str(app_id));
    map.insert("charset", QueryParam::Str("UTF-8"));
    map.insert("format", QueryParam::Str("json"));
    map.insert("sign_type", QueryParam::Str("RSA2"));
    map.insert("timestamp", QueryParam::String(timestamp));
    map.insert("version", QueryParam::Str("1.0"));
    map
}

#[get("/login")]
async fn ali_oauth_login() -> HttpResponse {
    HttpResponse::Found()
        .append_header(("location", REDIRECT_URL))
        .finish()
}

#[get("/callback")]
async fn ali_callback(param: web::Query<AliCallbackRequestParam>) -> ResponseWrapper<i32> {
    let map = ali_pay_common_args();
    let mut query_string_builder = QueryStringBuilder::new_with_initial_data(map);
    query_string_builder
        .append_query_param(("method", QueryParam::Str("alipay.system.oauth.token")));
    query_string_builder.append_query_param(("code", QueryParam::Str(&param.auth_code)));
    query_string_builder.append_query_param(("grant_type", QueryParam::Str("authorization_code")));
    query_string_builder.append_query_param((
        "sign",
        QueryParam::String(
            cipher::sign(query_string_builder.query_string().unwrap().as_bytes())
                .await
                .unwrap(),
        ),
    ));
    let url = query_string_builder
        .url_encode_query_string_with_base_url(GATEWAY_URL)
        .unwrap();
    let body_string = format!("grant_type=authorization_code&code={}", param.auth_code);
    let res = request::do_post(&url, MediaType::FormUrlEncoded, Some(body_string), None)
        .await
        .unwrap();
    let res: AliPayOauthResponse = serde_json::from_str(&res).unwrap();
    println!("res {:#?}", res);
    ResponseWrapper::success()
}

#[allow(unused)]
async fn get_user_info_from_ali_pay() -> anyhow::Result<()> {
    let mut query_string_builder = QueryStringBuilder::new_with_initial_data(ali_pay_common_args());
    query_string_builder.append_query_param(("method", QueryParam::Str("alipay.user.info.share")));
    query_string_builder.append_query_param(("auth_token", QueryParam::Str("")));
    let sign = cipher::sign(query_string_builder.query_string()?.as_bytes()).await?;
    query_string_builder.append_query_param(("sign", QueryParam::String(sign)));
    let url = query_string_builder.url_encode_query_string_with_base_url(GATEWAY_URL)?;
    let resp = Client::new().post(url).send().await?.text().await?;
    println!("获取到的用户信息 {:#?}", resp);
    Ok(())
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
    use crate::model::request::ali_pay::AlipaySystemOauthTokenResponse;
    use std::str::FromStr;

    #[actix_rt::test]
    async fn test_timestamp() {
        let time = chrono::Utc::now();
        let time = time.timestamp_millis();
        println!("time {:?}", time);
    }

    #[actix_rt::test]
    async fn test_get_user_info() -> anyhow::Result<()> {
        get_user_info_from_ali_pay().await?;
        Ok(())
    }

    #[actix_rt::test]
    async fn test_verify_alipay_sign() -> anyhow::Result<()> {
        let oauth_token_response = AlipaySystemOauthTokenResponse {
            access_token: String::from_str("")?,
            auth_start: String::from_str("2025-03-23 18:59:42")?,
            expires_in: 1296000,
            re_expires_in: 2592000,
            refresh_token: String::from_str("")?,
            open_id: String::from_str("")?,
        };
        let sign = "VnnSCXT5XivGj+0+++FPr2eZaXrp9gVPrm8G1G/ofUxCD2Kj/NDOQCIw==";
        cipher::verify(&serde_json::to_vec(&oauth_token_response)?, &sign).await?;
        Ok(())
    }
}

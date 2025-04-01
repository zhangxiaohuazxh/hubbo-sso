use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub username: String,
    #[serde(default)]
    pub passwd: String,
}

pub mod ali_pay {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct AliCallbackRequestParam {
        /* 授权码 */
        pub auth_code: String,
        pub app_id: u64,
        pub source: String,
        pub scope: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct AliPayOauthResponse {
        alipay_system_oauth_token_response: AlipaySystemOauthTokenResponse,
        sign: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct AlipaySystemOauthTokenResponse {
        pub access_token: String,
        pub auth_start: String,
        pub expires_in: u32,
        pub re_expires_in: u32,
        pub refresh_token: String,
        pub open_id: String,
    }
}

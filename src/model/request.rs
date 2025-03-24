use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub username: String,
    #[serde(default)]
    pub passwd: String,
}

#[derive(Debug, Deserialize)]
pub struct AliCallbackRequestParam {
    /* 授权码 */
    pub auth_code: String,
    pub app_id: u64,
    pub source: String,
    pub scope: String,
}

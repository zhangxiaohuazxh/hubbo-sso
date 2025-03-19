use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub username: String,
    #[serde(default)]
    pub passwd: String,
}

use clap::Parser;
use serde::Deserialize;
use std::{fs::File, io};

#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
    pub app: AppConfigurationProperties,
}

#[derive(Debug, Deserialize, Default)]
pub struct AppConfigurationProperties {
    // #[serde[default="default_data_id"]]
    pub name: String,
    // #[serde(default = "default_app_running_mode")]
    pub mode: String,
    pub nacos: NacosProperties,
}

#[allow(unused)]
#[derive(Debug, Deserialize, Parser)]
pub struct NacosProperties {
    #[serde[default="default_nacos_enabled_status"]]
    pub enabled: bool,
    #[serde(default = "default_server_address")]
    pub server_address: String,
    #[serde(default = "default_server_port")]
    pub port: u16,
    #[serde(default = "default_nacos_username")]
    pub username: String,
    #[serde(default = "default_naocs_passwd")]
    pub passwd: String,
    #[serde(default = "default_group_id")]
    pub group_id: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(default = "default_data_id")]
    pub data_id: String,
}

fn default_server_address() -> String {
    "localhost".to_string()
}

fn default_server_port() -> u16 {
    8848
}

fn default_nacos_username() -> String {
    Into::<String>::into("nacos")
}

fn default_naocs_passwd() -> String {
    Into::<String>::into("nacos")
}

fn default_group_id() -> String {
    Into::<String>::into("DEFAULT_GROUP")
}

fn default_data_id() -> String {
    String::from("hubbo-sso")
}

impl Default for NacosProperties {
    fn default() -> Self {
        NacosProperties {
            server_address: default_server_address(),
            port: default_server_port(),
            username: default_nacos_username(),
            passwd: default_naocs_passwd(),
            data_id: default_data_id(),
            group_id: default_group_id(),
            namespace: "".to_string(),
            enabled: true,
        }
    }
}

fn default_app_running_mode() -> String {
    "dev".to_string()
}

fn default_nacos_enabled_status() -> bool {
    true
}

/// 解析配置文件中指定的 app name作为配置的data id并区分dev prod环境
fn parse_app_name(app_config_string: &str) -> Result<String, Box<dyn std::error::Error>> {
    let config = serde_yaml::from_str::<AppConfig>(app_config_string)?;
    Ok(format!("{}-{}", config.app.name, config.app.mode))
}

pub async fn get_nacos_configuration() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let content = io::read_to_string(File::open("./config.yaml")?)?;
    println!("content {}", content);
    let config_file_name = parse_app_name(&content)?;
    let mut app_config = serde_yaml::from_str::<AppConfig>(&content)?;
    app_config.app.name = config_file_name;
    Ok(app_config)
}

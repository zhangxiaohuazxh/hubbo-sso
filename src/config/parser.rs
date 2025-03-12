use clap::Parser;
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct NacosConfig {
    #[serde(default)]
    pub nacos: NacosProperties,
}

#[allow(unused)]
#[derive(Debug, Deserialize, Parser)]
pub struct NacosProperties {
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

impl Default for NacosConfig {
    fn default() -> Self {
        NacosConfig {
            nacos: NacosProperties::default(),
        }
    }
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
        }
    }
}

/// 解析配置文件中指定的 app name作为配置的data id并区分dev prod环境
fn parse_app_name() -> String {
    todo!()
}

pub async fn get_nacos_configuration() -> Result<NacosConfig, Box<dyn std::error::Error>> {
    Ok(serde_yaml::from_reader(File::open("./config.yaml")?)?)
}

use clap::Parser;
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct NacosConfig {
    pub nacos: NacosProperties,
}

#[allow(unused)]
#[derive(Debug, Deserialize, Parser)]
pub struct NacosProperties {
    #[serde(default)]
    pub server_address: String,
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub passwd: String,
    #[serde(default)]
    pub group_id: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(default)]
    pub data_id: String,
}

pub async fn get_nacos_configuration() -> Result<NacosConfig, Box<dyn std::error::Error>> {
    Ok(serde_yaml::from_reader(File::open("./config.yaml")?)?)
}

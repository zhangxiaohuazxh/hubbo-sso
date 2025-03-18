mod parser;

use crate::config::parser::{AppConfig, parse_local_configuration_file};
use nacos_sdk::api::config::ConfigServiceBuilder;
use nacos_sdk::api::config::{ConfigChangeListener, ConfigResponse, ConfigService};
use nacos_sdk::api::props::ClientProps;
use std::sync::Arc;
use std::time::Duration;

pub struct ConfigSubscriber;

// todo 实现配置变更监听
impl ConfigChangeListener for ConfigSubscriber {
    fn notify(&self, config_resp: ConfigResponse) {
        println!("接收到配置变更的通知 {:#?}", config_resp)
    }
}

#[allow(unused)]
pub async fn init_configuration() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = parse_local_configuration_file().await?;
    println!("nacos config {:?}", config);
    get_configuration_from_remote_server(config).await?;
    // todo 服务注册与发现暂不做实现,待0.1发版之后再做打算
    Ok(())
}

#[allow(unused)]
async fn get_configuration_from_remote_server(
    app_config: AppConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let server_properties = app_config.app.nacos;
    let data_id = app_config.app.name;
    let group_id = server_properties.group_id;
    let config_service = ConfigServiceBuilder::new(
        ClientProps::new()
            .server_addr(server_properties.server_address)
            .namespace(server_properties.namespace)
            .auth_username(server_properties.username)
            .auth_password(server_properties.passwd)
            .app_name(&data_id),
    )
    .enable_auth_plugin_http()
    .build()?;
    let config_server_response = config_service
        .get_config(data_id.clone(), group_id.clone())
        .await?;
    println!("响应信息 {}", config_server_response.content());
    if let Ok(_) = config_service
        .add_listener(data_id, group_id, Arc::new(ConfigSubscriber))
        .await
    {
        println!("监听器启动成功")
    } else {
        panic!("监听器启动失败")
    }
    tokio::time::sleep(Duration::from_secs(60)).await;
    Ok(())
}

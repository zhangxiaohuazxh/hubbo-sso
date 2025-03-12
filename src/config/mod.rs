mod parser;
use crate::config::parser::{get_nacos_configuration, NacosConfig};
use nacos_sdk::api::config::ConfigService;
use nacos_sdk::api::config::ConfigServiceBuilder;
use nacos_sdk::api::props::ClientProps;

#[allow(unused)]
pub async fn init_configuration() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = get_nacos_configuration().await?;
    println!("nacos config {:?}", config);
    get_configuration_from_remote_server(config).await?;
    Ok(())
}

#[allow(unused)]
async fn get_configuration_from_remote_server(
    nacos_config: NacosConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let server_properties = nacos_config.nacos;
    let data_id = server_properties.data_id;
    let config_server = ConfigServiceBuilder::new(
        ClientProps::new()
            .server_addr(server_properties.server_address)
            .namespace(server_properties.namespace)
            .auth_username(server_properties.username)
            .auth_password(server_properties.passwd)
            .app_name(&data_id),
    )
    .enable_auth_plugin_http()
    .build()?;
    let config_server_response = config_server
        .get_config(data_id, server_properties.group_id.to_string())
        .await?;
    println!("响应信息 {}", config_server_response.content());
    Ok(())
}

use nacos_sdk::api::config::ConfigService;
use nacos_sdk::api::config::ConfigServiceBuilder;
use nacos_sdk::api::props::ClientProps;

const ADDRESS: &str = "172.16.10.70";
const USERNAME: &str = "nacos";
const PASSWORD: &str = "*";
const APP_NAME: &str = "";
const NAME_SPACE: &str = "603bb0c0-f9e9-4dc9-81c7-c01356799a27";
const DATA_ID: &str = "";
const GROUP_ID: &str = "DEFAULT_GROUP";

#[allow(unused)]
pub async fn init_configuration() -> Result<(), Box<dyn std::error::Error>> {
    let config_server = ConfigServiceBuilder::new(
        ClientProps::new()
            .namespace(NAME_SPACE)
            .server_addr(ADDRESS)
            .auth_username(USERNAME)
            .auth_password(PASSWORD)
            .app_name(APP_NAME),
    )
    .enable_auth_plugin_http()
    .build()?;
    let config_server_response = config_server
        .get_config(DATA_ID.to_string(), GROUP_ID.to_string())
        .await?;
    println!("响应信息 {}", config_server_response.content());
    Ok(())
}

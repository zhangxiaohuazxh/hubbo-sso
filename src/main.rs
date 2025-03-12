use hubbo_sso::config::init_configuration;

#[tokio::main]
async fn main() {
    init_configuration().await.expect("get configuration error");
}

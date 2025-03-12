use mongodb::{
    Client, Database,
    options::{ClientOptions, ServerApi, ServerApiVersion},
};
use std::sync::{Arc, OnceLock};

#[allow(unused)]
pub static MONGO_CLIENT: OnceLock<Arc<Client>> = OnceLock::new();

#[allow(unused)]
pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = "fuck";
    let mut client_options = ClientOptions::parse(db_url).await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    unsafe {
        MONGO_CLIENT.get_or_init(move || Arc::new(Client::with_options(client_options).unwrap()));
    }
    Ok(())
}

#[allow(unused)]
async fn db(database_name: &str) -> Result<Database, Box<dyn std::error::Error>> {
    Ok(MONGO_CLIENT.get().unwrap().database(database_name))
}

#[cfg(test)]
mod test {
    use super::*;
    use futures_util::TryStreamExt;
    use mongodb::bson::doc;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct User {
        username: String,
        country: String,
        address: String,
        hobby: Vec<String>,
    }

    #[actix_web::rt::test]
    async fn test_connect_mongo() {
        init().await.unwrap();
        let mut cursor = db("test")
            .await
            .unwrap()
            .collection::<User>("users")
            .find(doc! {})
            .await
            .unwrap();
        while let Some(user) = cursor.try_next().await.unwrap() {
            println!("{:#?}", user);
        }
    }
}

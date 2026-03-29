use anyhow::Result;
use mongodb::{Client, Database, options::ClientOptions};

pub struct MongoService {
    client: mongodb::Client,
    database: String,
}

impl MongoService {
    pub async fn new() -> Result<Self> {
        let host = std::env::var("MONGO_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("MONGO_PORT").unwrap_or_else(|_| "27017".to_string());
        let username = std::env::var("MONGO_USERNAME").unwrap_or_else(|_| "root".to_string());
        let password = std::env::var("MONGO_PASSWORD").unwrap_or_else(|_| "password".to_string());
        let database =
            std::env::var("MONGO_DATABASE").unwrap_or_else(|_| "profile_service".to_string());

        let uri = format!(
            "mongodb://{}:{}@{}:{}/{}",
            username, password, host, port, database
        );

        let mut client_options = ClientOptions::parse(uri).await?;

        client_options.max_pool_size = Some(20);
        client_options.min_pool_size = Some(5);

        let client = Client::with_options(client_options)?;

        Ok(MongoService { client, database })
    }

    pub fn get_database(&self) -> Database {
        self.client.database(&self.database)
    }
}

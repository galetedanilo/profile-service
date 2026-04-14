use anyhow::Result;
use mongodb::{Client, Database, options::ClientOptions};

pub struct MongoService {
    client: mongodb::Client,
    database: String,
}

impl MongoService {
    pub async fn new(
        username: String,
        password: String,
        hostname: String,
        database: String,
    ) -> Result<Self> {
        let uri = format!(
            "mongodb://{}:{}@{}/{}",
            username, password, hostname, database
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

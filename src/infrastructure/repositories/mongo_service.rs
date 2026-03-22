pub struct MongoService {
    username: String,
    password: String,
    host: String,
    port: u16,
    database: String,
    client: Option<mongodb::Client>,
}

impl MongoService {
    pub fn new(username: String, password: String, host: String, port: u16, database: String) -> Self {
        MongoService {
            username,
            password,
            host,
            port,
            database,
            client: None,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let uri = format!(
            "mongodb://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        );

        let client = mongodb::Client::with_uri_str(&uri).await?;

        self.client = Some(client);

        Ok(())
    }

    // Additional methods for CRUD operations can be added here
}
#[derive(Clone)]
pub struct Config {
    pub request_host: String,
    pub addr: String,
    pub public_key_path: String,
    pub db_protocol: String,
    pub hostname: String,
    pub database: String,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn new(
        request_host: String,
        addr: String,
        public_key_path: String,
        db_protocol: String,
        hostname: String,
        database: String,
        username: String,
        password: String,
    ) -> Self {
        Self {
            request_host,
            addr,
            public_key_path,
            db_protocol,
            hostname,
            database,
            username,
            password,
        }
    }
}

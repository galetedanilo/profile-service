use axum::http::request;

#[derive(Clone)]
pub struct Config {
    pub request_host: String,
    pub addr: String,
    pub public_key_path: String,
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
        hostname: String,
        database: String,
        username: String,
        password: String,
    ) -> Self {
        Self {
            request_host,
            addr,
            public_key_path,
            hostname,
            database,
            username,
            password,
        }
    }
}

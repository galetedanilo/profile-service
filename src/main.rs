use std::sync::Arc;

use anyhow::Result;
use dotenvy::dotenv;
use presentation::api::service::Service;

use crate::infrastructure::repositories::{
    mongo_profile_repo::MongoProfileRepository, mongo_service::MongoService,
};

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let mongo_service = MongoService::new().await?;
    let profile_repository = MongoProfileRepository::new(Arc::new(mongo_service));

    let request_host =
        std::env::var("REQUEST_HOST").unwrap_or_else(|_| "http://localhost:3000".into());
    let service_addr = std::env::var("SERVICE_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into());

    Service::run(profile_repository, request_host, service_addr).await;

    Ok(())
}

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

    Service::run(profile_repository).await;

    Ok(())
}

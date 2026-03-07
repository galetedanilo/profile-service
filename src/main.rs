use presentation::api::service::Service;

pub mod application;
pub mod domain;
pub mod presentation;

#[tokio::main]
async fn main() {
    Service::run().await;
}

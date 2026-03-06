use axum::{
    Router,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use super::handlers::{
    create_profile::create_profile_handler, get_profile_by_id::get_profile_by_id_handler,
    update_profile_by_id::update_profile_by_id_handler,
};

pub struct Service {}

impl Service {
    pub async fn run() {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .init();

        let routers = Router::new()
            .route("/profiles", post(create_profile_handler))
            .route(
                "/profiles/{id}",
                get(get_profile_by_id_handler).put(update_profile_by_id_handler),
            );

        let app = Router::new()
            .nest("/v1", routers)
            .layer(TraceLayer::new_for_http());

        let listener = tokio::net::TcpListener::bind("127.0.0.2:3000")
            .await
            .unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}

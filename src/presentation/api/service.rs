use std::{net::SocketAddr, sync::Arc};

use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
    routing::{get, post},
};
use jsonwebtoken::DecodingKey;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    infrastructure::repositories::{
        mongo_profile_repo::MongoProfileRepository, mongo_service::MongoService,
    },
    presentation::api::{
        handlers::health_handler::health_handler,
        helpers::{app_state::AppState, config::Config},
    },
};

use super::handlers::{
    create_profile_handler::create_profile_handler,
    get_profile_by_id_handler::get_profile_by_id_handler,
    update_profile_by_id_handler::update_profile_by_id_handler,
};

pub struct Service {
    config: Config,
}

impl Service {
    pub fn start() -> Self {
        let config = Config::new(
            std::env::var("REQUEST_HOST").expect("REQUEST_HOST must be set"),
            std::env::var("SERVICE_ADDR").expect("SERVICE_ADDR must be set"),
            std::env::var("PUBLIC_KEY_PATH").expect("PUBLIC_KEY_PATH must be set"),
            std::env::var("MONGO_DB_PROTOCOL").expect("MONGO_DB_PROTOCOL must be set"),
            std::env::var("MONGO_HOSTNAME").expect("MONGO_HOSTNAME must be set"),
            std::env::var("MONGO_DATABASE").expect("MONGO_DATABASE must be set"),
            std::env::var("MONGO_USERNAME").expect("MONGO_USERNAME must be set"),
            std::env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD must be set"),
        );

        Service { config }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .init();

        let cors_layer = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::PUT])
            .allow_origin(HeaderValue::from_str(&self.config.request_host).unwrap())
            .allow_headers([AUTHORIZATION, CONTENT_TYPE])
            .allow_credentials(true);

        let governor_conf = GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(8)
            .finish()
            .unwrap();

        let routers = Router::new()
            .route("/", post(create_profile_handler))
            .route(
                "/{id}",
                get(get_profile_by_id_handler).put(update_profile_by_id_handler),
            );

        let pem_content =
            std::fs::read(&self.config.public_key_path).expect("Failed to view EdDSA public key");

        let decoding_key = DecodingKey::from_ed_pem(&pem_content).expect("Invalid EdDSA key");

        let mongo_service = MongoService::new(
            self.config.db_protocol.clone(),
            self.config.username.clone(),
            self.config.password.clone(),
            self.config.hostname.clone(),
            self.config.database.clone(),
        )
        .await?;

        let repository = MongoProfileRepository::new(Arc::new(mongo_service));

        let state = AppState::new(Arc::new(repository), Arc::new(decoding_key));

        let app = Router::new()
            .nest("/profiles", routers)
            .route("/health", get(health_handler))
            .with_state(state)
            .layer(TraceLayer::new_for_http())
            .layer(GovernorLayer::new(governor_conf))
            .layer(cors_layer);

        let listener = tokio::net::TcpListener::bind(&self.config.addr).await?;

        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;

        Ok(())
    }
}

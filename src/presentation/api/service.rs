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
    application::use_cases::{
        create_profile::CreateProfileUseCase, get_profile_by_id::GetProfileByIdUseCase,
        update_profile::UpdateProfileUseCase,
    },
    domain::repositories::profile_repo::ProfileRepository,
    infrastructure::repositories::mongo_profile_repo::MongoProfileRepository,
};

use super::handlers::{
    create_profile::create_profile_handler, get_profile_by_id::get_profile_by_id_handler,
    update_profile_by_id::update_profile_by_id_handler,
};

#[derive(Clone)]
pub struct AppState<R: ProfileRepository> {
    pub create_profile_use_case: Arc<CreateProfileUseCase<R>>,
    pub get_profile_by_id_use_case: Arc<GetProfileByIdUseCase<R>>,
    pub update_profile_use_case: Arc<UpdateProfileUseCase<R>>,
    pub decoding_key: Arc<DecodingKey>,
}

impl<R: ProfileRepository> AppState<R> {
    pub fn new(repository: Arc<R>, decoding_key: Arc<DecodingKey>) -> Self {
        Self {
            create_profile_use_case: Arc::new(CreateProfileUseCase::new(Arc::clone(&repository))),
            get_profile_by_id_use_case: Arc::new(GetProfileByIdUseCase::new(Arc::clone(
                &repository,
            ))),
            update_profile_use_case: Arc::new(UpdateProfileUseCase::new(Arc::clone(&repository))),
            decoding_key,
        }
    }
}

pub struct Service {}

impl Service {
    pub async fn run(respository: MongoProfileRepository, request_host: String, addr: String) {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .init();

        let cors_layer = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::PUT])
            .allow_origin(HeaderValue::from_str(&request_host).unwrap())
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
            std::fs::read("./keys/public_key.pem").expect("Failed to view EdDSA public key");

        let decoding_key = DecodingKey::from_ed_pem(&pem_content).expect("Invalid EdDSA key");

        let state = AppState::new(Arc::new(respository), Arc::new(decoding_key));

        let app = Router::new()
            .nest("/profiles", routers)
            .with_state(state)
            .layer(TraceLayer::new_for_http())
            .layer(GovernorLayer::new(governor_conf))
            .layer(cors_layer);

        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap();
    }
}

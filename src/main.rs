use axum::http::{
    HeaderValue, Method,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};

use surrealdb_axum_demo::infrastructure::data::db_context::surreal_context::connect_db;
use surrealdb_axum_demo::{api::router::create_router, settings::Settings};
use tower_http::cors::CorsLayer;

use std::sync::OnceLock;

use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

static SETTINGS: OnceLock<Settings> = OnceLock::new();

#[tokio::main]
async fn main() {
    // Set up tracing subscriber with env filter
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true); // creates a formatting layer
    tracing_subscriber::Registry::default()
        .with(env_filter) //Adds the env filter to the registry
        .with(fmt_layer) //Adds the formatting layer to the registry
        .init(); //initialises the subscriber

    let settings = SETTINGS.get_or_init(|| Settings::new().unwrap());
    // tracing::info!("Settings configuration: {:?}", settings);

    connect_db(settings.db_config.clone()).await.unwrap();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router().layer(cors);

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

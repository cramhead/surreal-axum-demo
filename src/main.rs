use axum::http::{
    HeaderValue, Method,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};

use surrealdb_axum_demo::api::router::create_router;
use surrealdb_axum_demo::infrastructure::data::db_context::surreal_context::{
    DbConfig, connect_db,
};
use tower_http::cors::CorsLayer;

use config::FileFormat;
use config::{Config, Environment, File};
use std::{collections::HashMap, time::Duration};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .event_format(
            tracing_subscriber::fmt::format()
                .with_file(true)
                .with_line_number(true),
        )
        .init();

    let settings = Config::builder()
        // Add the .env file as a source
        .add_source(
            File::with_name(".env")
                .required(false)
                .format(FileFormat::Ini),
        )
        // Read from environment variables (overrides .env if present)
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    // Extract db config from environment variables
    let db_config = DbConfig {
        host: settings.get_string("DB_HOST").expect("DB_HOST must be set"),
        port: settings.get_int("DB_PORT").expect("DB_PORT must be set") as u16,
        username: settings.get_string("DB_USER").expect("DB_USER must be set"),
        password: settings
            .get_string("DB_PASSWORD")
            .expect("DB_PASSWORD must be set"),
        namespace: settings
            .get_string("DB_NAMESPACE")
            .expect("DB_NAMESPACE must be set"),
        db_name: settings.get_string("DB_NAME").expect("DB_NAME must be set"),
        timeout: Duration::from_secs(settings.get_int("DB_TIMEOUT").unwrap_or(30) as u64),
    };

    info!("Database configuration: {:?}", db_config);

    connect_db(db_config).await.unwrap();

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

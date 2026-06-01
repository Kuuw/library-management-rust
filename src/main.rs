use std::sync::Arc;
use anyhow::Result;
use axum::{Router, http::HeaderValue};
use reqwest::{Method, header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}};
use tower_http::cors::CorsLayer;
use std::net::SocketAddr;
use tracing_subscriber::{filter::LevelFilter};

mod config;
mod errors;
mod domain;
mod infrastructure;
mod state;
mod token;
mod middleware;
mod utils;

use config::Config;
use state::AppState;

use crate::infrastructure::routes::create_router;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    dotenvy::dotenv().ok();

    let config = Config::from_env()?;
    let state = AppState::new(config.clone()).await?;

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT]);

    sqlx::migrate!("./migrations").run(&state.db).await?;

    let app: Router = create_router(Arc::new(state.clone())).layer(cors.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

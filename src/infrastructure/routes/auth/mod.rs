pub mod dtos;
pub mod handlers;

use axum::{
    routing::{post, get},
    Router,
    middleware,
};
use std::sync::Arc;
use crate::state::AppState;
use crate::middleware::auth;

pub fn auth_handler() -> Router {
    Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .route("/me", get(handlers::me).layer(middleware::from_fn(auth)))
}

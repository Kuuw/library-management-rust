use axum::{
    Router, middleware, routing::{delete, get, post, put}
};

mod handlers;
pub mod dtos;

use crate::middleware::role_check;

use handlers::{create_author, update_author, delete_author, get_author_by_id, get_all_authors};

pub fn author_handler() -> Router {
    Router::new()
        .route("/create", post(create_author).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![2])
        })))
        .route("/update/{id}", put(update_author).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![2])
        })))
        .route("/delete/{id}", delete(delete_author).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![2])
        })))
        .route("/getById/{id}", get(get_author_by_id))
        .route("/getAll", get(get_all_authors))
}
use axum::{
    Router, middleware, routing::{delete, get, post, put}
};

mod handlers;
pub mod dtos;

use crate::middleware::role_check;

use handlers::{create_category, update_category, delete_category, get_category_by_id, get_all_categories};

pub fn category_handler() -> Router {
    Router::new()
        .route("/create", post(create_category).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![2])
        })))
        .route("/update/{id}", put(update_category).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![2])
        })))
        .route("/delete/{id}", delete(delete_category).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![2])
        })))
        .route("/getById/{id}", get(get_category_by_id))
        .route("/getAll", get(get_all_categories))
}
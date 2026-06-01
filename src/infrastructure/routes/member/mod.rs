use axum::{
    Router, middleware, routing::{delete, get, post, put}
};

mod handlers;
pub mod dtos;

use crate::middleware::role_check;

use handlers::{create_member, update_member, delete_member, get_member_by_id, get_all_members};

pub fn member_handler() -> Router {
    Router::new()
        .route("/create", post(create_member).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![2])
        })))
        .route("/update/{id}", put(update_member).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![2])
        })))
        .route("/delete/{id}", delete(delete_member).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![2])
        })))
        .route("/getById/{id}", get(get_member_by_id).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![2])
        })))
        .route("/getAll", get(get_all_members).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![2])
        })))
}
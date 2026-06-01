use axum::{
    Router, middleware, routing::{delete, get, post, put}
};

mod handlers;
pub mod dtos;

use crate::middleware::role_check;

use handlers::{create_book, update_book, delete_book, get_book_by_id, get_all_books};

pub fn book_handler() -> Router {
    Router::new()
        .route("/create", post(create_book).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![2])
        })))
        .route("/update/{id}", put(update_book).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![2])
        })))
        .route("/delete/{id}", delete(delete_book).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![2])
        })))
        .route("/getById/{id}", get(get_book_by_id))
        .route("/getAll", get(get_all_books))
}
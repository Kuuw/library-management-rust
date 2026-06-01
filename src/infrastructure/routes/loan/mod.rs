use axum::{
    Router, middleware, routing::{delete, get, post, put}
};

mod handlers;
pub mod dtos;

use crate::middleware::role_check;

use handlers::{create_loan, update_loan, delete_loan, get_loan_by_id, get_all_loans, loan_book, return_loan_handler, get_my_loans};

pub fn loan_handler() -> Router {
    Router::new()
        .route("/loan", post(loan_book).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![1, 2])
        })))
        .route("/return/{id}", put(return_loan_handler).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![2])
        })))
        .route("/my-loans", get(get_my_loans).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![1, 2])
        })))
        .route("/create", post(create_loan).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![2])
        })))
        .route("/update/{id}", put(update_loan).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![2])
        })))
        .route("/delete/{id}", delete(delete_loan).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![2])
        })))
        .route("/getById/{id}", get(get_loan_by_id).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![2])
        })))
        .route("/getAll", get(get_all_loans).layer(middleware::from_fn(|state, req, next| {
            role_check(state, req, next, vec![2])
        })))
}
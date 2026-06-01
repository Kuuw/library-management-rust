mod author;
mod book;
mod category;
mod loan;
mod member;
mod auth;

use crate::state::AppState;
use crate::middleware::auth;

use axum::{Extension, Router, middleware};
use tower_http::trace::TraceLayer;
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let api_route = Router::new()
        .nest("/auth", auth::auth_handler())
        .nest("/author", author::author_handler().layer(middleware::from_fn(auth)))
        .nest("/book", book::book_handler().layer(middleware::from_fn(auth)))
        .nest("/category", category::category_handler().layer(middleware::from_fn(auth)))
        .nest("/loan", loan::loan_handler().layer(middleware::from_fn(auth)))
        .nest("/member", member::member_handler().layer(middleware::from_fn(auth)))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state))
    ;

    Router::new().nest("/api", api_route)
}

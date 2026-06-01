use std::sync::Arc;

use axum::{Extension, Json, Router, extract::{Path, Query}, http::{HeaderMap, StatusCode, header}, response::{IntoResponse, Redirect}, routing::{get, post}};
use axum_extra::extract::cookie::Cookie;
use chrono::{Utc, Duration};
use validator::Validate;

use crate::{domain::models::author::{CreateAuthor, UpdateAuthor, AuthorQuery}, errors::AppError, infrastructure::routes::author::dtos::{CreateAuthorDto, UpdateAuthorDto, AuthorQueryDto}, state::AppState};
use crate::domain::repositories::author_repository::AuthorRepository;


pub async fn create_author(
    Extension(state): Extension<Arc<AppState>>,
    Json(body): Json<CreateAuthorDto>,
) -> Result<impl IntoResponse, AppError> {
    let author = state.create_author(CreateAuthor {
        first_name: body.first_name,
        last_name: body.last_name,
    }).await?; // Errors map to AppError automatically

    Ok((StatusCode::CREATED, Json(author)))
}

pub async fn update_author(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>, // Path should likely be i32 since Repository takes i32
    Json(body): Json<UpdateAuthorDto>,
) -> Result<impl IntoResponse, AppError> {
    let updated = state.update_author(id, UpdateAuthor {
        first_name: body.first_name,
        last_name: body.last_name,
    }).await?;

    match updated {
        Some(author) => Ok(Json(author)),
        None => Err(AppError::NotFound(format!("Author with id {} not found", id))),
    }
}

pub async fn delete_author(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    state.delete_author(id).await?;
    
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_author_by_id(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let author = state.get_author_by_id(id).await?;
    
    match author {
        Some(a) => Ok(Json(a)),
        None => Err(AppError::NotFound(format!("Author with id {} not found", id))),
    }
}

pub async fn get_all_authors(
    Extension(state): Extension<Arc<AppState>>,
    Query(query): Query<AuthorQueryDto>,
) -> Result<impl IntoResponse, AppError> {
    let authors = state.query_authors(AuthorQuery {
        first_name: query.first_name,
        last_name: query.last_name,
    }).await?;
    
    Ok(Json(authors))
}
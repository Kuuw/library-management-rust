use std::sync::Arc;

use axum::{Extension, Json, extract::{Path, Query}, http::StatusCode, response::IntoResponse};

use crate::{domain::models::category::{CreateCategory, UpdateCategory, CategoryQuery}, errors::AppError, infrastructure::routes::category::dtos::{CreateCategoryDto, UpdateCategoryDto, CategoryQueryDto}, state::AppState};
use crate::domain::repositories::category_repository::CategoryRepository;


pub async fn create_category(
    Extension(state): Extension<Arc<AppState>>,
    Json(body): Json<CreateCategoryDto>,
) -> Result<impl IntoResponse, AppError> {
    let category = state.create_category(CreateCategory {
        name: body.name,
        description: body.description,
    }).await?;

    Ok((StatusCode::CREATED, Json(category)))
}

pub async fn update_category(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateCategoryDto>,
) -> Result<impl IntoResponse, AppError> {
    let updated = state.update_category(id, UpdateCategory {
        name: body.name,
        description: body.description,
    }).await?;

    match updated {
        Some(category) => Ok(Json(category)),
        None => Err(AppError::NotFound(format!("Category with id {} not found", id))),
    }
}

pub async fn delete_category(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    state.delete_category(id).await?;
    
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_category_by_id(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let category = state.get_category_by_id(id).await?;
    
    match category {
        Some(c) => Ok(Json(c)),
        None => Err(AppError::NotFound(format!("Category with id {} not found", id))),
    }
}

pub async fn get_all_categories(
    Extension(state): Extension<Arc<AppState>>,
    Query(query): Query<CategoryQueryDto>,
) -> Result<impl IntoResponse, AppError> {
    let categories = state.query_categories(CategoryQuery {
        name: query.name,
    }).await?;
    
    Ok(Json(categories))
}
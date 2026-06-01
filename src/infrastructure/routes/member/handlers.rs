use std::sync::Arc;

use axum::{Extension, Json, extract::{Path, Query}, http::StatusCode, response::IntoResponse};

use crate::{domain::models::member::{CreateMember, UpdateMember, MemberQuery}, errors::AppError, infrastructure::routes::member::dtos::{CreateMemberDto, UpdateMemberDto, MemberQueryDto}, state::AppState};
use crate::domain::repositories::member_repository::MemberRepository;


pub async fn create_member(
    Extension(state): Extension<Arc<AppState>>,
    Json(body): Json<CreateMemberDto>,
) -> Result<impl IntoResponse, AppError> {
    let member = state.create_member(CreateMember {
        first_name: body.first_name,
        last_name: body.last_name,
        email: body.email,
        password_hash: body.password_hash,
        phone: body.phone,
        address: body.address,
        membership_date: body.membership_date,
        membership_status: body.membership_status,
    }).await?;

    Ok((StatusCode::CREATED, Json(member)))
}

pub async fn update_member(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateMemberDto>,
) -> Result<impl IntoResponse, AppError> {
    let updated = state.update_member(id, UpdateMember {
        first_name: body.first_name,
        last_name: body.last_name,
        email: body.email,
        password_hash: body.password_hash,
        phone: body.phone,
        address: body.address,
        membership_date: body.membership_date,
        membership_status: body.membership_status,
    }).await?;

    match updated {
        Some(member) => Ok(Json(member)),
        None => Err(AppError::NotFound(format!("Member with id {} not found", id))),
    }
}

pub async fn delete_member(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    state.delete_member(id).await?;
    
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_member_by_id(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let member = state.get_member_by_id(id).await?;
    
    match member {
        Some(m) => Ok(Json(m)),
        None => Err(AppError::NotFound(format!("Member with id {} not found", id))),
    }
}

pub async fn get_all_members(
    Extension(state): Extension<Arc<AppState>>,
    Query(query): Query<MemberQueryDto>,
) -> Result<impl IntoResponse, AppError> {
    let members = state.query_members(MemberQuery {
        first_name: query.first_name,
        last_name: query.last_name,
        email: query.email,
        membership_status: query.membership_status,
    }).await?;
    
    Ok(Json(members))
}
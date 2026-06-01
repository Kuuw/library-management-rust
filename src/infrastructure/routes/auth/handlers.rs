use std::sync::Arc;
use axum::{
    http::StatusCode,
    Json,
    Extension,
};

use crate::domain::models::member::CreateMember;
use crate::domain::repositories::member_repository::MemberRepository;
use crate::domain::repositories::loan_repository::LoanRepository;
use crate::errors::AppError;
use crate::infrastructure::routes::auth::dtos::*;
use crate::state::AppState;
use crate::{token, utils};

use crate::middleware::AuthUser;

pub async fn me(
    Extension(state): Extension<Arc<AppState>>,
    Extension(user): Extension<AuthUser>,
) -> Result<Json<MeResponse>, AppError> {
    let loans = state.get_loans_with_books_by_member(user.member.member_id).await?;
    
    Ok(Json(MeResponse {
        member_id: user.member.member_id,
        first_name: user.member.first_name,
        last_name: user.member.last_name,
        email: user.member.email,
        phone: user.member.phone,
        address: user.member.address,
        membership_date: user.member.membership_date,
        membership_status: user.member.membership_status,
        loans,
    }))
}

pub async fn register(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    let existing_member = state.get_member_by_email(&payload.email).await?;
    if existing_member.is_some() {
        return Err(AppError::Validation("Email already exists".to_string()));
    }

    let password_hash = utils::hash_password(&payload.password)
        .map_err(|e| AppError::Unexpected(anyhow::anyhow!("Failed to hash password: {}", e)))?;

    let create_member = CreateMember {
        first_name: payload.first_name,
        last_name: payload.last_name,
        email: payload.email,
        password_hash,
        phone: payload.phone,
        address: payload.address,
        membership_date: Some(chrono::Utc::now().to_rfc3339()),
        membership_status: 1, // Default status
    };

    let member = state.create_member(create_member).await?;
    let token = token::create_token(member.member_id, &state.config.jwt_secret)
        .map_err(|e| AppError::Unexpected(anyhow::anyhow!("Failed to create token: {}", e)))?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            member_id: member.member_id,
        }),
    ))
}

pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let member = state
        .get_member_by_email(&payload.email)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

    let is_valid = utils::verify_password(&payload.password, &member.password_hash)
        .map_err(|e| AppError::Unexpected(anyhow::anyhow!("Failed to verify password: {}", e)))?;

    if !is_valid {
        return Err(AppError::Unauthorized("Invalid email or password".to_string()));
    }

    let token = token::create_token(member.member_id, &state.config.jwt_secret)
        .map_err(|e| AppError::Unexpected(anyhow::anyhow!("Failed to create token: {}", e)))?;

    Ok(Json(AuthResponse {
        token,
        member_id: member.member_id,
    }))
}

use std::sync::Arc;
use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Extension
};
use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};

use crate::domain::models::member::Member;
use crate::domain::repositories::member_repository::MemberRepository;
use crate::errors::AppError;
use crate::token;
use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthUser {
    pub member: Member,
}

pub async fn auth(
    cookie_jar: CookieJar,
    Extension(app_state): Extension<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        })
        .ok_or_else(|| AppError::Unauthorized("Token not provided".to_string()))?;

    let member_id = token::decode_token(&token, &app_state.config.jwt_secret)
        .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;

    let member = app_state
        .get_member_by_id(member_id as i32)
        .await
        .map_err(|_| AppError::Unauthorized("User no longer exists".to_string()))?
        .ok_or_else(|| AppError::Unauthorized("User no longer exists".to_string()))?;

    req.extensions_mut().insert(AuthUser {
        member,
    });

    Ok(next.run(req).await)
}

pub async fn role_check(
    Extension(_app_state): Extension<Arc<AppState>>,
    req: Request,
    next: Next,
    required_roles: Vec<i64>,
) -> Result<impl IntoResponse, AppError> {
    let user = req
            .extensions()
            .get::<AuthUser>()
            .ok_or_else(|| {
                AppError::Unauthorized("User not authenticated".to_string())
            })?;
    
    if !required_roles.contains(&user.member.membership_status) {
        return Err(AppError::Unauthorized("Permission denied".to_string()));
    }

    Ok(next.run(req).await)
}
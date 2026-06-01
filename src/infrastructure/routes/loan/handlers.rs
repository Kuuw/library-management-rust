use std::sync::Arc;

use axum::{Extension, Json, extract::{Path, Query}, http::StatusCode, response::IntoResponse};

use crate::{domain::models::loan::{CreateLoan, UpdateLoan, LoanQuery, LoanWithBook}, errors::AppError, infrastructure::routes::loan::dtos::{CreateLoanDto, UpdateLoanDto, LoanQueryDto, LoanBookDto}, state::AppState};
use crate::domain::repositories::loan_repository::LoanRepository;
use crate::domain::repositories::book_repository::BookRepository;
use crate::middleware::AuthUser;

pub async fn loan_book(
    Extension(state): Extension<Arc<AppState>>,
    Extension(user): Extension<AuthUser>,
    Json(body): Json<LoanBookDto>,
) -> Result<impl IntoResponse, AppError> {
    // Availability check
    let book = state.get_book_by_id(body.book_id as i32).await?
        .ok_or_else(|| AppError::NotFound(format!("Book with id {} not found", body.book_id)))?;
    
    let active_loans = state.count_active_loans(body.book_id).await?;
    let total_copies = book.total_copies.unwrap_or(0);
    
    if active_loans >= total_copies {
        return Err(AppError::Validation("No copies available for loan".to_string()));
    }

    let now = chrono::Utc::now();
    let due_date = now + chrono::Duration::days(14);

    let loan = state.create_loan(CreateLoan {
        member_id: user.member.member_id,
        book_id: body.book_id,
        loan_date: now.to_rfc3339(),
        due_date: due_date.to_rfc3339(),
        return_date: None,
    }).await?;

    Ok((StatusCode::CREATED, Json(loan)))
}

pub async fn return_loan_handler(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let loan = state.return_loan(id).await?;
    
    match loan {
        Some(l) => Ok(Json(l)),
        None => Err(AppError::NotFound(format!("Loan with id {} not found", id))),
    }
}

pub async fn get_my_loans(
    Extension(state): Extension<Arc<AppState>>,
    Extension(user): Extension<AuthUser>,
) -> Result<impl IntoResponse, AppError> {
    let loans = state.get_loans_with_books_by_member(user.member.member_id).await?;
    Ok(Json(loans))
}

pub async fn create_loan(
    Extension(state): Extension<Arc<AppState>>,
    Json(body): Json<CreateLoanDto>,
) -> Result<impl IntoResponse, AppError> {
    let loan = state.create_loan(CreateLoan {
        member_id: body.member_id,
        book_id: body.book_id,
        loan_date: body.loan_date,
        due_date: body.due_date,
        return_date: body.return_date,
    }).await?;

    Ok((StatusCode::CREATED, Json(loan)))
}

pub async fn update_loan(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateLoanDto>,
) -> Result<impl IntoResponse, AppError> {
    let updated = state.update_loan(id, UpdateLoan {
        member_id: body.member_id,
        book_id: body.book_id,
        loan_date: body.loan_date,
        due_date: body.due_date,
        return_date: body.return_date,
    }).await?;

    match updated {
        Some(loan) => Ok(Json(loan)),
        None => Err(AppError::NotFound(format!("Loan with id {} not found", id))),
    }
}

pub async fn delete_loan(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    state.delete_loan(id).await?;
    
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_loan_by_id(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let loan = state.get_loan_by_id(id).await?;
    
    match loan {
        Some(l) => Ok(Json(l)),
        None => Err(AppError::NotFound(format!("Loan with id {} not found", id))),
    }
}

pub async fn get_all_loans(
    Extension(state): Extension<Arc<AppState>>,
    Query(query): Query<LoanQueryDto>,
) -> Result<impl IntoResponse, AppError> {
    let loans = state.query_loans(LoanQuery {
        member_id: query.member_id,
        book_id: query.book_id,
        loan_date: query.loan_date,
        due_date: query.due_date,
        return_date: query.return_date,
    }).await?;
    
    Ok(Json(loans))
}
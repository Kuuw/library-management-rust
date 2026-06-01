use std::sync::Arc;

use axum::{Extension, Json, extract::{Path, Query}, http::StatusCode, response::IntoResponse};

use crate::{domain::models::book::{CreateBook, UpdateBook, BookQuery}, errors::AppError, infrastructure::routes::book::dtos::{CreateBookDto, UpdateBookDto, BookQueryDto, BookResponseDto}, state::AppState};
use crate::domain::repositories::book_repository::BookRepository;
use crate::domain::repositories::loan_repository::LoanRepository;


pub async fn create_book(
    Extension(state): Extension<Arc<AppState>>,
    Json(body): Json<CreateBookDto>,
) -> Result<impl IntoResponse, AppError> {
    let book = state.create_book(CreateBook {
        category_id: body.category_id,
        book_name: body.book_name,
        book_description: body.book_description,
        isbn: body.isbn,
        publication_date: body.publication_date,
        total_copies: body.total_copies,
    }).await?;

    let active_loans = state.count_active_loans(book.book_id).await?;
    let response = BookResponseDto::from_domain(book, active_loans);

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn update_book(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateBookDto>,
) -> Result<impl IntoResponse, AppError> {
    let updated = state.update_book(id, UpdateBook {
        category_id: body.category_id,
        book_name: body.book_name,
        book_description: body.book_description,
        isbn: body.isbn,
        publication_date: body.publication_date,
        total_copies: body.total_copies,
    }).await?;

    match updated {
        Some(book) => {
            let active_loans = state.count_active_loans(book.book_id).await?;
            let response = BookResponseDto::from_domain(book, active_loans);
            Ok(Json(response))
        },
        None => Err(AppError::NotFound(format!("Book with id {} not found", id))),
    }
}

pub async fn delete_book(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    state.delete_book(id).await?;
    
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_book_by_id(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let book = state.get_book_by_id(id).await?;
    
    match book {
        Some(b) => {
            let active_loans = state.count_active_loans(b.book_id).await?;
            let response = BookResponseDto::from_domain(b, active_loans);
            Ok(Json(response))
        },
        None => Err(AppError::NotFound(format!("Book with id {} not found", id))),
    }
}

pub async fn get_all_books(
    Extension(state): Extension<Arc<AppState>>,
    Query(query): Query<BookQueryDto>,
) -> Result<impl IntoResponse, AppError> {
    let books = state.query_books(BookQuery {
        category_id: query.category_id,
        book_name: query.book_name,
        isbn: query.isbn,
        publication_date: query.publication_date,
    }).await?;
    
    let mut responses = Vec::new();
    for book in books {
        let active_loans = state.count_active_loans(book.book_id).await?;
        responses.push(BookResponseDto::from_domain(book, active_loans));
    }
    
    Ok(Json(responses))
}
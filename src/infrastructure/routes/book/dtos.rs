use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBookDto {
    pub category_id: i64,
    pub book_name: String,
    pub book_description: Option<String>,
    pub isbn: Option<String>,
    pub publication_date: Option<String>,
    pub total_copies: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBookDto {
    pub category_id: Option<i64>,
    pub book_name: Option<String>,
    pub book_description: Option<String>,
    pub isbn: Option<String>,
    pub publication_date: Option<String>,
    pub total_copies: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookQueryDto {
    pub category_id: Option<i64>,
    pub book_name: Option<String>,
    pub isbn: Option<String>,
    pub publication_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookResponseDto {
    pub book_id: i64,
    pub category_id: i64,
    pub book_name: String,
    pub book_description: Option<String>,
    pub isbn: Option<String>,
    pub publication_date: Option<String>,
    pub total_copies: Option<i64>,
    pub available_copies: i64,
}

impl BookResponseDto {
    pub fn from_domain(book: crate::domain::models::book::Book, active_loans: i64) -> Self {
        Self {
            book_id: book.book_id,
            category_id: book.category_id,
            book_name: book.book_name,
            book_description: book.book_description,
            isbn: book.isbn,
            publication_date: book.publication_date,
            total_copies: book.total_copies,
            available_copies: book.total_copies.unwrap_or(0) - active_loans,
        }
    }
}
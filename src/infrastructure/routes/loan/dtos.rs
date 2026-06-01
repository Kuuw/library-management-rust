use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLoanDto {
    pub member_id: i64,
    pub book_id: i64,
    pub loan_date: String,
    pub due_date: String,
    pub return_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateLoanDto {
    pub member_id: Option<i64>,
    pub book_id: Option<i64>,
    pub loan_date: Option<String>,
    pub due_date: Option<String>,
    pub return_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoanQueryDto {
    pub member_id: Option<i64>,
    pub book_id: Option<i64>,
    pub loan_date: Option<String>,
    pub due_date: Option<String>,
    pub return_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoanBookDto {
    pub book_id: i64,
}
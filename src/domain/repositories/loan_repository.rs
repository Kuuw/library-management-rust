use async_trait::async_trait;
use crate::domain::models::loan::*;

#[async_trait]
pub trait LoanRepository {
    async fn get_loan_by_id(&self, id: i32) -> anyhow::Result<Option<Loan>>;
    async fn create_loan(&self, loan: CreateLoan) -> anyhow::Result<Loan>;
    async fn update_loan(&self, id: i32, loan: UpdateLoan) -> anyhow::Result<Option<Loan>>;
    async fn delete_loan(&self, id: i32) -> anyhow::Result<()>;
    async fn query_loans(&self, query: LoanQuery) -> anyhow::Result<Vec<Loan>>;
    async fn count_active_loans(&self, book_id: i64) -> anyhow::Result<i64>;
    async fn return_loan(&self, id: i32) -> anyhow::Result<Option<Loan>>;
    async fn get_loans_with_books_by_member(&self, member_id: i64) -> anyhow::Result<Vec<LoanWithBook>>;
}
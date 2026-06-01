use async_trait::async_trait;
use crate::domain::models::loan::*;
use crate::domain::repositories::loan_repository::LoanRepository;
use crate::state::AppState;

#[async_trait]
impl LoanRepository for AppState {
    async fn get_loan_by_id(&self, id: i32) -> anyhow::Result<Option<Loan>> {
        let loan = sqlx::query_as!(
            Loan,
            r#"
            SELECT loan_id, member_id, book_id, loan_date, due_date, return_date
            FROM Loan
            WHERE loan_id = ?
            "#,
            id
        )
        .fetch_optional(&self.db)
        .await?;
        Ok(loan)
    }

    async fn create_loan(&self, loan: CreateLoan) -> anyhow::Result<Loan> {
        let result = sqlx::query!(
            r#"
            INSERT INTO Loan (member_id, book_id, loan_date, due_date, return_date)
            VALUES (?, ?, ?, ?, ?)
            "#,
            loan.member_id,
            loan.book_id,
            loan.loan_date,
            loan.due_date,
            loan.return_date
        )
        .execute(&self.db)
        .await?;

        let id = result.last_insert_rowid();

        Ok(Loan {
            loan_id: id,
            member_id: loan.member_id,
            book_id: loan.book_id,
            loan_date: loan.loan_date,
            due_date: loan.due_date,
            return_date: loan.return_date,
        })
    }

    async fn update_loan(&self, id: i32, loan: UpdateLoan) -> anyhow::Result<Option<Loan>> {
        let mut existing = self.get_loan_by_id(id).await?;

        if let Some(ref mut existing_loan) = existing {
            if let Some(member_id) = loan.member_id {
                existing_loan.member_id = member_id;
            }
            if let Some(book_id) = loan.book_id {
                existing_loan.book_id = book_id;
            }
            if let Some(loan_date) = loan.loan_date {
                existing_loan.loan_date = loan_date;
            }
            if let Some(due_date) = loan.due_date {
                existing_loan.due_date = due_date;
            }
            if loan.return_date.is_some() {
                existing_loan.return_date = loan.return_date;
            }

            sqlx::query!(
                r#"
                UPDATE Loan
                SET member_id = ?, book_id = ?, loan_date = ?, due_date = ?, return_date = ?
                WHERE loan_id = ?
                "#,
                existing_loan.member_id,
                existing_loan.book_id,
                existing_loan.loan_date,
                existing_loan.due_date,
                existing_loan.return_date,
                id
            )
            .execute(&self.db)
            .await?;
        }

        Ok(existing)
    }

    async fn delete_loan(&self, id: i32) -> anyhow::Result<()> {
        sqlx::query!(
            "DELETE FROM Loan WHERE loan_id = ?",
            id
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    async fn query_loans(&self, query: LoanQuery) -> anyhow::Result<Vec<Loan>> {
        let loans = sqlx::query_as!(
            Loan,
            r#"
            SELECT loan_id, member_id, book_id, loan_date, due_date, return_date
            FROM Loan
            WHERE (? IS NULL OR member_id = ?)
              AND (? IS NULL OR book_id = ?)
              AND (? IS NULL OR loan_date = ?)
              AND (? IS NULL OR due_date = ?)
              AND (? IS NULL OR return_date = ?)
            "#,
            query.member_id, query.member_id,
            query.book_id, query.book_id,
            query.loan_date, query.loan_date,
            query.due_date, query.due_date,
            query.return_date, query.return_date
        )
        .fetch_all(&self.db)
        .await?;

        Ok(loans)
    }

    async fn count_active_loans(&self, book_id: i64) -> anyhow::Result<i64> {
        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM Loan
            WHERE book_id = ? AND return_date IS NULL
            "#,
            book_id
        )
        .fetch_one(&self.db)
        .await?
        .count;
        
        Ok(count as i64)
    }

    async fn return_loan(&self, id: i32) -> anyhow::Result<Option<Loan>> {
        let now = chrono::Utc::now().to_rfc3339();
        
        sqlx::query!(
            r#"
            UPDATE Loan
            SET return_date = ?
            WHERE loan_id = ?
            "#,
            now,
            id
        )
        .execute(&self.db)
        .await?;

        self.get_loan_by_id(id).await
    }

    async fn get_loans_with_books_by_member(&self, member_id: i64) -> anyhow::Result<Vec<LoanWithBook>> {
        let loans = sqlx::query_as!(
            LoanWithBook,
            r#"
            SELECT l.loan_id, l.member_id, l.book_id, b.book_name, l.loan_date, l.due_date, l.return_date
            FROM Loan l
            JOIN Book b ON l.book_id = b.book_id
            WHERE l.member_id = ?
            "#,
            member_id
        )
        .fetch_all(&self.db)
        .await?;
        
        Ok(loans)
    }
}

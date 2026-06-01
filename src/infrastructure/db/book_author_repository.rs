use async_trait::async_trait;
use crate::domain::models::author::Author;
use crate::domain::models::book::Book;
use crate::domain::repositories::book_author_repository::BookAuthorRepository;
use crate::state::AppState;

#[async_trait]
impl BookAuthorRepository for AppState {
    async fn add_author_to_book(&self, book_id: i64, author_id: i64) -> anyhow::Result<()> {
        sqlx::query!(
            "INSERT INTO BookAuthor (book_id, author_id) VALUES (?, ?)",
            book_id,
            author_id
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    async fn remove_author_from_book(&self, book_id: i64, author_id: i64) -> anyhow::Result<()> {
        sqlx::query!(
            "DELETE FROM BookAuthor WHERE book_id = ? AND author_id = ?",
            book_id,
            author_id
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    async fn get_authors_by_book_id(&self, book_id: i64) -> anyhow::Result<Vec<Author>> {
        let authors = sqlx::query_as!(
            Author,
            r#"
            SELECT 
                a.author_id as "author_id!", 
                a.first_name as "first_name!", 
                a.last_name as "last_name!"
            FROM Author a
            JOIN BookAuthor ba ON a.author_id = ba.author_id
            WHERE ba.book_id = ?
            "#,
            book_id
        )
        .fetch_all(&self.db)
        .await?;

        Ok(authors)
    }

    async fn get_books_by_author_id(&self, author_id: i64) -> anyhow::Result<Vec<Book>> {
        let books = sqlx::query_as!(
            Book,
            r#"
            SELECT b.book_id, b.category_id, b.book_name, b.book_description, b.isbn, b.publication_date, b.total_copies
            FROM Book b
            JOIN BookAuthor ba ON b.book_id = ba.book_id
            WHERE ba.author_id = ?
            "#,
            author_id
        )
        .fetch_all(&self.db)
        .await?;
        Ok(books)
    }
}

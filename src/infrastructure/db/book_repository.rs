use async_trait::async_trait;
use crate::domain::models::book::*;
use crate::domain::repositories::book_repository::BookRepository;
use crate::state::AppState;

#[async_trait]
impl BookRepository for AppState {
    async fn get_book_by_id(&self, id: i32) -> anyhow::Result<Option<Book>> {
        let book = sqlx::query_as!(
            Book,
            r#"
            SELECT book_id, category_id, book_name, book_description, isbn, publication_date, total_copies
            FROM Book
            WHERE book_id = ?
            "#,
            id
        )
        .fetch_optional(&self.db)
        .await?;
        Ok(book)
    }

    async fn create_book(&self, book: CreateBook) -> anyhow::Result<Book> {
        let result = sqlx::query!(
            r#"
            INSERT INTO Book (category_id, book_name, book_description, isbn, publication_date, total_copies)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            book.category_id,
            book.book_name,
            book.book_description,
            book.isbn,
            book.publication_date,
            book.total_copies
        )
        .execute(&self.db)
        .await?;

        let id = result.last_insert_rowid();

        Ok(Book {
            book_id: id,
            category_id: book.category_id,
            book_name: book.book_name,
            book_description: book.book_description,
            isbn: book.isbn,
            publication_date: book.publication_date,
            total_copies: book.total_copies,
        })
    }

    async fn update_book(&self, id: i32, book: UpdateBook) -> anyhow::Result<Option<Book>> {
        let mut existing = self.get_book_by_id(id).await?;

        if let Some(ref mut existing_book) = existing {
            if let Some(category_id) = book.category_id {
                existing_book.category_id = category_id;
            }
            if let Some(book_name) = book.book_name {
                existing_book.book_name = book_name;
            }
            if book.book_description.is_some() {
                existing_book.book_description = book.book_description;
            }
            if book.isbn.is_some() {
                existing_book.isbn = book.isbn;
            }
            if book.publication_date.is_some() {
                existing_book.publication_date = book.publication_date;
            }
            if book.total_copies.is_some() {
                existing_book.total_copies = book.total_copies;
            }

            sqlx::query!(
                r#"
                UPDATE Book
                SET category_id = ?, book_name = ?, book_description = ?, isbn = ?, publication_date = ?, total_copies = ?
                WHERE book_id = ?
                "#,
                existing_book.category_id,
                existing_book.book_name,
                existing_book.book_description,
                existing_book.isbn,
                existing_book.publication_date,
                existing_book.total_copies,
                id
            )
            .execute(&self.db)
            .await?;
        }

        Ok(existing)
    }

    async fn delete_book(&self, id: i32) -> anyhow::Result<()> {
        sqlx::query!(
            "DELETE FROM Book WHERE book_id = ?",
            id
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    async fn query_books(&self, query: BookQuery) -> anyhow::Result<Vec<Book>> {
        let book_name = query.book_name.map(|s| format!("%{}%", s));
        let isbn = query.isbn.map(|s| format!("%{}%", s));

        let books = sqlx::query_as!(
            Book,
            r#"
            SELECT book_id, category_id, book_name, book_description, isbn, publication_date, total_copies
            FROM Book
            WHERE (? IS NULL OR category_id = ?)
              AND (? IS NULL OR book_name LIKE ?)
              AND (? IS NULL OR isbn LIKE ?)
              AND (? IS NULL OR publication_date = ?)
            "#,
            query.category_id, query.category_id,
            book_name, book_name,
            isbn, isbn,
            query.publication_date, query.publication_date
        )
        .fetch_all(&self.db)
        .await?;

        Ok(books)
    }
}

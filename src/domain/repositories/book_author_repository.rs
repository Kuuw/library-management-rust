use async_trait::async_trait;
use crate::domain::models::author::Author;
use crate::domain::models::book::Book;

#[async_trait]
pub trait BookAuthorRepository {
    async fn add_author_to_book(&self, book_id: i64, author_id: i64) -> anyhow::Result<()>;
    async fn remove_author_from_book(&self, book_id: i64, author_id: i64) -> anyhow::Result<()>;
    async fn get_authors_by_book_id(&self, book_id: i64) -> anyhow::Result<Vec<Author>>;
    async fn get_books_by_author_id(&self, author_id: i64) -> anyhow::Result<Vec<Book>>;
}

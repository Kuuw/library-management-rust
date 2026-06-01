use async_trait::async_trait;
use crate::domain::models::book::*;

#[async_trait]
pub trait BookRepository {
    async fn get_book_by_id(&self, id: i32) -> anyhow::Result<Option<Book>>;
    async fn create_book(&self, book: CreateBook) -> anyhow::Result<Book>;
    async fn update_book(&self, id: i32, book: UpdateBook) -> anyhow::Result<Option<Book>>;
    async fn delete_book(&self, id: i32) -> anyhow::Result<()>;
    async fn query_books(&self, query: BookQuery) -> anyhow::Result<Vec<Book>>;
}
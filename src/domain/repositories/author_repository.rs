use async_trait::async_trait;
use crate::domain::models::author::*;

#[async_trait]
pub trait AuthorRepository {
    async fn get_author_by_id(&self, id: i32) -> anyhow::Result<Option<Author>>;
    async fn create_author(&self, author: CreateAuthor) -> anyhow::Result<Author>;
    async fn update_author(&self, id: i32, author: UpdateAuthor) -> anyhow::Result<Option<Author>>;
    async fn delete_author(&self, id: i32) -> anyhow::Result<()>;
    async fn query_authors(&self, query: AuthorQuery) -> anyhow::Result<Vec<Author>>;
}
use async_trait::async_trait;
use crate::domain::models::category::*;

#[async_trait]
pub trait CategoryRepository {
    async fn get_category_by_id(&self, id: i32) -> anyhow::Result<Option<Category>>;
    async fn create_category(&self, category: CreateCategory) -> anyhow::Result<Category>;
    async fn update_category(&self, id: i32, category: UpdateCategory) -> anyhow::Result<Option<Category>>;
    async fn delete_category(&self, id: i32) -> anyhow::Result<()>;
    async fn query_categories(&self, query: CategoryQuery) -> anyhow::Result<Vec<Category>>;
}
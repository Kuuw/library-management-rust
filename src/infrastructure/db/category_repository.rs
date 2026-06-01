use async_trait::async_trait;
use crate::domain::models::category::*;
use crate::domain::repositories::category_repository::CategoryRepository;
use crate::state::AppState;

#[async_trait]
impl CategoryRepository for AppState {
    async fn get_category_by_id(&self, id: i32) -> anyhow::Result<Option<Category>> {
        let category = sqlx::query_as!(
            Category,
            r#"
            SELECT category_id, name, description
            FROM Category
            WHERE category_id = ?
            "#,
            id
        )
        .fetch_optional(&self.db)
        .await?;
        Ok(category)
    }

    async fn create_category(&self, category: CreateCategory) -> anyhow::Result<Category> {
        let result = sqlx::query!(
            r#"
            INSERT INTO Category (name, description)
            VALUES (?, ?)
            "#,
            category.name,
            category.description
        )
        .execute(&self.db)
        .await?;

        let id = result.last_insert_rowid();

        Ok(Category {
            category_id: id,
            name: category.name,
            description: category.description,
        })
    }

    async fn update_category(&self, id: i32, category: UpdateCategory) -> anyhow::Result<Option<Category>> {
        let mut existing = self.get_category_by_id(id).await?;

        if let Some(ref mut existing_category) = existing {
            if let Some(name) = category.name {
                existing_category.name = name;
            }
            if category.description.is_some() {
                existing_category.description = category.description;
            }

            sqlx::query!(
                r#"
                UPDATE Category
                SET name = ?, description = ?
                WHERE category_id = ?
                "#,
                existing_category.name,
                existing_category.description,
                id
            )
            .execute(&self.db)
            .await?;
        }

        Ok(existing)
    }

    async fn delete_category(&self, id: i32) -> anyhow::Result<()> {
        sqlx::query!(
            "DELETE FROM Category WHERE category_id = ?",
            id
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    async fn query_categories(&self, query: CategoryQuery) -> anyhow::Result<Vec<Category>> {
        let name = query.name.map(|s| format!("%{}%", s));

        let categories = sqlx::query_as!(
            Category,
            r#"
            SELECT category_id, name, description
            FROM Category
            WHERE (? IS NULL OR name LIKE ?)
            "#,
            name, name
        )
        .fetch_all(&self.db)
        .await?;

        Ok(categories)
    }
}

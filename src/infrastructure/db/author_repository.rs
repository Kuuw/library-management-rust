use async_trait::async_trait;
use crate::domain::models::author::*;
use crate::domain::repositories::author_repository::AuthorRepository;
use crate::state::AppState;

#[async_trait]
impl AuthorRepository for AppState {
    async fn get_author_by_id(&self, id: i32) -> anyhow::Result<Option<Author>> {
        let author = sqlx::query_as!(
            Author,
            r#"
            SELECT author_id, first_name, last_name
            FROM Author
            WHERE author_id = ?
            "#,
            id
        )
        .fetch_optional(&self.db)
        .await?;
        Ok(author)
    }

    async fn create_author(&self, author: CreateAuthor) -> anyhow::Result<Author> {
        let result = sqlx::query!(
            r#"
            INSERT INTO Author (first_name, last_name)
            VALUES (?, ?)
            "#,
            author.first_name,
            author.last_name
        )
        .execute(&self.db)
        .await?;

        let id = result.last_insert_rowid();
        
        Ok(Author {
            author_id: id,
            first_name: author.first_name,
            last_name: author.last_name,
        })
    }

    async fn update_author(&self, id: i32, author: UpdateAuthor) -> anyhow::Result<Option<Author>> {
        let mut existing = self.get_author_by_id(id).await?;
        
        if let Some(ref mut existing_author) = existing {
            if let Some(first_name) = author.first_name {
                existing_author.first_name = first_name;
            }
            if let Some(last_name) = author.last_name {
                existing_author.last_name = last_name;
            }

            sqlx::query!(
                r#"
                UPDATE Author
                SET first_name = ?, last_name = ?
                WHERE author_id = ?
                "#,
                existing_author.first_name,
                existing_author.last_name,
                id
            )
            .execute(&self.db)
            .await?;
        }

        Ok(existing)
    }

    async fn delete_author(&self, id: i32) -> anyhow::Result<()> {
        sqlx::query!(
            "DELETE FROM Author WHERE author_id = ?",
            id
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    async fn query_authors(&self, query: AuthorQuery) -> anyhow::Result<Vec<Author>> {
        let first_name = query.first_name.map(|s| format!("%{}%", s));
        let last_name = query.last_name.map(|s| format!("%{}%", s));

        let authors = sqlx::query_as!(
            Author,
            r#"
            SELECT author_id, first_name, last_name
            FROM Author
            WHERE (? IS NULL OR first_name LIKE ?)
              AND (? IS NULL OR last_name LIKE ?)
            "#,
            first_name, first_name,
            last_name, last_name
        )
        .fetch_all(&self.db)
        .await?;

        Ok(authors)
    }
}

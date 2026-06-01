#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct BookAuthor {
    pub book_id: i64,
    pub author_id: i64,
}
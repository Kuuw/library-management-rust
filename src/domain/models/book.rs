#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Book {
    pub book_id: i64,
    pub category_id: i64,
    pub book_name: String,
    pub book_description: Option<String>,
    pub isbn: Option<String>,
    pub publication_date: Option<String>,
    pub total_copies: Option<i64>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateBook {
    pub category_id: i64,
    pub book_name: String,
    pub book_description: Option<String>,
    pub isbn: Option<String>,
    pub publication_date: Option<String>,
    pub total_copies: Option<i64>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateBook {
    pub category_id: Option<i64>,
    pub book_name: Option<String>,
    pub book_description: Option<String>,
    pub isbn: Option<String>,
    pub publication_date: Option<String>,
    pub total_copies: Option<i64>,
}

#[derive(Debug, serde::Deserialize)]
pub struct BookQuery {
    pub category_id: Option<i64>,
    pub book_name: Option<String>,
    pub isbn: Option<String>,
    pub publication_date: Option<String>,
}
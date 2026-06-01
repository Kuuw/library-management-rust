#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Category {
    pub category_id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CategoryQuery {
    pub name: Option<String>,
}
#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Author {
    pub author_id: i64,
    pub first_name: String,
    pub last_name: String,
}

pub struct CreateAuthor {
    pub first_name: String,
    pub last_name: String,
}

pub struct UpdateAuthor {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

pub struct AuthorQuery {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize, Clone)]
pub struct Member {
    pub member_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub membership_date: Option<String>,
    pub membership_status: i64,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct CreateMember {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub membership_date: Option<String>,
    pub membership_status: i64,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct UpdateMember {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub membership_date: Option<String>,
    pub membership_status: Option<i64>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct MemberQuery {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub membership_status: Option<i64>,
}

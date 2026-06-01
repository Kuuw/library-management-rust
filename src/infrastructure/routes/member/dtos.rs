use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMemberDto {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub membership_date: Option<String>,
    pub membership_status: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMemberDto {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub membership_date: Option<String>,
    pub membership_status: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberQueryDto {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub membership_status: Option<i64>,
}
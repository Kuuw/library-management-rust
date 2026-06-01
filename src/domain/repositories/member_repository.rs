use async_trait::async_trait;
use crate::domain::models::member::*;

#[async_trait]
pub trait MemberRepository {
    async fn get_member_by_id(&self, id: i32) -> anyhow::Result<Option<Member>>;
    async fn get_member_by_email(&self, email: &str) -> anyhow::Result<Option<Member>>;
    async fn create_member(&self, member: CreateMember) -> anyhow::Result<Member>;
    async fn update_member(&self, id: i32, member: UpdateMember) -> anyhow::Result<Option<Member>>;
    async fn delete_member(&self, id: i32) -> anyhow::Result<()>;
    async fn query_members(&self, query: MemberQuery) -> anyhow::Result<Vec<Member>>;
}
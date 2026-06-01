use async_trait::async_trait;
use crate::domain::models::member::*;
use crate::domain::repositories::member_repository::MemberRepository;
use crate::state::AppState;

#[async_trait]
impl MemberRepository for AppState {
    async fn get_member_by_id(&self, id: i32) -> anyhow::Result<Option<Member>> {
        let member = sqlx::query_as!(
            Member,
            r#"
            SELECT member_id, first_name, last_name, email, password_hash, phone, address, membership_date, membership_status
            FROM Member
            WHERE member_id = ?
            "#,
            id
        )
        .fetch_optional(&self.db)
        .await?;
        Ok(member)
    }

    async fn get_member_by_email(&self, email: &str) -> anyhow::Result<Option<Member>> {
        let member = sqlx::query_as!(
            Member,
            r#"
            SELECT member_id, first_name, last_name, email, password_hash, phone, address, membership_date, membership_status
            FROM Member
            WHERE email = ?
            "#,
            email
        )
        .fetch_optional(&self.db)
        .await?;
        Ok(member)
    }

    async fn create_member(&self, member: CreateMember) -> anyhow::Result<Member> {
        let result = sqlx::query!(
            r#"
            INSERT INTO Member (first_name, last_name, email, password_hash, phone, address, membership_date, membership_status)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            member.first_name,
            member.last_name,
            member.email,
            member.password_hash,
            member.phone,
            member.address,
            member.membership_date,
            member.membership_status
        )
        .execute(&self.db)
        .await?;

        let id = result.last_insert_rowid();

        Ok(Member {
            member_id: id,
            first_name: member.first_name,
            last_name: member.last_name,
            email: member.email,
            password_hash: member.password_hash,
            phone: member.phone,
            address: member.address,
            membership_date: member.membership_date,
            membership_status: member.membership_status,
        })
    }

    async fn update_member(&self, id: i32, member: UpdateMember) -> anyhow::Result<Option<Member>> {
        let mut existing = self.get_member_by_id(id).await?;

        if let Some(ref mut existing_member) = existing {
            if let Some(first_name) = member.first_name {
                existing_member.first_name = first_name;
            }
            if let Some(last_name) = member.last_name {
                existing_member.last_name = last_name;
            }
            if let Some(email) = member.email {
                existing_member.email = email;
            }
            if let Some(password_hash) = member.password_hash {
                existing_member.password_hash = password_hash;
            }
            if member.phone.is_some() {
                existing_member.phone = member.phone;
            }
            if member.address.is_some() {
                existing_member.address = member.address;
            }
            if member.membership_date.is_some() {
                existing_member.membership_date = member.membership_date;
            }
            if let Some(membership_status) = member.membership_status {
                existing_member.membership_status = membership_status;
            }

            sqlx::query!(
                r#"
                UPDATE Member
                SET first_name = ?, last_name = ?, email = ?, password_hash = ?, phone = ?, address = ?, membership_date = ?, membership_status = ?
                WHERE member_id = ?
                "#,
                existing_member.first_name,
                existing_member.last_name,
                existing_member.email,
                existing_member.password_hash,
                existing_member.phone,
                existing_member.address,
                existing_member.membership_date,
                existing_member.membership_status,
                id
            )
            .execute(&self.db)
            .await?;
        }

        Ok(existing)
    }

    async fn delete_member(&self, id: i32) -> anyhow::Result<()> {
        sqlx::query!(
            "DELETE FROM Member WHERE member_id = ?",
            id
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    async fn query_members(&self, query: MemberQuery) -> anyhow::Result<Vec<Member>> {
        let first_name = query.first_name.map(|s| format!("%{}%", s));
        let last_name = query.last_name.map(|s| format!("%{}%", s));
        let email = query.email.map(|s| format!("%{}%", s));

        let members = sqlx::query_as!(
            Member,
            r#"
            SELECT member_id, first_name, last_name, email, password_hash, phone, address, membership_date, membership_status
            FROM Member
            WHERE (? IS NULL OR first_name LIKE ?)
              AND (? IS NULL OR last_name LIKE ?)
              AND (? IS NULL OR email LIKE ?)
              AND (? IS NULL OR membership_status = ?)
            "#,
            first_name, first_name,
            last_name, last_name,
            email, email,
            query.membership_status, query.membership_status
        )
        .fetch_all(&self.db)
        .await?;

        Ok(members)
    }
}

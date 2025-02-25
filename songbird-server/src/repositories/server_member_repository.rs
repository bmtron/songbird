use sqlx::{Pool, Postgres};
use chrono::{DateTime, Utc};
use crate::models::models::{ServerMember, NewServerMember};

pub struct ServerMemberRepository {
    pool: Pool<Postgres>,
}

impl ServerMemberRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, new_server_member: NewServerMember) -> Result<ServerMember, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            INSERT INTO server_members (server_id, user_id, nickname)
            VALUES ($1, $2, $3)
            RETURNING server_id, user_id, nickname, joined_at
            "#,
            new_server_member.server_id,
            new_server_member.user_id,
            new_server_member.nickname
        )
        .fetch_one(&self.pool)
        .await?;

        let server_member = ServerMember {
            server_id: record.server_id,
            user_id: record.user_id,
            nickname: record.nickname,
            joined_at: DateTime::from_naive_utc_and_offset(record.joined_at, Utc)
        };

        Ok(server_member)
    }

    pub async fn find_by_id(&self, server_id: i32, user_id: i32) -> Result<Option<ServerMember>, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            SELECT server_id, user_id, nickname, joined_at
            FROM server_members
            WHERE server_id = $1 AND user_id = $2
            "#,
            server_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        let server_member = record.map(|r| ServerMember {
            server_id: r.server_id,
            user_id: r.user_id,
            nickname: r.nickname,
            joined_at: DateTime::from_naive_utc_and_offset(r.joined_at, Utc)
        });

        Ok(server_member)
    }

    pub async fn find_by_server(&self, server_id: i32) -> Result<Vec<ServerMember>, sqlx::Error> {
        let records = sqlx::query!(
            r#"
            SELECT server_id, user_id, nickname, joined_at
            FROM server_members
            WHERE server_id = $1
            "#,
            server_id
        )
        .fetch_all(&self.pool)
        .await?;

        let server_members = records
            .into_iter()
            .map(|r| ServerMember {
                server_id: r.server_id,
                user_id: r.user_id,
                nickname: r.nickname,
                joined_at: DateTime::from_naive_utc_and_offset(r.joined_at, Utc)
            })
            .collect();

        Ok(server_members)
    }

    pub async fn find_by_user(&self, user_id: i32) -> Result<Vec<ServerMember>, sqlx::Error> {
        let records = sqlx::query!(
            r#"
            SELECT server_id, user_id, nickname, joined_at
            FROM server_members
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let server_members = records
            .into_iter()
            .map(|r| ServerMember {
                server_id: r.server_id,
                user_id: r.user_id,
                nickname: r.nickname,
                joined_at: DateTime::from_naive_utc_and_offset(r.joined_at, Utc)
            })
            .collect();

        Ok(server_members)
    }

    pub async fn update_nickname(&self, server_id: i32, user_id: i32, nickname: Option<String>) -> Result<ServerMember, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            UPDATE server_members
            SET nickname = $1
            WHERE server_id = $2 AND user_id = $3
            RETURNING server_id, user_id, nickname, joined_at
            "#,
            nickname,
            server_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        let updated_server_member = ServerMember {
            server_id: record.server_id,
            user_id: record.user_id,
            nickname: record.nickname,
            joined_at: DateTime::from_naive_utc_and_offset(record.joined_at, Utc)
        };

        Ok(updated_server_member)
    }

    pub async fn delete(&self, server_id: i32, user_id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM server_members
            WHERE server_id = $1 AND user_id = $2
            "#,
            server_id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn is_member(&self, server_id: i32, user_id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT 1 as exists
            FROM server_members
            WHERE server_id = $1 AND user_id = $2
            "#,
            server_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.is_some())
    }

    pub async fn count_members(&self, server_id: i32) -> Result<i64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM server_members
            WHERE server_id = $1
            "#,
            server_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.count.unwrap_or(0))
    }
}
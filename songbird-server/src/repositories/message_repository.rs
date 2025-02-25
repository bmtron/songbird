use sqlx::{Pool, Postgres};
use chrono::{DateTime, Utc};
use crate::models::models::{Message, NewMessage, MessageWithAuthorResponse, UserResponse};

pub struct MessageRepository {
    pool: Pool<Postgres>,
}

impl MessageRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, new_message: NewMessage) -> Result<Message, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            INSERT INTO messages (channel_id, author_user_id, content)
            VALUES ($1, $2, $3)
            RETURNING message_id, channel_id, author_user_id, content, created_at, updated_at, edited_at
            "#,
            new_message.channel_id,
            new_message.author_user_id,
            new_message.content
        )
        .fetch_one(&self.pool)
        .await?;

        let message = Message {
            message_id: record.message_id,
            channel_id: record.channel_id,
            author_user_id: record.author_user_id,
            content: record.content,
            created_at: DateTime::from_naive_utc_and_offset(record.created_at, Utc),
            updated_at: record.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            edited_at: record.edited_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
        };

        Ok(message)
    }

    pub async fn find_by_id(&self, message_id: i32) -> Result<Option<Message>, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            SELECT message_id, channel_id, author_user_id, content, created_at, updated_at, edited_at
            FROM messages
            WHERE message_id = $1
            "#,
            message_id
        )
        .fetch_optional(&self.pool)
        .await?;

        let message = record.map(|r| Message {
            message_id: r.message_id,
            channel_id: r.channel_id,
            author_user_id: r.author_user_id,
            content: r.content,
            created_at: DateTime::from_naive_utc_and_offset(r.created_at, Utc),
            updated_at: r.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            edited_at: r.edited_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
        });

        Ok(message)
    }

    pub async fn find_by_channel(&self, channel_id: i32, limit: i64) -> Result<Vec<Message>, sqlx::Error> {
        let records = sqlx::query!(
            r#"
            SELECT message_id, channel_id, author_user_id, content, created_at, updated_at, edited_at
            FROM messages
            WHERE channel_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
            channel_id,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        let messages = records
            .into_iter()
            .map(|r| Message {
                message_id: r.message_id,
                channel_id: r.channel_id,
                author_user_id: r.author_user_id,
                content: r.content,
                created_at: DateTime::from_naive_utc_and_offset(r.created_at, Utc),
                updated_at: r.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
                edited_at: r.edited_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
            })
            .collect();

        Ok(messages)
    }

    pub async fn find_by_channel_with_authors(&self, channel_id: i32, limit: i64) -> Result<Vec<MessageWithAuthorResponse>, sqlx::Error> {
        let records = sqlx::query!(
            r#"
            SELECT 
                m.message_id, m.content, m.created_at, m.edited_at,
                u.user_id, u.username, u.email, u.avatar_url, u.created_at as user_created_at, u.status
            FROM messages m
            JOIN users u ON m.author_user_id = u.user_id
            WHERE m.channel_id = $1
            ORDER BY m.created_at DESC
            LIMIT $2
            "#,
            channel_id,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        let message_responses = records.into_iter().map(|r| {
            MessageWithAuthorResponse {
                message_id: r.message_id,
                content: r.content,
                created_at: DateTime::from_naive_utc_and_offset(r.created_at, Utc),
                edited_at: r.edited_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
                author: UserResponse {
                    user_id: r.user_id,
                    username: r.username,
                    email: r.email,
                    avatar_url: r.avatar_url,
                    status: r.status,
                    created_at: DateTime::from_naive_utc_and_offset(r.user_created_at, Utc),
                },
            }
        }).collect();

        Ok(message_responses)
    }

    pub async fn update_content(&self, message_id: i32, content: String) -> Result<Message, sqlx::Error> {
        let now = Utc::now();
        let record = sqlx::query!(
            r#"
            UPDATE messages
            SET content = $1, updated_at = $2, edited_at = $2
            WHERE message_id = $3
            RETURNING message_id, channel_id, author_user_id, content, created_at, updated_at, edited_at
            "#,
            content,
            now as _,
            message_id
        )
        .fetch_one(&self.pool)
        .await?;

        let updated_message = Message {
            message_id: record.message_id,
            channel_id: record.channel_id,
            author_user_id: record.author_user_id,
            content: record.content,
            created_at: DateTime::from_naive_utc_and_offset(record.created_at, Utc),
            updated_at: record.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            edited_at: record.edited_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
        };

        Ok(updated_message)
    }

    pub async fn delete(&self, message_id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM messages
            WHERE message_id = $1
            "#,
            message_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn count_by_channel(&self, channel_id: i32) -> Result<i64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM messages
            WHERE channel_id = $1
            "#,
            channel_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.count.unwrap_or(0))
    }

    pub async fn count_by_user(&self, user_id: i32) -> Result<i64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM messages
            WHERE author_user_id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.count.unwrap_or(0))
    }
}
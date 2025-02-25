use sqlx::{Pool, Postgres};
use chrono::{DateTime, Utc};
use crate::models::models::{Channel, NewChannel, ChannelWithMessagesResponse, MessageWithAuthorResponse};
use crate::repositories::MessageRepository;

pub struct ChannelRepository {
    pool: Pool<Postgres>,
    message_repository: Option<MessageRepository>,
}

impl ChannelRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool, message_repository: None }
    }

    pub fn with_message_repository(pool: Pool<Postgres>, message_repository: MessageRepository) -> Self {
        Self { pool, message_repository: Some(message_repository) }
    }

    pub async fn create(&self, new_channel: NewChannel) -> Result<Channel, sqlx::Error> {
        let now = Utc::now();
        let record = sqlx::query!(
            r#"
            INSERT INTO channels (server_id, name, type)
            VALUES ($1, $2, $3)
            RETURNING channel_id, server_id, name, type as "channel_type", created_at, updated_at
            "#,
            new_channel.server_id,
            new_channel.name,
            new_channel.channel_type
        )
        .fetch_one(&self.pool)
        .await?;

        let channel = Channel {
            channel_id: record.channel_id,
            server_id: record.server_id,
            name: record.name,
            channel_type: record.channel_type,
            created_at: DateTime::from_naive_utc_and_offset(record.created_at, Utc),
            updated_at: record.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
        };

        Ok(channel)
    }

    pub async fn find_by_id(&self, channel_id: i32) -> Result<Option<Channel>, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            SELECT channel_id, server_id, name, type as "channel_type", created_at, updated_at
            FROM channels
            WHERE channel_id = $1
            "#,
            channel_id
        )
        .fetch_optional(&self.pool)
        .await?;

        let channel = record.map(|r| Channel {
            channel_id: r.channel_id,
            server_id: r.server_id,
            name: r.name,
            channel_type: r.channel_type,
            created_at: DateTime::from_naive_utc_and_offset(r.created_at, Utc),
            updated_at: r.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
        });

        Ok(channel)
    }

    pub async fn find_by_server(&self, server_id: i32) -> Result<Vec<Channel>, sqlx::Error> {
        let records = sqlx::query!(
            r#"
            SELECT channel_id, server_id, name, type as "channel_type", created_at, updated_at
            FROM channels
            WHERE server_id = $1
            ORDER BY name
            "#,
            server_id
        )
        .fetch_all(&self.pool)
        .await?;

        let channels = records
            .into_iter()
            .map(|r| Channel {
                channel_id: r.channel_id,
                server_id: r.server_id,
                name: r.name,
                channel_type: r.channel_type,
                created_at: DateTime::from_naive_utc_and_offset(r.created_at, Utc),
                updated_at: r.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
            })
            .collect();

        Ok(channels)
    }

    pub async fn find_direct_message_channels(&self, user_id: i32) -> Result<Vec<Channel>, sqlx::Error> {
        let records = sqlx::query!(
            r#"
            SELECT c.channel_id, c.server_id, c.name, c.type as "channel_type", c.created_at, c.updated_at
            FROM channels c
            JOIN direct_message_members dm ON c.channel_id = dm.channel_id
            WHERE dm.user_id = $1 AND c.type = 'dm'
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let channels = records
            .into_iter()
            .map(|r| Channel {
                channel_id: r.channel_id,
                server_id: r.server_id,
                name: r.name,
                channel_type: r.channel_type,
                created_at: DateTime::from_naive_utc_and_offset(r.created_at, Utc),
                updated_at: r.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
            })
            .collect();

        Ok(channels)
    }

    pub async fn update(&self, channel_id: i32, name: String) -> Result<Channel, sqlx::Error> {
        let now = Utc::now();
        let record = sqlx::query!(
            r#"
            UPDATE channels
            SET name = $1, updated_at = $2
            WHERE channel_id = $3
            RETURNING channel_id, server_id, name, type as "channel_type", created_at, updated_at
            "#,
            name,
            now as _,
            channel_id
        )
        .fetch_one(&self.pool)
        .await?;

        let updated_channel = Channel {
            channel_id: record.channel_id,
            server_id: record.server_id,
            name: record.name,
            channel_type: record.channel_type,
            created_at: DateTime::from_naive_utc_and_offset(record.created_at, Utc),
            updated_at: record.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
        };

        Ok(updated_channel)
    }

    pub async fn delete(&self, channel_id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM channels
            WHERE channel_id = $1
            "#,
            channel_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_channel_with_messages(&self, channel_id: i32, limit: i64) -> Result<Option<ChannelWithMessagesResponse>, sqlx::Error> {
        if self.message_repository.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        let channel = self.find_by_id(channel_id).await?;
        
        if let Some(channel) = channel {
            let messages = self.message_repository.as_ref().unwrap().find_by_channel_with_authors(channel_id, limit).await?;
            Ok(Some(ChannelWithMessagesResponse {
                channel,
                messages,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn is_direct_message_member(&self, channel_id: i32, user_id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT 1 as exists
            FROM direct_message_members
            WHERE channel_id = $1 AND user_id = $2
            "#,
            channel_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.is_some())
    }

    pub async fn add_direct_message_member(&self, channel_id: i32, user_id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            INSERT INTO direct_message_members (channel_id, user_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#,
            channel_id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn remove_direct_message_member(&self, channel_id: i32, user_id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM direct_message_members
            WHERE channel_id = $1 AND user_id = $2
            "#,
            channel_id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
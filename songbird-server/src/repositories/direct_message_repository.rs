use sqlx::{Pool, Postgres, Transaction};
use chrono::{DateTime, Utc};
use crate::models::models::{DirectMessageMember, NewDirectMessageMember, Channel, NewChannel};
use crate::repositories::ChannelRepository;

pub struct DirectMessageRepository {
    pool: Pool<Postgres>,
    channel_repository: ChannelRepository,
}

impl DirectMessageRepository {
    pub fn new(pool: Pool<Postgres>, channel_repository: ChannelRepository) -> Self {
        Self { pool, channel_repository }
    }

    pub async fn create_dm_channel(&self, user_id1: i32, user_id2: i32, name: String) -> Result<Channel, sqlx::Error> {
        // Start a transaction
        let mut tx = self.pool.begin().await?;

        // Create the DM channel
        let now = Utc::now();
        let record = sqlx::query!(
            r#"
            INSERT INTO channels (server_id, name, type, created_at, updated_at)
            VALUES (NULL, $1, $2, $3, $3)
            RETURNING channel_id, server_id, name, type as "channel_type", created_at, updated_at
            "#,
            name,
            "dm",
            now as _
        )
        .fetch_one(&mut *tx)
        .await?;

        let channel = Channel {
            channel_id: record.channel_id,
            server_id: record.server_id,
            name: record.name,
            channel_type: record.channel_type,
            created_at: DateTime::from_naive_utc_and_offset(record.created_at, Utc),
            updated_at: record.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
        };

        // Add both users to the direct message channel
        self.add_dm_member_tx(&mut tx, channel.channel_id, user_id1).await?;
        self.add_dm_member_tx(&mut tx, channel.channel_id, user_id2).await?;

        // Commit the transaction
        tx.commit().await?;

        Ok(channel)
    }

    async fn add_dm_member_tx(&self, tx: &mut Transaction<'_, Postgres>, channel_id: i32, user_id: i32) -> Result<DirectMessageMember, sqlx::Error> {
        let member = sqlx::query_as!(
            DirectMessageMember,
            r#"
            INSERT INTO direct_message_members (channel_id, user_id)
            VALUES ($1, $2)
            RETURNING channel_id, user_id
            "#,
            channel_id,
            user_id
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(member)
    }

    pub async fn add_dm_member(&self, channel_id: i32, user_id: i32) -> Result<DirectMessageMember, sqlx::Error> {
        let member = sqlx::query_as!(
            DirectMessageMember,
            r#"
            INSERT INTO direct_message_members (channel_id, user_id)
            VALUES ($1, $2)
            RETURNING channel_id, user_id
            "#,
            channel_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(member)
    }

    pub async fn remove_dm_member(&self, channel_id: i32, user_id: i32) -> Result<bool, sqlx::Error> {
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

    pub async fn find_dm_members(&self, channel_id: i32) -> Result<Vec<DirectMessageMember>, sqlx::Error> {
        let members = sqlx::query_as!(
            DirectMessageMember,
            r#"
            SELECT channel_id, user_id
            FROM direct_message_members
            WHERE channel_id = $1
            "#,
            channel_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(members)
    }

    pub async fn find_dm_member(&self, channel_id: i32, user_id: i32) -> Result<Option<DirectMessageMember>, sqlx::Error> {
        let member = sqlx::query_as!(
            DirectMessageMember,
            r#"
            SELECT channel_id, user_id
            FROM direct_message_members
            WHERE channel_id = $1 AND user_id = $2
            "#,
            channel_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(member)
    }

    pub async fn find_or_create_dm_channel(&self, user_id1: i32, user_id2: i32) -> Result<Channel, sqlx::Error> {
        // First, check if a DM channel already exists between these users
        let record = sqlx::query!(
            r#"
            SELECT c.channel_id, c.server_id, c.name, c.type as "channel_type", c.created_at, c.updated_at
            FROM channels c
            JOIN direct_message_members dm1 ON c.channel_id = dm1.channel_id
            JOIN direct_message_members dm2 ON c.channel_id = dm2.channel_id
            WHERE c.type = 'dm' AND dm1.user_id = $1 AND dm2.user_id = $2
            "#,
            user_id1,
            user_id2
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(r) = record {
            let channel = Channel {
                channel_id: r.channel_id,
                server_id: r.server_id,
                name: r.name,
                channel_type: r.channel_type,
                created_at: DateTime::from_naive_utc_and_offset(r.created_at, Utc),
                updated_at: r.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
            };
            Ok(channel)
        } else {
            // Create a new DM channel
            let channel_name = format!("dm_{}_{}", user_id1, user_id2);
            self.create_dm_channel(user_id1, user_id2, channel_name).await
        }
    }

    pub async fn get_dm_channels_for_user(&self, user_id: i32) -> Result<Vec<Channel>, sqlx::Error> {
        self.channel_repository.find_direct_message_channels(user_id).await
    }

    pub async fn delete_dm_channel(&self, channel_id: i32) -> Result<bool, sqlx::Error> {
        // Start a transaction
        let mut tx = self.pool.begin().await?;

        // Delete all DM members
        sqlx::query!(
            r#"
            DELETE FROM direct_message_members
            WHERE channel_id = $1
            "#,
            channel_id
        )
        .execute(&mut *tx)
        .await?;

        // Delete all messages in the channel
        sqlx::query!(
            r#"
            DELETE FROM messages
            WHERE channel_id = $1
            "#,
            channel_id
        )
        .execute(&mut *tx)
        .await?;

        // Delete the channel
        let result = sqlx::query!(
            r#"
            DELETE FROM channels
            WHERE channel_id = $1 AND type = 'dm'
            "#,
            channel_id
        )
        .execute(&mut *tx)
        .await?;

        // Commit the transaction
        tx.commit().await?;

        Ok(result.rows_affected() > 0)
    }
}
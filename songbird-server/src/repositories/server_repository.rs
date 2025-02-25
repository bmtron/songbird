use sqlx::{Pool, Postgres};
use chrono::{DateTime, Utc};
use crate::models::models::{Server, NewServer, ServerWithMembersResponse, UserResponse};
use crate::repositories::UserRepository;

pub struct ServerRepository {
    pool: Pool<Postgres>,
    user_repository: UserRepository,
}

impl ServerRepository {
    pub fn new(pool: Pool<Postgres>, user_repository: UserRepository) -> Self {
        Self { pool, user_repository }
    }

    pub async fn create(&self, new_server: NewServer) -> Result<Server, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            INSERT INTO servers (server_name, owner_user_id, icon_url)
            VALUES ($1, $2, $3)
            RETURNING server_id, server_name, owner_user_id, icon_url, created_at, updated_at
            "#,
            new_server.server_name,
            new_server.owner_user_id,
            new_server.icon_url
        )
        .fetch_one(&self.pool)
        .await?;

        let server = Server {
            server_id: record.server_id,
            server_name: record.server_name,
            owner_user_id: record.owner_user_id,
            icon_url: record.icon_url,
            created_at: DateTime::from_naive_utc_and_offset(record.created_at, Utc),
            updated_at: record.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
        };

        Ok(server)
    }

    pub async fn find_by_id(&self, server_id: i32) -> Result<Option<Server>, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            SELECT server_id, server_name, owner_user_id, icon_url, created_at, updated_at
            FROM servers
            WHERE server_id = $1
            "#,
            server_id
        )
        .fetch_optional(&self.pool)
        .await?;

        let server = record.map(|r| Server {
            server_id: r.server_id,
            server_name: r.server_name,
            owner_user_id: r.owner_user_id,
            icon_url: r.icon_url,
            created_at: DateTime::from_naive_utc_and_offset(r.created_at, Utc),
            updated_at: r.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
        });

        Ok(server)
    }

    pub async fn find_by_owner(&self, owner_user_id: i32) -> Result<Vec<Server>, sqlx::Error> {
        let records = sqlx::query!(
            r#"
            SELECT server_id, server_name, owner_user_id, icon_url, created_at, updated_at
            FROM servers
            WHERE owner_user_id = $1
            "#,
            owner_user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let servers = records
            .into_iter()
            .map(|r| Server {
                server_id: r.server_id,
                server_name: r.server_name,
                owner_user_id: r.owner_user_id,
                icon_url: r.icon_url,
                created_at: DateTime::from_naive_utc_and_offset(r.created_at, Utc),
                updated_at: r.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
            })
            .collect();

        Ok(servers)
    }

    pub async fn find_all(&self) -> Result<Vec<Server>, sqlx::Error> {
        let records = sqlx::query!(
            r#"
            SELECT server_id, server_name, owner_user_id, icon_url, created_at, updated_at
            FROM servers
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let servers = records
            .into_iter()
            .map(|r| Server {
                server_id: r.server_id,
                server_name: r.server_name,
                owner_user_id: r.owner_user_id,
                icon_url: r.icon_url,
                created_at: DateTime::from_naive_utc_and_offset(r.created_at, Utc),
                updated_at: r.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
            })
            .collect();

        Ok(servers)
    }

    pub async fn find_servers_for_user(&self, user_id: i32) -> Result<Vec<Server>, sqlx::Error> {
        let records = sqlx::query!(
            r#"
            SELECT s.server_id, s.server_name, s.owner_user_id, s.icon_url, s.created_at, s.updated_at
            FROM servers s
            JOIN server_members sm ON s.server_id = sm.server_id
            WHERE sm.user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let servers = records
            .into_iter()
            .map(|r| Server {
                server_id: r.server_id,
                server_name: r.server_name,
                owner_user_id: r.owner_user_id,
                icon_url: r.icon_url,
                created_at: DateTime::from_naive_utc_and_offset(r.created_at, Utc),
                updated_at: r.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
            })
            .collect();

        Ok(servers)
    }

    pub async fn update(&self, server_id: i32, server: Server) -> Result<Server, sqlx::Error> {
        let now = Utc::now();
        let record = sqlx::query!(
            r#"
            UPDATE servers
            SET server_name = $1, owner_user_id = $2, icon_url = $3, updated_at = $4
            WHERE server_id = $5
            RETURNING server_id, server_name, owner_user_id, icon_url, created_at, updated_at
            "#,
            server.server_name,
            server.owner_user_id,
            server.icon_url,
            now as _,
            server_id
        )
        .fetch_one(&self.pool)
        .await?;

        let updated_server = Server {
            server_id: record.server_id,
            server_name: record.server_name,
            owner_user_id: record.owner_user_id,
            icon_url: record.icon_url,
            created_at: DateTime::from_naive_utc_and_offset(record.created_at, Utc),
            updated_at: record.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
        };

        Ok(updated_server)
    }

    pub async fn delete(&self, server_id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM servers
            WHERE server_id = $1
            "#,
            server_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_server_members(&self, server_id: i32) -> Result<Vec<UserResponse>, sqlx::Error> {
        let user_records = sqlx::query!(
            r#"
            SELECT u.user_id, u.username, u.email, u.avatar_url, u.created_at, u.status
            FROM users u
            JOIN server_members sm ON u.user_id = sm.user_id
            WHERE sm.server_id = $1
            "#,
            server_id
        )
        .fetch_all(&self.pool)
        .await?;

        let members = user_records
            .into_iter()
            .map(|record| UserResponse {
                user_id: record.user_id,
                username: record.username,
                email: record.email,
                avatar_url: record.avatar_url,
                status: record.status,
                created_at: DateTime::from_naive_utc_and_offset(record.created_at, Utc),
            })
            .collect();

        Ok(members)
    }

    pub async fn get_server_with_members(&self, server_id: i32) -> Result<Option<ServerWithMembersResponse>, sqlx::Error> {
        let server = self.find_by_id(server_id).await?;
        
        if let Some(server) = server {
            let members = self.get_server_members(server_id).await?;
            Ok(Some(ServerWithMembersResponse {
                server,
                members,
            }))
        } else {
            Ok(None)
        }
    }
}
use sqlx::{Pool, Postgres};
use chrono::{DateTime, Utc};
use crate::models::models::{User, NewUser, UserResponse};

#[derive(Clone)]
pub struct UserRepository {
    pool: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, new_user: NewUser) -> Result<User, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            INSERT INTO users (username, email, password_hash, avatar_url, status)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING user_id, username, email, password_hash, avatar_url, created_at, updated_at, status
            "#,
            new_user.username,
            new_user.email,
            new_user.password_hash,
            new_user.avatar_url,
            new_user.status
        )
        .fetch_one(&self.pool)
        .await?;

        let user = User {
            user_id: record.user_id,
            username: record.username,
            email: record.email,
            password_hash: record.password_hash,
            avatar_url: record.avatar_url,
            created_at: DateTime::from_naive_utc_and_offset(record.created_at, Utc),
            updated_at: record.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            status: record.status
        };
        Ok(user)
    }

    pub async fn find_by_id(&self, user_id: i32) -> Result<Option<User>, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            SELECT user_id, username, email, password_hash, avatar_url, created_at, updated_at, status
            FROM users
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record.map(|r| User {
            user_id: r.user_id,
            username: r.username,
            email: r.email,
            password_hash: r.password_hash,
            avatar_url: r.avatar_url,
            created_at: DateTime::from_naive_utc_and_offset(r.created_at, Utc),
            updated_at: r.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            status: r.status
        }))
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        let record = sqlx::query!(

            r#"
            SELECT user_id, username, email, password_hash, avatar_url, created_at, updated_at, status
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record.map(|r| User {
            user_id: r.user_id,
            username: r.username,
            email: r.email,
            password_hash: r.password_hash,
            avatar_url: r.avatar_url,
            created_at: DateTime::from_naive_utc_and_offset(r.created_at, Utc),
            updated_at: r.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            status: r.status
        }))
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            SELECT user_id, username, email, password_hash, avatar_url, created_at, updated_at, status
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record.map(|r| User {
            user_id: r.user_id,
            username: r.username,
            email: r.email,
            password_hash: r.password_hash,
            avatar_url: r.avatar_url,
            created_at: DateTime::from_naive_utc_and_offset(r.created_at, Utc),
            updated_at: r.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            status: r.status
        }))
    }

    pub async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let records = sqlx::query!(
            r#"
            SELECT user_id, username, email, password_hash, avatar_url, created_at, updated_at, status
            FROM users
            ORDER BY username
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut vec_users: Vec<User> = Vec::new();
        for record in records.iter() {
            let user = User {
                user_id: record.user_id,
                username: record.username.clone(),
                email: record.email.clone(),
                password_hash: record.password_hash.clone(),
                avatar_url: record.avatar_url.clone(),
                created_at: DateTime::from_naive_utc_and_offset(record.created_at, Utc),
                updated_at: record.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
                status: record.status.clone()
            };
            vec_users.push(user);
        }

        Ok(vec_users)
    }

    pub async fn update(&self, user_id: i32, user: User) -> Result<User, sqlx::Error> {
        let now = Utc::now();
        let record = sqlx::query!(
            r#"
            UPDATE users
            SET username = $1, email = $2, password_hash = $3, avatar_url = $4, updated_at = $5, status = $6
            WHERE user_id = $7
            RETURNING user_id, username, email, password_hash, avatar_url, created_at, updated_at, status
            "#,
            user.username,
            user.email,
            user.password_hash,
            user.avatar_url,
            now as _,
            user.status,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

    let updated_user = User {
        user_id: record.user_id,
        username: record.username,
        email: record.email,
        password_hash: record.password_hash,
        avatar_url: record.avatar_url,
        created_at: DateTime::from_naive_utc_and_offset(record.created_at, Utc),
        updated_at: record.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
        status: record.status
    };
        Ok(updated_user)
    }

    pub async fn delete(&self, user_id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn to_response(&self, user: User) -> UserResponse {
        UserResponse {
            user_id: user.user_id,
            username: user.username,
            email: user.email,
            avatar_url: user.avatar_url,
            status: user.status,
            created_at: user.created_at,
        }
    }
}

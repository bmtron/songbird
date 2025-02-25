use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub avatar_url: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub server_id: i32,
    pub server_name: String,
    pub owner_user_id: i32,
    pub icon_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewServer {
    pub server_name: String,
    pub owner_user_id: i32,
    pub icon_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerMember {
    pub server_id: i32,
    pub user_id: i32,
    pub nickname: Option<String>,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewServerMember {
    pub server_id: i32,
    pub user_id: i32,
    pub nickname: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub channel_id: i32,
    pub server_id: Option<i32>,
    pub name: String,
    pub channel_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewChannel {
    pub server_id: Option<i32>,
    pub name: String,
    pub channel_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message_id: i32,
    pub channel_id: i32,
    pub author_user_id: i32,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub edited_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewMessage {
    pub channel_id: i32,
    pub author_user_id: i32,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectMessageMember {
    pub channel_id: i32,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewDirectMessageMember {
    pub channel_id: i32,
    pub user_id: i32,
}

// Response structures (useful for API responses)

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerWithMembersResponse {
    pub server: Server,
    pub members: Vec<UserResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelWithMessagesResponse {
    pub channel: Channel,
    pub messages: Vec<MessageWithAuthorResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageWithAuthorResponse {
    pub message_id: i32,
    pub content: String,
    pub author: UserResponse,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
}
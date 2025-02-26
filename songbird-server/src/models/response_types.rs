use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::{channel::Channel, server::Server, user::User};

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

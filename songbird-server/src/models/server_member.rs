use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

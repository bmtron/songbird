use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

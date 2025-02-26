use serde::{Deserialize, Serialize};

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

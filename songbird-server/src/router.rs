// src/router.rs
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::handlers::{
    user_handlers::{create_user, get_user, update_user, delete_user, get_all_users, AppState},
    // Import other handlers here as you implement them
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        // User routes
        .route("/api/users", post(create_user))
        .route("/api/users", get(get_all_users))
        .route("/api/users/{user_id}", get(get_user))
        .route("/api/users/{user_id}", put(update_user))
        .route("/api/users/{user_id}", delete(delete_user))
        
        // Server routes
        // .route("/api/servers", post(create_server))
        // .route("/api/servers", get(get_all_servers))
        // .route("/api/servers/:server_id", get(get_server))
        // .route("/api/servers/:server_id", put(update_server))
        // .route("/api/servers/:server_id", delete(delete_server))
        // .route("/api/servers/:server_id/members", get(get_server_members))
        // .route("/api/servers/:server_id/members", post(add_server_member))
        // .route("/api/servers/:server_id/members/:user_id", delete(remove_server_member))
        
        // Channel routes
        // .route("/api/channels", post(create_channel))
        // .route("/api/channels/:channel_id", get(get_channel))
        // .route("/api/channels/:channel_id", put(update_channel))
        // .route("/api/channels/:channel_id", delete(delete_channel))
        // .route("/api/servers/:server_id/channels", get(get_server_channels))
        
        // Message routes
        // .route("/api/channels/:channel_id/messages", post(create_message))
        // .route("/api/channels/:channel_id/messages", get(get_channel_messages))
        // .route("/api/messages/:message_id", put(update_message))
        // .route("/api/messages/:message_id", delete(delete_message))
        
        // Direct message routes
        // .route("/api/dm", post(create_dm_channel))
        // .route("/api/users/:user_id/dm", get(get_user_dm_channels))
        
        .with_state(app_state)
}
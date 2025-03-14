// src/router.rs
use crate::handlers::{
    server_handlers::{
        create_server, delete_server, get_all_servers, get_server, get_servers_by_owner,
        update_server,
    },
    user_handlers::{create_user, login_attempt, delete_user, get_all_users, get_user, get_user_by_username, update_user},
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};

use sqlx::{Pool, Postgres};

// Create a common AppState that combines both repositories
#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub user_repository: crate::repositories::UserRepository,
    pub server_repository: crate::repositories::ServerRepository,
}

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        // Login Route
        .route("/api/login", post(login_attempt))
        // User routes
        .route("/api/users/create", post(create_user))
        .route("/api/users", get(get_all_users))
        .route("/api/users/{user_id}", get(get_user))
        .route("/api/users/by_username/{username}", get(get_user_by_username))
        .route("/api/users/{user_id}", put(update_user))
        .route("/api/users/{user_id}", delete(delete_user))
        // Server routes
        .route("/api/servers", post(create_server))
        .route("/api/servers", get(get_all_servers))
        .route("/api/servers/{server_id}", get(get_server))
        .route("/api/servers/{server_id}", put(update_server))
        .route("/api/servers/{server_id}", delete(delete_server))
        .route("/api/servers/owner/{owner_user_id}", get(get_servers_by_owner))
        // Commented out routes for server members until they are implemented
        // .route("/api/servers/:server_id/members", get(get_server_members))
        // .route("/api/servers/:server_id/members", post(add_server_member))
        // .route(
        //     "/api/servers/:server_id/members/:user_id",
        //     delete(remove_server_member),
        // )
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

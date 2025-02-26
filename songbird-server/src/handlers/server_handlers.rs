// src/handlers/server_handlers.rs
use crate::models::models::{NewServer, Server, ServerWithMembersResponse};
use crate::router::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateServerRequest {
    pub name: String,
    pub description: String,
    pub owner_user_id: i32,
    pub icon_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateServerRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub owner_user_id: Option<i32>,
    pub icon_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

pub async fn create_server(
    State(state): State<AppState>,
    Json(payload): Json<CreateServerRequest>,
) -> impl IntoResponse {
    let new_server = NewServer {
        server_name: payload.name,
        owner_user_id: payload.owner_user_id,
        icon_url: payload.icon_url,
    };

    match state.server_repository.create(new_server).await {
        Ok(server) => (
            StatusCode::CREATED,
            Json(ApiResponse {
                success: true,
                data: Some(server),
                error: None,
            }),
        ),
        Err(e) => {
            let error_message = if e.to_string().contains("duplicate key") {
                "Server name already exists"
            } else {
                "Failed to create server"
            };

            (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    success: false,
                    data: None::<Server>,
                    error: Some(error_message.to_string()),
                }),
            )
        }
    }
}

pub async fn get_server(
    State(state): State<AppState>,
    Path(server_id): Path<i32>,
) -> impl IntoResponse {
    match state.server_repository.find_by_id(server_id).await {
        Ok(Some(server)) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(server),
                error: None,
            }),
        ),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                data: None::<Server>,
                error: Some("Server not found".to_string()),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                data: None::<Server>,
                error: Some("Failed to fetch server".to_string()),
            }),
        ),
    }
}

pub async fn get_server_with_members(
    State(state): State<AppState>,
    Path(server_id): Path<i32>,
) -> impl IntoResponse {
    match state
        .server_repository
        .get_server_with_members(server_id)
        .await
    {
        Ok(Some(server_with_members)) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(server_with_members),
                error: None,
            }),
        ),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                data: None::<ServerWithMembersResponse>,
                error: Some("Server not found".to_string()),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                data: None::<ServerWithMembersResponse>,
                error: Some("Failed to fetch server with members".to_string()),
            }),
        ),
    }
}

pub async fn update_server(
    State(state): State<AppState>,
    Path(server_id): Path<i32>,
    Json(payload): Json<UpdateServerRequest>,
) -> impl IntoResponse {
    // First, get the current server
    let current_server = match state.server_repository.find_by_id(server_id).await {
        Ok(Some(server)) => server,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(ApiResponse {
                    success: false,
                    data: None::<Server>,
                    error: Some("Server not found".to_string()),
                }),
            )
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None::<Server>,
                    error: Some("Failed to fetch server".to_string()),
                }),
            )
        }
    };

    // Update the server fields
    let mut updated_server = current_server;

    if let Some(name) = payload.name {
        updated_server.server_name = name;
    }

    if let Some(owner_user_id) = payload.owner_user_id {
        updated_server.owner_user_id = owner_user_id;
    }

    if let Some(icon_url) = payload.icon_url {
        updated_server.icon_url = Some(icon_url);
    }

    // Save the updated server
    match state
        .server_repository
        .update(server_id, updated_server)
        .await
    {
        Ok(server) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(server),
                error: None,
            }),
        ),
        Err(e) => {
            let error_message = if e.to_string().contains("duplicate key") {
                "Server name already exists"
            } else {
                "Failed to update server"
            };

            (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    success: false,
                    data: None::<Server>,
                    error: Some(error_message.to_string()),
                }),
            )
        }
    }
}

pub async fn delete_server(
    State(state): State<AppState>,
    Path(server_id): Path<i32>,
) -> impl IntoResponse {
    match state.server_repository.delete(server_id).await {
        Ok(true) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some("Server deleted successfully".to_string()),
                error: None,
            }),
        ),
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                data: None::<String>,
                error: Some("Server not found".to_string()),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                data: None::<String>,
                error: Some("Failed to delete server".to_string()),
            }),
        ),
    }
}

pub async fn get_all_servers(State(state): State<AppState>) -> impl IntoResponse {
    match state.server_repository.find_all().await {
        Ok(servers) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(servers),
                error: None,
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                data: None::<Vec<Server>>,
                error: Some("Failed to fetch servers".to_string()),
            }),
        ),
    }
}

pub async fn get_servers_by_owner(
    State(state): State<AppState>,
    Path(owner_user_id): Path<i32>,
) -> impl IntoResponse {
    match state.server_repository.find_by_owner(owner_user_id).await {
        Ok(servers) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(servers),
                error: None,
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                data: None::<Vec<Server>>,
                error: Some("Failed to fetch servers for owner".to_string()),
            }),
        ),
    }
}

pub async fn get_servers_for_user(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> impl IntoResponse {
    match state.server_repository.find_servers_for_user(user_id).await {
        Ok(servers) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(servers),
                error: None,
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                data: None::<Vec<Server>>,
                error: Some("Failed to fetch servers for user".to_string()),
            }),
        ),
    }
}

// src/handlers/user_handlers.rs
use crate::models::{
    response_types::UserResponse,
    user::{NewUser, User},
};
use crate::repositories::UserRepository;
use crate::router::AppState;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub avatar_url: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    // Hash the password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(payload.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None::<UserResponse>,
                    error: Some("Failed to hash password".to_string()),
                }),
            )
        }
    };

    let new_user = NewUser {
        username: payload.username,
        email: payload.email,
        password_hash,
        avatar_url: payload.avatar_url,
        status: "online".to_string(),
    };

    match state.user_repository.create(new_user).await {
        Ok(user) => {
            let user_response = UserResponse {
                user_id: user.user_id,
                username: user.username,
                email: user.email,
                avatar_url: user.avatar_url,
                status: user.status,
                created_at: user.created_at,
            };

            (
                StatusCode::CREATED,
                Json(ApiResponse {
                    success: true,
                    data: Some(user_response),
                    error: None,
                }),
            )
        }
        Err(e) => {
            let error_message = if e.to_string().contains("duplicate key") {
                if e.to_string().contains("username") {
                    "Username already taken"
                } else if e.to_string().contains("email") {
                    "Email already registered"
                } else {
                    "Duplicate key violation"
                }
            } else {
                "Failed to create user"
            };

            (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    success: false,
                    data: None::<UserResponse>,
                    error: Some(error_message.to_string()),
                }),
            )
        }
    }
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> impl IntoResponse {
    match state.user_repository.find_by_id(user_id).await {
        Ok(Some(user)) => {
            let user_response = UserResponse {
                user_id: user.user_id,
                username: user.username,
                email: user.email,
                avatar_url: user.avatar_url,
                status: user.status,
                created_at: user.created_at,
            };

            (
                StatusCode::OK,
                Json(ApiResponse {
                    success: true,
                    data: Some(user_response),
                    error: None,
                }),
            )
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                data: None::<UserResponse>,
                error: Some("User not found".to_string()),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                data: None::<UserResponse>,
                error: Some("Failed to fetch user".to_string()),
            }),
        ),
    }
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> impl IntoResponse {
    // First, get the current user
    let current_user = match state.user_repository.find_by_id(user_id).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(ApiResponse {
                    success: false,
                    data: None::<UserResponse>,
                    error: Some("User not found".to_string()),
                }),
            )
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None::<UserResponse>,
                    error: Some("Failed to fetch user".to_string()),
                }),
            )
        }
    };

    // Update the user fields
    let mut updated_user = current_user;

    if let Some(username) = payload.username {
        updated_user.username = username;
    }

    if let Some(email) = payload.email {
        updated_user.email = email;
    }

    if let Some(password) = payload.password {
        // Hash the new password
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        updated_user.password_hash = match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(hash) => hash.to_string(),
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        success: false,
                        data: None::<UserResponse>,
                        error: Some("Failed to hash password".to_string()),
                    }),
                )
            }
        };
    }

    if let Some(avatar_url) = payload.avatar_url {
        updated_user.avatar_url = Some(avatar_url);
    }

    if let Some(status) = payload.status {
        updated_user.status = status;
    }

    // Save the updated user
    match state.user_repository.update(user_id, updated_user).await {
        Ok(user) => {
            let user_response = UserResponse {
                user_id: user.user_id,
                username: user.username,
                email: user.email,
                avatar_url: user.avatar_url,
                status: user.status,
                created_at: user.created_at,
            };

            (
                StatusCode::OK,
                Json(ApiResponse {
                    success: true,
                    data: Some(user_response),
                    error: None,
                }),
            )
        }
        Err(e) => {
            let error_message = if e.to_string().contains("duplicate key") {
                if e.to_string().contains("username") {
                    "Username already taken"
                } else if e.to_string().contains("email") {
                    "Email already registered"
                } else {
                    "Duplicate key violation"
                }
            } else {
                "Failed to update user"
            };

            (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    success: false,
                    data: None::<UserResponse>,
                    error: Some(error_message.to_string()),
                }),
            )
        }
    }
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> impl IntoResponse {
    match state.user_repository.delete(user_id).await {
        Ok(true) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some("User deleted successfully".to_string()),
                error: None,
            }),
        ),
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                data: None::<String>,
                error: Some("User not found".to_string()),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                data: None::<String>,
                error: Some("Failed to delete user".to_string()),
            }),
        ),
    }
}

pub async fn get_all_users(State(state): State<AppState>) -> impl IntoResponse {
    tracing::info!("getting users...");
    match state.user_repository.find_all().await {
        Ok(users) => {
            let user_responses: Vec<UserResponse> = users
                .into_iter()
                .map(|user| UserResponse {
                    user_id: user.user_id,
                    username: user.username,
                    email: user.email,
                    avatar_url: user.avatar_url,
                    status: user.status,
                    created_at: user.created_at,
                })
                .collect();

            (
                StatusCode::OK,
                Json(ApiResponse {
                    success: true,
                    data: Some(user_responses),
                    error: None,
                }),
            )
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                data: None::<Vec<UserResponse>>,
                error: Some("Failed to fetch users".to_string()),
            }),
        ),
    }
}

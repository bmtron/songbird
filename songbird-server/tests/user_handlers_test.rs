use serde_json;
use songbird_server::handlers::user_handlers::{CreateUserRequest, UpdateUserRequest};

#[test]
fn test_create_user_request_serialization() {
    let request = CreateUserRequest {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        avatar_url: Some("https://example.com/avatar.jpg".to_string()),
    };

    let json = serde_json::to_string(&request).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed["username"], "testuser");
    assert_eq!(parsed["email"], "test@example.com");
    assert_eq!(parsed["password"], "password123");
    assert_eq!(parsed["avatar_url"], "https://example.com/avatar.jpg");
}

#[test]
fn test_update_user_request_serialization() {
    let request = UpdateUserRequest {
        username: Some("updateduser".to_string()),
        email: Some("updated@example.com".to_string()),
        password: Some("newpassword".to_string()),
        avatar_url: Some("https://example.com/new-avatar.jpg".to_string()),
        status: Some("away".to_string()),
    };

    let json = serde_json::to_string(&request).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed["username"], "updateduser");
    assert_eq!(parsed["email"], "updated@example.com");
    assert_eq!(parsed["password"], "newpassword");
    assert_eq!(parsed["avatar_url"], "https://example.com/new-avatar.jpg");
    assert_eq!(parsed["status"], "away");
}

#[test]
fn test_partial_update_user_request_serialization() {
    let request = UpdateUserRequest {
        username: Some("updateduser".to_string()),
        email: None,
        password: None,
        avatar_url: None,
        status: Some("away".to_string()),
    };

    let json = serde_json::to_string(&request).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed["username"], "updateduser");
    assert!(parsed["email"].is_null());
    assert!(parsed["password"].is_null());
    assert!(parsed["avatar_url"].is_null());
    assert_eq!(parsed["status"], "away");
}

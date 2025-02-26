use songbird_server::models::user::NewUser;

#[test]
fn test_new_user_creation() {
    let new_user = NewUser {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: "hashed_password".to_string(),
        avatar_url: Some("https://example.com/avatar.jpg".to_string()),
        status: "online".to_string(),
    };

    assert_eq!(new_user.username, "testuser");
    assert_eq!(new_user.email, "test@example.com");
    assert_eq!(new_user.password_hash, "hashed_password");
    assert_eq!(
        new_user.avatar_url,
        Some("https://example.com/avatar.jpg".to_string())
    );
    assert_eq!(new_user.status, "online");
}

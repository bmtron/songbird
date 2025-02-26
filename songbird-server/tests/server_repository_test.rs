use songbird_server::models::models::NewServer;

#[test]
fn test_new_server_creation() {
    let new_server = NewServer {
        server_name: "Test Server".to_string(),
        owner_user_id: 1,
        icon_url: Some("https://example.com/icon.jpg".to_string()),
    };

    assert_eq!(new_server.server_name, "Test Server");
    assert_eq!(new_server.owner_user_id, 1);
    assert_eq!(
        new_server.icon_url,
        Some("https://example.com/icon.jpg".to_string())
    );
}

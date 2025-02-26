use songbird_server::handlers::server_handlers::{CreateServerRequest, UpdateServerRequest};

#[test]
fn test_create_server_request_fields() {
    let request = CreateServerRequest {
        name: "Test Server".to_string(),
        description: "A test server".to_string(),
        owner_user_id: 1,
        icon_url: Some("https://example.com/icon.jpg".to_string()),
    };

    assert_eq!(request.name, "Test Server");
    assert_eq!(request.description, "A test server");
    assert_eq!(request.owner_user_id, 1);
    assert_eq!(
        request.icon_url,
        Some("https://example.com/icon.jpg".to_string())
    );
}

#[test]
fn test_update_server_request_fields() {
    let request = UpdateServerRequest {
        name: Some("Updated Server".to_string()),
        description: Some("An updated server".to_string()),
        owner_user_id: Some(2),
        icon_url: Some("https://example.com/new-icon.jpg".to_string()),
    };

    assert_eq!(request.name, Some("Updated Server".to_string()));
    assert_eq!(request.description, Some("An updated server".to_string()));
    assert_eq!(request.owner_user_id, Some(2));
    assert_eq!(
        request.icon_url,
        Some("https://example.com/new-icon.jpg".to_string())
    );
}

#[test]
fn test_partial_update_server_request_fields() {
    let request = UpdateServerRequest {
        name: Some("Updated Server".to_string()),
        description: None,
        owner_user_id: None,
        icon_url: Some("https://example.com/new-icon.jpg".to_string()),
    };

    assert_eq!(request.name, Some("Updated Server".to_string()));
    assert_eq!(request.description, None);
    assert_eq!(request.owner_user_id, None);
    assert_eq!(
        request.icon_url,
        Some("https://example.com/new-icon.jpg".to_string())
    );
}

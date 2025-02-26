use songbird_server::handlers::user_handlers::ApiResponse;

#[test]
fn test_api_response_success() {
    let response: ApiResponse<String> = ApiResponse {
        success: true,
        data: Some("Test data".to_string()),
        error: None,
    };

    assert_eq!(response.success, true);
    assert_eq!(response.data, Some("Test data".to_string()));
    assert_eq!(response.error, None);
}

#[test]
fn test_api_response_error() {
    let response: ApiResponse<String> = ApiResponse {
        success: false,
        data: None,
        error: Some("Test error".to_string()),
    };

    assert_eq!(response.success, false);
    assert_eq!(response.data, None);
    assert_eq!(response.error, Some("Test error".to_string()));
}

#[test]
fn test_api_response_with_complex_data() {
    #[derive(Debug, PartialEq, Clone)]
    struct TestData {
        id: i32,
        name: String,
    }

    let test_data = TestData {
        id: 1,
        name: "Test".to_string(),
    };

    let response: ApiResponse<TestData> = ApiResponse {
        success: true,
        data: Some(test_data.clone()),
        error: None,
    };

    assert_eq!(response.success, true);
    assert_eq!(response.data, Some(test_data));
    assert_eq!(response.error, None);
}

# Songbird Server Tests

This directory contains unit tests for the Songbird server application.

## Test Files

-   `api_response_test.rs`: Tests for the API response structure
-   `server_handlers_test.rs`: Tests for the server handlers
-   `server_repository_test.rs`: Tests for the server repository
-   `user_handlers_test.rs`: Tests for the user handlers
-   `user_repository_test.rs`: Tests for the user repository

## Running Tests

To run the tests, use the following command:

```bash
cargo test
```

## Test Structure

The tests are organized by module and functionality:

1. **API Response Tests**: Verify that the API response structure works correctly
2. **Handler Tests**: Test the request and response structures for handlers
3. **Repository Tests**: Test the repository models and methods

## Adding New Tests

When adding new tests, follow these guidelines:

1. Create a new test file for each module
2. Use descriptive test names
3. Follow the Arrange-Act-Assert pattern
4. Mock external dependencies when necessary

## Integration Tests

For integration tests that require a database connection, you'll need to set up a test database.
These tests are not included in this directory and should be run separately.

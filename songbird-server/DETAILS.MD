Backend (Rust):

Actix or Axum for the web framework
SQLx for database operations (works well with PostgreSQL)
Tokio for async runtime
Tungstenite for WebSocket handling

Frontend (Tauri):

Choice of web framework (React, Svelte, Vue)
WebSocket client for real-time communication
SQLite for local caching if needed
Built-in IPC for any needed system-level operations

// src/main.rs
mod models;
mod repositories;
mod handlers;
mod database;
mod router;

use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::{
    database::establish_connection,
    repositories::UserRepository,
    handlers::user_handlers::AppState,
    router::create_router,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();
    
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create a connection pool
    let pool = establish_connection().await?;
    
    // Initialize repositories
    let user_repository = UserRepository::new(pool.clone());
    
    // Create app state
    let app_state = AppState {
        pool: pool.clone(),
        user_repository,
    };
    
    // Build the router
    let app = create_router(app_state);
    
    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("listening on {}", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
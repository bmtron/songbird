use core::fmt;
use axum::{
    routing::{get,post},
    http::StatusCode,
    Json, Router
};

use serde::{Deserialize, Serialize};
use password_hash::{PasswordHasher, SaltString, rand_core::OsRng};
use argon2::Argon2;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/auth/register", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<CreateUser>) {
    println!("{}", payload);
    let salt = SaltString::generate(OsRng);
    let argon2 = Argon2::default();
    let new_user = CreateUser {
        username: payload.username,
        email: payload.email,
        password: argon2.hash_password(payload.password.as_bytes(), &salt).unwrap().to_string()
    };
    (StatusCode::CREATED, Json(new_user))
}


#[derive(Deserialize, Serialize)]
struct CreateUser {
    username: String,
    email: String,
    password: String
}

impl fmt::Display for CreateUser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "username: {}, email: {}, password: {}", self.username, self.email, self.password)
    }
}
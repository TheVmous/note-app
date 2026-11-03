use axum::{
    Json, Router,
    extract::Path,
    routing::{delete, get, post},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub name: String,
    pub pass: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub secret: String,
}

#[axum::debug_handler]
pub async fn get_user_cred(Json(body): Json<LoginRequest>) -> Json<LoginResponse> {
    Json(LoginResponse {secret:format!("ur token is: {}", ) })
}

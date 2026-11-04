use std::env;

use axum::{
    Json, Router,
    extract::Path,
    routing::{delete, get, post},
};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Extras, Header, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    sub: String,
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
    let jwt_secret = env::var("JWT_SECRET").unwrap();
    let claim = Claim { sub: body.name };
    let token = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    );
    Json(LoginResponse {
        secret: token.unwrap(),
    })
}

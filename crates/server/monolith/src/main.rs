use axum::{
    Router,
    extract::Path,
    routing::{delete, get, post},
};
use tokio::sync::{Mutex, RwLock};
use uuid::{Timestamp, Uuid};

use crate::api::auth::get_user_cred;
mod api;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let mut app = Router::new()
        .route("/login",post(get_user_cred));
        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // println!("Hello, world!");

    //asmap that registers all nodes
    // post, get, put, delete to modify the hashmap
}

//usually json for api's

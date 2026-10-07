use axum::{
    Router,
    extract::Path,
    routing::{delete, get, post},
};
use std::{collections::HashMap, str::FromStr, sync::Arc};
use tokio::sync::{Mutex, RwLock};
use uuid::{Timestamp, Uuid};

#[tokio::main]
async fn main() {
    let notes: HashMap<Uuid, String> = HashMap::new();
    let notes = RwLock::new(notes);
    let notes = Arc::new(notes);
    let notes_cpy_get = notes.clone();
    let notes_cpy_delete = notes.clone();
    let notes_cpy_put = notes.clone();
    let mut app = Router::new()
        .route(
            "/",
            get(|Path(note_id): Path<String>| async move {
                let guard = notes_cpy_get.read().await;
                guard.get(&Uuid::from_str(&note_id).unwrap());
            }),
        )
        .route(
            "/",
            delete(|Path(note_id): Path<String>| async move {
                let mut guard = notes_cpy_delete.write().await;
                guard.remove(&Uuid::from_str(&note_id).unwrap());
            }),
        )
        .route(
            "/note/",
            post(|body: String| async move {
                let mut guard = notes.write().await;
                let id = Uuid::now_v7();
                guard.insert(id, body);
            }),
        )
        .route(
            "/note/",
            post(|body: String| async move {
                let mut guard = notes_cpy_put.write().await;
                let id = Uuid::now_v7();
                guard.insert(id, body);
            }),
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // println!("Hello, world!");

    //asmap that registers all nodes
    // post, get, put, delete to modify the hashmap
}

//usually json for api's

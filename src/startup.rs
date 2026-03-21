use std::sync::Arc;
use tokio::sync::RwLock;

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::memory::{Db, Store};

pub type Memory = Arc<RwLock<Db>>;

pub fn build_app(db: Memory) -> Router {
    Router::new()
        .route("/insert", post(insert_handler))
        .route("/get/{key}", get(get_handler))
        .with_state(db)
}

#[derive(Deserialize, Serialize)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
}

pub async fn get_handler(State(db): State<Memory>, Path(key): Path<String>) -> String {
    let memory = db.read().await;
    let res = memory.get(key);

    match res {
        Some(s) => s.to_string(),
        None => "".to_string(),
    }
}

pub async fn insert_handler(
    State(db): State<Memory>,
    Json(payload): Json<KeyValuePair>,
) -> StatusCode {
    let mut memory = db.write().await;
    memory.set(payload.key, payload.value);

    StatusCode::OK
}

pub async fn store(State(db): State<Memory>) -> Memory {
    db
}

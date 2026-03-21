use std::sync::Arc;
use tokio::sync::RwLock;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};

use crate::memory::{Db, Store, Value};

pub type Memory = Arc<RwLock<Db>>;

pub fn build_app() -> Router {
    Router::new()
        .route("/insert/{key}", post(insert_handler))
        .route("/get/{key}", get(get_handler))
        .with_state(store())
}

pub async fn get_handler(State(db): State<Memory>, Path(key): Path<String>) -> Vec<u8> {
    let memory = db.read().await;
    let res = memory.get(key);

    res
}

pub async fn insert_handler(
    State(db): State<Memory>,
    Path(key): Path<String>,
    Json(payload): Json<Value>,
) -> StatusCode {
    let mut memory = db.write().await;
    memory.set(key, payload);

    StatusCode::OK
}

pub fn store() -> Memory {
    Arc::new(RwLock::new(Db::new()))
}

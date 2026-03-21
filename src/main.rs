use rbrldb::memory::{Db, Store};
use rbrldb::startup::build_app;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let db = Arc::new(RwLock::new(Db::new()));
    let app = build_app(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    let _ = axum::serve(listener, app).await;

    Ok(())
}

use std::sync::Arc;
use tokio::sync::RwLock;

use axum_test::TestServer;
use rbrldb::{
    memory::{Db, Store},
    startup::{KeyValuePair, build_app},
};

#[tokio::test]
async fn write_test() {
    let db = Arc::new(RwLock::new(Db::new()));
    let app = build_app(db);
    let server = TestServer::new(app);

    let res = server
        .post("/insert")
        .json(&KeyValuePair {
            key: "foo".to_string(),
            value: "bar".to_string(),
        })
        .await;
    assert!(res.status_code().is_success())
}

#[tokio::test]
async fn read_test() {
    let db = Arc::new(RwLock::new(Db::new()));
    let app = build_app(db);
    let server = TestServer::new(app);

    let res = server
        .post("/insert")
        .json(&KeyValuePair {
            key: "foo".to_string(),
            value: "bar".to_string(),
        })
        .await;
    assert!(res.status_code().is_success());

    let read_result = server.get("/get/foo").await;

    assert_eq!(read_result.text(), "bar".to_string());
}

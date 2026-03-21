use axum_test::TestServer;
use rbrldb::{memory::Value, startup::build_app};

#[tokio::test]
async fn write_test() {
    let app = build_app();
    let server = TestServer::new(app);

    let res = server
        .post("/insert/foo")
        .json(&Value {
            data: "bar".as_bytes().to_owned(),
            ttl: None,
        })
        .await;
    assert!(res.status_code().is_success())
}

#[tokio::test]
async fn read_test() {
    let app = build_app();
    let server = TestServer::new(app);

    let res = server
        .post("/insert/foo")
        .json(&Value {
            data: "bar".as_bytes().to_owned(),
            ttl: None,
        })
        .await;
    assert!(res.status_code().is_success());

    let read_result = server.get("/get/foo").await;

    assert_eq!(read_result.text(), "bar".to_string());
}

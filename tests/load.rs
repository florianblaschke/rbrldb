use std::time::Instant;

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

#[tokio::test]
async fn multiple_writes_and_reads() {
    let app = build_app();
    let server = TestServer::new(app);
    let num_entries = 1_000_000;

    let start = Instant::now();
    println!("start insert");

    for i in 0..num_entries {
        let key = format!("key_{i}");
        let value = format!("value_{i}");
        let res = server
            .post(&format!("/insert/{key}"))
            .json(&Value {
                data: value.as_bytes().to_owned(),
                ttl: None,
            })
            .await;
        assert!(res.status_code().is_success());
    }

    let end_write = Instant::now() - start;
    println!("End write in: {:?}", end_write);

    for i in 0..num_entries {
        let key = format!("key_{i}");
        let expected = format!("value_{i}");
        let res = server.get(&format!("/get/{key}")).await;
        assert_eq!(res.text(), expected);
    }

    let end_read = Instant::now() - start;
    println!("End read in: {:?}", end_read);
}

#[tokio::test]
async fn overwrite_existing_key() {
    let app = build_app();
    let server = TestServer::new(app);

    server
        .post("/insert/mykey")
        .json(&Value {
            data: "first".as_bytes().to_owned(),
            ttl: None,
        })
        .await;

    server
        .post("/insert/mykey")
        .json(&Value {
            data: "second".as_bytes().to_owned(),
            ttl: None,
        })
        .await;

    let res = server.get("/get/mykey").await;
    assert_eq!(res.text(), "second");
}

#[tokio::test]
async fn get_nonexistent_key_returns_empty() {
    let app = build_app();
    let server = TestServer::new(app);

    let res = server.get("/get/doesnotexist").await;
    assert_eq!(res.text(), "");
}

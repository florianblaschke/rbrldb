use rbrldb::startup::build_app;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = build_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    let _ = axum::serve(listener, app).await;

    Ok(())
}

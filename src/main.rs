use rbrldb::{
    startup::start_db,
    tracing::{get_subscriber, init_subscriber},
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("kv_store".into(), "error".into(), std::io::stdout);
    init_subscriber(subscriber);

    let port = std::env::var("PORT").unwrap_or_else(|_| "7878".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("starting server on port: {}", &addr);

    start_db(listener).await;

    Ok(())
}

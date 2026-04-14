use rbrldb::{
    startup::start_db,
    tracing::{get_subscriber, init_subscriber},
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("kv_store".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let addr = format!("{}:{}", "127.0.0.1", "7878");
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("starting server on port: {}", &addr);

    start_db(listener).await;

    Ok(())
}

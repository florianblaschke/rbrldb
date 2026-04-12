use rbrldb::startup::start_db;

#[tokio::main]
async fn main() {
    start_db("127.0.0.0:7878").await;
}

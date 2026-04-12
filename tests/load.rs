use rbrldb::startup::start_db;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::Instant,
};

#[tokio::test]
async fn health_test() {
    let addr = start_db("127.0.0.1:0").await;

    let mut stream = TcpStream::connect(addr).await.unwrap();
    stream.write(b"!;\r\n").await.unwrap();

    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await.unwrap();
    let response = std::str::from_utf8(&buf[..n]).unwrap();

    assert_eq!(response, "ok");
}

#[tokio::test]
async fn insert_test() {
    let addr = start_db("127.0.0.1:0").await;

    // now act as a client
    let mut stream = TcpStream::connect(addr).await.unwrap();
    stream.write(b"!;\r\n").await.unwrap();
    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await.unwrap();
    let response = std::str::from_utf8(&buf[..n]).unwrap();

    assert_eq!(response, "ok");

    let start = Instant::now();
    for i in 0..1_000_000 {
        let s = format!("+;{}$3;{}\r\n", i, i);
        stream.write(s.as_bytes()).await.unwrap();
        let mut buf = vec![0u8; 1024];
        let _ = stream.read(&mut buf).await.unwrap();
    }
    let insert_finished = start.elapsed();
    println!("Insert took: {:?}", insert_finished);

    for i in 0..1_000_000 {
        let s = format!("?;{}\r\n", i);
        stream.write(s.as_bytes()).await.unwrap();
        let mut buf = vec![0u8; 1024];
        let n = stream.read(&mut buf).await.unwrap();
        let response = std::str::from_utf8(&buf[..n]).unwrap();
        assert_eq!(response, &i.to_string());
    }

    let elapsed = start.elapsed();
    println!("Took: {:?}", elapsed);
}

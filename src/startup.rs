use crate::{
    channel::{ChannelPayload, send_command_to_channel, spawn_channel},
    connect::Connection,
};
use anyhow::Result;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::Sender,
};

pub async fn start_db(addr: &str) -> std::net::SocketAddr {
    let listener = TcpListener::bind(addr).await.unwrap();
    let local_addr = listener.local_addr().unwrap();
    let tx = spawn_channel();

    tokio::spawn(async move {
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let sender = tx.clone();
            tokio::spawn(async move { handle_stream(stream, sender).await });
        }
    });

    local_addr
}

async fn handle_stream(stream: TcpStream, channel: Sender<ChannelPayload>) -> Result<()> {
    let mut connection = Connection::new(stream);

    while let Some(bytes) = connection.read().await.unwrap() {
        let value = send_command_to_channel(&channel, &bytes).await?;
        let _ = connection.write(&value).await;

        if connection.has_empty_buffer() {
            connection.flush().await?;
        }
    }

    Ok(())
}

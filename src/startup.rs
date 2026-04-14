use crate::{
    channel::{ChannelPayload, send_command_to_channel, spawn_channel},
    connect::Connection,
};
use anyhow::Result;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::Sender,
};

pub async fn start_db(listener: TcpListener) {
    let tx = spawn_channel();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let sender = tx.clone();
        tokio::spawn(async move { handle_stream(stream, sender).await });
    }
}

#[tracing::instrument(name = "handle_stream", skip(stream, channel))]
async fn handle_stream(stream: TcpStream, channel: Sender<ChannelPayload>) -> Result<()> {
    let mut connection = Connection::new(stream);

    while let Some(bytes) = connection.read().await.unwrap() {
        let value = send_command_to_channel(&channel, &bytes).await?;
        tracing::info!("writing to connection");
        let _ = connection.write(&value).await;

        tracing::info!("flushing buffer");
        connection.flush().await?;
    }

    Ok(())
}

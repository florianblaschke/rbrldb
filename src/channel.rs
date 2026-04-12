use std::str::from_utf8;

use anyhow::{Result, anyhow};
use bytes::BytesMut;
use tokio::sync::{
    mpsc::{self, Sender},
    oneshot,
};

use crate::{
    command::{Command, CommandType},
    memory::{Db, Store, Value},
};

pub struct ChannelPayload {
    command: Command,
    responder: oneshot::Sender<String>,
}

pub fn spawn_channel() -> Sender<ChannelPayload> {
    let mut db = Db::new();
    let (tx, mut rx) = mpsc::channel::<ChannelPayload>(32);

    tokio::spawn(async move {
        while let Some(ChannelPayload { command, responder }) = rx.recv().await {
            match command.kind {
                CommandType::Health => {
                    let _ = responder.send("ok".to_string()).unwrap();
                }
                CommandType::Insert => {
                    if let Some(payload) = command.payload {
                        db.set(
                            payload.key,
                            Value {
                                data: payload.value.into(),
                                ttl: None,
                            },
                        );
                        let _ = responder.send("ok".to_string());
                    }
                }
                CommandType::Get => {
                    if let Some(payload) = command.payload {
                        let value = db.get(payload.key);
                        let _ = responder.send(from_utf8(&value).unwrap().to_string());
                    }
                }
                CommandType::Delete => panic!("not implemented delete"),
            };
        }
    });

    tx
}

pub async fn send_command_to_channel(
    channel: &Sender<ChannelPayload>,
    bytes: &BytesMut,
) -> Result<String> {
    let (send, receive) = oneshot::channel::<String>();
    let command = match Command::new(bytes.to_owned()) {
        Ok(c) => c,
        Err(_) => return Err(anyhow!("unable to parse command")),
    };

    let _ = channel
        .send(ChannelPayload {
            command: command,
            responder: send,
        })
        .await;

    match receive.await {
        Ok(v) => Ok(v),
        Err(_) => Err(anyhow!("unable to send command")),
    }
}
